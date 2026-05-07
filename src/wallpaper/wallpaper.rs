use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command},
};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SetParent,
    SystemParametersInfoW,
};
use windows::core::HSTRING;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wry::WebViewBuilder;

use crate::utils::worker_w;

#[derive(Copy, Clone)]
pub enum WallpaperType {
    Image,
    Video,
}

pub struct Wallpaper {
    pub name: String,
    pub w_type: WallpaperType,
    pub path: PathBuf,
}

impl Wallpaper {
    pub fn apply(&self) -> Result<Option<Child>, String> {
        match self.w_type {
            WallpaperType::Image => {
                self.apply_image()?;
                Ok(None)
            }
            WallpaperType::Video => self.apply_video().map(Some),
        }
    }

    fn apply_image(&self) -> Result<(), String> {
        let absolute_path = canonical_media_path(&self.path)?;
        let path = absolute_path.to_string_lossy();
        let path_hstring = HSTRING::from(path.as_ref());

        unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(path_hstring.as_ptr() as *mut _),
                SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
            )
            .map_err(|error| format!("Failed to change wallpaper {}: {error}", self.name))?;
        }

        Ok(())
    }

    fn apply_video(&self) -> Result<Child, String> {
        let absolute_path = canonical_media_path(&self.path)?;
        let executable = env::current_exe()
            .map_err(|error| format!("Could not find current executable: {error}"))?;

        Command::new(executable)
            .arg("--video-wallpaper")
            .arg(&absolute_path)
            .spawn()
            .map_err(|error| format!("Failed to start video wallpaper process: {error}"))
    }
}

pub fn run_video_wallpaper(path: PathBuf) -> Result<(), String> {
    let absolute_path = canonical_media_path(&path)?;
    let video_url = file_url_from_path(&absolute_path);
    let worker_w_hwnd = worker_w::get_wallpaper_worker_window();

    if worker_w_hwnd.0 == 0 {
        return Err(String::from("Could not find the desktop WorkerW window."));
    }

    let event_loop = EventLoop::new()
        .map_err(|error| format!("Failed to create wallpaper event loop: {error}"))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let screen_size = event_loop
        .primary_monitor()
        .map(|monitor| monitor.size())
        .unwrap_or(PhysicalSize::new(1920, 1080));

    let window = WindowBuilder::new()
        .with_title("RsPaper Video Wallpaper")
        .with_decorations(false)
        .with_resizable(false)
        .with_visible(false)
        .with_inner_size(screen_size)
        .build(&event_loop)
        .map_err(|error| format!("Failed to create wallpaper window: {error}"))?;

    let window_hwnd = hwnd_from_window(&window)?;

    unsafe {
        SetParent(window_hwnd, worker_w_hwnd);
    }

    window.set_outer_position(PhysicalPosition::new(0, 0));
    let _ = window.request_inner_size(screen_size);
    window.set_visible(true);

    let html = build_video_html(&video_url);
    let webview = WebViewBuilder::new()
        .with_html(html)
        .build(&window)
        .map_err(|error| format!("Failed to create wallpaper webview: {error}"))?;

    event_loop
        .run(move |event, elwt| {
            let _keep_alive = &webview;

            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                elwt.exit();
            }
        })
        .map_err(|error| format!("Wallpaper event loop failed: {error}"))
}

fn canonical_media_path(path: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(path).map_err(|_| format!("No media found at {}", path.display()))
}

fn hwnd_from_window(window: &Window) -> Result<HWND, String> {
    let raw_handle = window
        .window_handle()
        .map_err(|error| format!("Failed to read wallpaper window handle: {error}"))?
        .as_raw();

    match raw_handle {
        RawWindowHandle::Win32(handle) => Ok(HWND(handle.hwnd.get())),
        _ => Err(String::from(
            "Video wallpapers are only supported on Windows.",
        )),
    }
}

fn build_video_html(video_url: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        html, body, video {{
            width: 100%;
            height: 100%;
            margin: 0;
            overflow: hidden;
            background: black;
        }}

        video {{
            object-fit: cover;
        }}
    </style>
</head>
<body>
    <video src="{video_url}" autoplay loop muted playsinline></video>
</body>
</html>"#
    )
}

fn file_url_from_path(path: &Path) -> String {
    let mut normalized = path.to_string_lossy().replace('\\', "/");

    if let Some(path) = normalized.strip_prefix("//?/UNC/") {
        normalized = format!("//{path}");
    } else if let Some(path) = normalized.strip_prefix("//?/") {
        normalized = path.to_string();
    }

    let encoded = percent_encode_url_path(&normalized);

    if encoded.starts_with("//") {
        format!("file:{encoded}")
    } else {
        format!("file:///{encoded}")
    }
}

fn percent_encode_url_path(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());

    for byte in path.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b':' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char)
            }
            byte => {
                let _ = write!(encoded, "%{byte:02X}");
            }
        }
    }

    encoded
}
