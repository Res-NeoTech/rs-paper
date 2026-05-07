use std::{path::PathBuf, process::Child};

use crate::wallpaper::wallpaper::{Wallpaper, WallpaperType};
use iced::Alignment;
use iced::Length;
use iced::widget::{Button, Column, button, column, text, text_input};

pub struct RsPaperApp {
    wallpaper_name: String,
    wallpaper_path: String,
    status_message: String,
    video_wallpaper: Option<Child>,
}

#[derive(Debug, Clone)]
pub enum Message {
    WallpaperNameChanged(String),
    WallpaperPathChanged(String),
    ApplyImageWallpaper,
    ApplyVideoWallpaper,
}

impl Default for RsPaperApp {
    fn default() -> Self {
        Self {
            wallpaper_name: String::new(),
            wallpaper_path: String::new(),
            status_message: String::from("Ready to set wallpaper"),
            video_wallpaper: None,
        }
    }
}

impl Drop for RsPaperApp {
    fn drop(&mut self) {
        self.stop_video_wallpaper();
    }
}

fn text_button(label: &str, on_press: Message) -> Button<'_, Message> {
    button(text(label).size(16)).on_press(on_press).padding(8)
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
        }
    }

    fn apply_wallpaper(&mut self, w_type: WallpaperType) {
        if self.wallpaper_name.is_empty() || self.wallpaper_path.is_empty() {
            self.status_message = String::from("Fill in the wallpaper name and path.");
            return;
        }

        let wallpaper = Wallpaper {
            name: self.wallpaper_name.clone(),
            w_type,
            path: PathBuf::new().join(&self.wallpaper_path),
        };

        match wallpaper.apply() {
            Ok(video_process) => {
                match video_process {
                    Some(process) => {
                        self.stop_video_wallpaper();
                        self.video_wallpaper = Some(process);
                    }
                    None => {
                        self.stop_video_wallpaper();
                    }
                }

                self.status_message = format!(
                    "Applied {} wallpaper: {}",
                    match w_type {
                        WallpaperType::Image => "image",
                        WallpaperType::Video => "video",
                    },
                    self.wallpaper_name
                );
            }
            Err(error) => {
                self.status_message = error;
            }
        }
    }

    fn stop_video_wallpaper(&mut self) {
        let Some(mut process) = self.video_wallpaper.take() else {
            return;
        };

        match process.try_wait() {
            Ok(Some(_)) => {}
            Ok(None) => {
                let _ = process.kill();
                let _ = process.wait();
            }
            Err(_) => {
                let _ = process.kill();
                let _ = process.wait();
            }
        }
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
            text(&self.status_message).size(16),
        ]
        .padding(30)
        .spacing(15)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    }
}
