use std::fs;
use windows::{
    core::HSTRING,
    Win32::UI::WindowsAndMessaging::{
        SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SystemParametersInfoW,
    },
};

mod gui;

fn main() {
    let media_path = "media/wallpaper.jpg";

    match fs::canonicalize(&media_path) {
        Ok(absolute_path) => {
            let path_hstring = HSTRING::from(absolute_path.to_str().unwrap());

            unsafe {
                match SystemParametersInfoW(
                    SPI_SETDESKWALLPAPER,
                    0,
                    Some(path_hstring.as_ptr() as *mut _),
                    SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
                ) {
                    Ok(()) => println!("Wallpaper changed."),
                    Err(_) => println!("Failed to change wallpaper.")
                };
            }
        }
        Err(_) => {
            eprintln!("No media found.");
        }
    }

    // Supprime = pas de gui
    gui::run();
}
