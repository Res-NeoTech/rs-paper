use std::fs;
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SystemParametersInfoW,
};
use windows::core::HSTRING;

#[derive(Copy, Clone)]
pub enum WallpaperType {
    Image,
    Video,
}

pub struct Wallpaper<'a> {
    pub name: &'a str,
    pub w_type: WallpaperType,
    pub path: &'a str,
}

impl<'a> Wallpaper<'a> {
    pub fn apply(&self) {
        match self.w_type {
            WallpaperType::Image => Self::apply_image(&self),
            WallpaperType::Video => println!("Video is not yet supported."),
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
}
