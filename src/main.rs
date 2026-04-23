use std::path::PathBuf;

use crate::wallpaper::wallpaper::{Wallpaper, WallpaperType};

mod wallpaper;
mod utils;

fn main() {
    let wallpaper: Wallpaper = Wallpaper {
        name: "Example".to_string(),
        w_type: WallpaperType::Image,
        path: PathBuf::new().join("media/wallpaper.jpg")
    };

    wallpaper.apply();
}