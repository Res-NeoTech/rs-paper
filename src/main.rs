mod gui;
mod wallpaper;

use gui::RsPaperApp;

fn main() -> iced::Result {
    iced::run(RsPaperApp::update, RsPaperApp::view)
}