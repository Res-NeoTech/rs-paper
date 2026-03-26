use std::fs;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};
use Window;
fn main() {
    let media_path = "media/wallpaper.jpg";

    match fs::canonicalize(&media_path) {
        Ok(absolute_path) => {
            
        }
        Err(_) => {
            eprintln!("No media found.");
        }
    }
    
    println!("Hello, world!");
    Window::build();
}