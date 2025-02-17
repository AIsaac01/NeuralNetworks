use iced::widget::{Column, column, text_input, row, button, text, vertical_space, horizontal_space};

use crate::neural_network::network::*;
use crate::file_handling::{nnd_file_handler::*, list_file_handler::*};

use super::app::*;

#[derive(Default)]
pub struct TestMenu {
	nn_filename: String,			// text input for nnd file
	inp_filename: String,			// text input for input file
	inputs: Vec<f32>,				// Numbers read from input file
	notification: String,			// text output for notification bar
}

impl AppPage for TestMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			vertical_space(),
			text("NND file name:"),
			text_input("Enter NND File name here ...", &self.nn_filename).on_input(Message::TestUpdateNNFilename),
			vertical_space(),
			text("Input file name:"),
			text_input("Enter Input File name Here ...", &self.inp_filename).on_input(Message::TestUpdateInpFilename),
			vertical_space(),
			button("Test Network").on_press(Message::TestTestNetwork),
			vertical_space(),
			text(self.notification.clone()),
			vertical_space(),
			row![
				horizontal_space(),
				button("Main Menu").on_press(Message::GoToMainMenu),
				horizontal_space()
			],
			vertical_space(),
		]
	}

	fn update(&mut self, message: &Message) {
		match message {
			Message::GoToMainMenu => {
				println!("Navigtaing to Main Menu!");
			},
			Message::TestUpdateNNFilename(content) => {
				self.nn_filename = String::from(content);
			},
			Message::TestUpdateInpFilename(content) => {
				self.inp_filename = String::from(content);
			},
			Message::TestTestNetwork => {
				// read and validate all files
				let mut nnd_filepath = String::new();
				nnd_filepath.push_str("../nnd_files/");
				nnd_filepath.push_str(&self.nn_filename);
				nnd_filepath.push_str(".nnd");

				let mut network: Network = match read_nnd(&nnd_filepath) {
					None => {
						self.notification = String::from("ERROR: Could Not Read NND File, Check console output");
						return;
					},
					Some(n) => n
				};

				let mut inp_file = String::new();
				inp_file.push_str("../inputs/");
				inp_file.push_str(&self.inp_filename);

					self.inputs = match read_list_file(&inp_file) {
					None => {
						self.notification = String::from("ERROR: Could Not Read Input File, Check console output");
						return;
					},
					Some(n) => n
				};

				// attach inputs
				network.attach_inputs(self.inputs.clone());

				// forward propagate
				network.forward_prop();

				// print output statistics and time
				network.print_neuron_vals();
				self.notification = String::from("Testing Complete!, Check console output for statistics");
			},
			_ => (),
        }
	}
}
