use crate::wallpaper::wallpaper::{Wallpaper, WallpaperType};

mod wallpaper;
fn main() {
    let wallpaper: Wallpaper = Wallpaper {
        name: "Example",
        w_type: WallpaperType::Image,
        path: "media/wallpaper.jpg"
    };

    wallpaper.apply();
}
