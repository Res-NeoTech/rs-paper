use crate::wallpaper::wallpaper::{Wallpaper, WallpaperType};
use iced::widget::{button, column, text, text_input, Column, Button};
use iced::Alignment;
use iced::Length;

pub struct RsPaperApp {
    wallpaper_name: String,
    wallpaper_path: String,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    WallpaperNameChanged(String),
    WallpaperPathChanged(String),
    ApplyImageWallpaper,
    ApplyVideoWallpaper,
    ApplyEpstein,
}

impl Default for RsPaperApp {
    fn default() -> Self {
        Self {
            wallpaper_name: String::new(),
            wallpaper_path: String::new(),
            status_message: String::from("Ready to set wallpaper"),
        }
    }
}

fn text_button(label: &str, on_press: Message) -> Button<'_, Message> {
    button(text(label).size(16))
        .on_press(on_press)
        .padding(8)
}

impl RsPaperApp {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::WallpaperNameChanged(name) => {
                self.wallpaper_name = name;
            }
            Message::WallpaperPathChanged(path) => {
                self.wallpaper_path = path;
            }
            Message::ApplyImageWallpaper => {
                self.apply_wallpaper(WallpaperType::Image);
            }
            Message::ApplyVideoWallpaper => {
                self.apply_wallpaper(WallpaperType::Video);
            }
            Message::ApplyEpstein => {
                self.status_message = String::from("Epstein didn't kill himself");
            }
        }
    }

    fn apply_wallpaper(&mut self, w_type: WallpaperType) {
        if self.wallpaper_name.is_empty() || self.wallpaper_path.is_empty() {
            self.status_message = String::from("Fill everything monkey !");
            return;
        }

        let wallpaper = Wallpaper {
            name: &self.wallpaper_name,
            w_type,
            path: &self.wallpaper_path,
        };

        wallpaper.apply();
        self.status_message = format!("Appled {} wallpaper: {}", 
            match w_type { 
                WallpaperType::Image => "image",
                WallpaperType::Video => "video",
            }, 
            self.wallpaper_name);
    }

    pub fn view(&self) -> Column<'_, Message> {
        column![
            text("RsPaper").size(50),
            text("Wallpaper Manager").size(20),
            
            text("Wallpaper Name:").size(18),
            text_input("Enter wallpaper name", &self.wallpaper_name)
                .on_input(Message::WallpaperNameChanged)
                .padding(10)
                .size(20)
                .width(300),
            
            text("Wallpaper Path:").size(18),
            text_input("Enter image/video path", &self.wallpaper_path)
                .on_input(Message::WallpaperPathChanged)
                .padding(10)
                .size(20)
                .width(300),
            
            text_button("Apply Image Wallpaper", Message::ApplyImageWallpaper),
            text_button("Apply Video Wallpaper", Message::ApplyVideoWallpaper),
            text_button("Epstein", Message::ApplyEpstein),
            
            text(&self.status_message).size(16),
        ]
        .padding(30)
        .spacing(15)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    }
}