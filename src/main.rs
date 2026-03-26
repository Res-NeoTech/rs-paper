mod gui;

use crate::gui::{support, window::WindowUI};

fn main() {
    support::simple_init("rs-paper", |_, ui| {
        WindowUI::build(ui);
    });
}