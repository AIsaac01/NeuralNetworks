use iced::widget::{Column, column, text_input, row, button,
				   text, vertical_space, horizontal_space};

use crate::neural_network::network::*;
use crate::file_handling::nnd_file_handler::*;
use crate::read_list_file;

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct TestMenu {
	nn_filepath: String,			// text input for nnd file
	inp_filepath: String,			// text input for input file
	inputs: Vec<f32>,				// Numbers read from input file
	notification: String,			// text output for notification bar
}

impl AppPage for TestMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			vertical_space(),
			text("NND file path:"),
			text_input("Enter NND Filepath here ...", &self.nn_filepath).on_input(Message::Test_UpdateNNFilePath),
			vertical_space(),
			text("Input file path:"),
			text_input("Enter Input Filepath Here ...", &self.inp_filepath).on_input(Message::Test_UpdateInpFilePath),
			vertical_space(),
			button("Test Network").on_press(Message::Test_TestNetwork),
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
			Message::Test_UpdateNNFilePath(content) => {
				self.nn_filepath = String::from(content);
			},
			Message::Test_UpdateInpFilePath(content) => {
				self.inp_filepath = String::from(content);
			},
			Message::Test_TestNetwork => {
				// read and validate all files
				let mut network: Network = match read_nnd(&self.nn_filepath) {
					None => {
						self.notification = String::from("ERROR: Could Not Read NND File, Check console output");
						return;
					},
					Some(n) => n
				};

				let inputs: Vec<f32> = match read_list_file(&self.inp_filepath) {
					None => {
						self.notification = String::from("ERROR: Could Not Read Input File, Check console output");
						return;
					},
					Some(n) => n
				};

				// attach inputs
				network.attach_inputs(inputs);

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
