mod gui;
mod math;
mod neural_network;
mod file_handling;

use gui::app::App;
use gui::main_menu::*;
use math::functions;
use neural_network::network::*;
use file_handling::nnd_file_handler::*;
use file_handling::list_file_handler::*;

use std::fs;
use std::fs::File;
use std::io::Write;

use iced::Theme;

fn main() {
	let _ = iced::application("Neural Network App", App::update, App::view)
		.theme(|_| Theme::Dark)
        .centered()
        .run();
}
