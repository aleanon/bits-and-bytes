#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod converter;
mod app;

use app::Application;
use iced::{window, Size};

const ICON: &'static [u8] = include_bytes!("./bits_and_bytes_60x48.png"); 


fn main() {
    let icon = window::icon::from_file_data(
        ICON,
        None,
    )
    .unwrap();

    let settings = window::Settings {
        min_size: Some(Size{height: 600., width: 500.}),
        icon: Some(icon),
        ..Default::default()
    };

    iced::application("Bits and bytes", Application::update, Application::view)
        .window(settings)
        .run_with(||Application::new()).unwrap();
}

