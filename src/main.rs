#![windows_subsystem = "windows"]

mod gui {
    pub(crate) mod gui_main;
    mod gui_logic;
    mod serialize;
}
mod config;
mod utils;

use crate::gui::gui_main::App;

use iced:: Size;
use iced_aw::iced_fonts;

fn  main() -> iced::Result {
    iced::application("My Time Tracker", App::update, App::view)
        .theme(App::theme)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .window_size(Size::new(500., 300.))
        .run_with(App::new)
}