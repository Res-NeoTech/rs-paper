mod gui;
mod utils;
mod wallpaper;

use gui::RsPaperApp;
use std::{env, ffi::OsStr, path::PathBuf, process};

fn main() -> iced::Result {
    let mut args = env::args_os();
    let _program = args.next();

    if let Some(flag) = args.next() {
        if flag == OsStr::new("--video-wallpaper") {
            let Some(path) = args.next() else {
                eprintln!("Missing video path.");
                process::exit(2);
            };

            if let Err(error) = wallpaper::wallpaper::run_video_wallpaper(PathBuf::from(path)) {
                eprintln!("{error}");
                process::exit(1);
            }

            return Ok(());
        }
    }

    iced::run(RsPaperApp::update, RsPaperApp::view)
}
