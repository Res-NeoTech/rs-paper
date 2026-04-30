use std::fs;
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SetParent,
    SystemParametersInfoW,
};
use windows::core::HSTRING;

// Imports pour la vidéo (winit et vlc)
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::ffi::c_void;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
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
    pub fn apply(&self) {
        match self.w_type {
            WallpaperType::Image => Self::apply_image(&self),
            WallpaperType::Video => Self::apply_video(&self),
        }
    }

    fn apply_image(&self) {
        match fs::canonicalize(&self.path) {
            Ok(absolute_path) => {
                let path_hstring = HSTRING::from(absolute_path.to_str().unwrap());

                unsafe {
                    match SystemParametersInfoW(
                        SPI_SETDESKWALLPAPER,
                        0,
                        Some(path_hstring.as_ptr() as *mut _),
                        SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
                    ) {
                        Ok(()) => println!("Wallpaper changed to {}.", &self.name),
                        Err(e) => eprintln!("Failed to change wallpaper {}: {e}", &self.name),
                    };
                }
            }
            Err(_) => {
                eprintln!("No media found.");
            }
        }
    }

    fn apply_video(&self) {
        match fs::canonicalize(&self.path) {
            Ok(absolute_path) => {
                // 1. Préparer le chemin pour le navigateur web
                let mut clean_path = absolute_path.to_string_lossy().to_string();
                if clean_path.starts_with("\\\\?\\") {
                    clean_path = clean_path.replace("\\\\?\\", "");
                }
                // Remplacer les \ par des / pour l'URL
                clean_path = clean_path.replace("\\", "/");
                let video_url = format!("file:///{}", clean_path);

                println!("Lancement de la vidéo à l'URL : {}", video_url);

                // 2. Créer l'EventLoop et la fenêtre winit
                let event_loop = EventLoop::new().unwrap();
                event_loop.set_control_flow(ControlFlow::Wait);

                let window = WindowBuilder::new()
                    .with_decorations(false)
                    .with_maximized(true)
                    .with_visible(false)
                    .build(&event_loop)
                    .unwrap();

                // 3. Récupérer le HWND
                let raw_handle = window.window_handle().unwrap().as_raw();
                let hwnd_ptr = match raw_handle {
                    RawWindowHandle::Win32(handle) => handle.hwnd.get() as *mut c_void,
                    _ => panic!("Ce programme nécessite Windows."),
                };

                // 4. Attacher au bureau
                let worker_w_hwnd = worker_w::get_wallpaper_worker_window();
                if worker_w_hwnd.0 != 0 {
                    unsafe {
                        SetParent(HWND(hwnd_ptr as isize), worker_w_hwnd);
                    }
                    window.set_visible(true);
                } else {
                    eprintln!("Erreur : Impossible de trouver WorkerW.");
                    return;
                }

                // 5. La magie Wry : On injecte notre HTML
                let html = format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <body style="margin:0; overflow:hidden; background-color:black;">
                        <video src="{}" autoplay loop muted style="width:100vw; height:100vh; object-fit:cover;"></video>
                    </body>
                    </html>"#,
                    video_url
                );

                // On attache le navigateur à notre fenêtre
                let webview = WebViewBuilder::new()
                    .with_html(html)
                    .build(&window)
                    .unwrap();

                // 6. Boucle principale (Plus besoin de gérer le loop de la vidéo, le HTML s'en charge !)
                event_loop
                    .run(move |event, elwt| {
                        let _keep_alive = &webview;

                        if let winit::event::Event::WindowEvent {
                            event: winit::event::WindowEvent::CloseRequested,
                            ..
                        } = event
                        {
                            println!("Fermeture du fond d'écran.");
                            elwt.exit();
                        }
                    })
                    .unwrap();
            }
            Err(_) => {
                eprintln!("Aucun média trouvé au chemin : {:?}", self.path);
            }
        }
    }
}
