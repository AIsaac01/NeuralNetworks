use iced::widget::{Column, column, text_input, row, button, text, vertical_space, horizontal_space, progress_bar, checkbox};

use crate::neural_network::network::*;
use crate::file_handling::{nnd_file_handler::*, list_file_handler::*};

use super::app::*;

#[derive(Default)]
pub struct TrainMenu {
	nn_filename: String,			// text input for nnd file
	inp_filename: String,			// text input for input file
	out_filename: String,			// text input for output file
	epochs: String,					// text input for epochs
	learning_rate: String,			// text input for learning rate
	progress: f32,					// progress bar value (range is 0 -> 100 as defined below)
	notification: String,			// text output for notification bar
	inputs: Vec<f32>,				// Numbers read from input file
	expected_outputs: Vec<f32>,		// Numbers read from output file
	save_weights: bool,				// Saves weights at end of training if checkbox is set true
}

impl AppPage for TrainMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			text("NND file name:"),
			text_input("Enter NND File name here ...", &self.nn_filename).on_input(Message::TrainUpdateNNFilename),
			vertical_space(),
			text("Input file name:"),
			text_input("Enter Input File name Here ...", &self.inp_filename).on_input(Message::TrainUpdateInpFilename),
			vertical_space(),
			text("Expected Output file name:"),
			text_input("Enter Expected Outut File name Here ...", &self.out_filename).on_input(Message::TrainUpdateOutFilename),
			vertical_space(),
			text("Training Epochs:"),
			text_input("Enter Epochs ...", &self.epochs).on_input(Message::TrainUpdateEpochs),
			vertical_space(),
			text("Training Learning Rate:"),
			text_input("Enter Learning Rate ...", &self.learning_rate).on_input(Message::TrainUpdateLearningRate),
			vertical_space(),
			checkbox("Save Weights", self.save_weights).on_toggle(Message::TrainCheckSaveWeights),
			vertical_space(),
			button("Train Network").on_press(Message::TrainTrainNetwork),
			vertical_space(),
			progress_bar(0.0..=100.0, self.progress),
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
			Message::TrainUpdateNNFilename(content) => {
				self.nn_filename = String::from(content);
			},
			Message::TrainUpdateInpFilename(content) => {
				self.inp_filename = String::from(content);
			},
			Message::TrainUpdateOutFilename(content) => {
				self.out_filename = String::from(content);
			},
			Message::TrainUpdateEpochs(content) => {
				self.epochs = String::from(content);
			},
			Message::TrainUpdateLearningRate(content) => {
				self.learning_rate = String::from(content);
			},
			Message::TrainCheckSaveWeights(b) => {
				self.save_weights = *b;
			},
			Message::TrainTrainNetwork => {
				let mut nn_file = String::new();
				nn_file.push_str("../nnd_files/");
				nn_file.push_str(&self.nn_filename);
				nn_file.push_str(".nnd");
				let mut inp_file = String::new();
				inp_file.push_str("../inputs/");
				inp_file.push_str(&self.inp_filename);
				let mut out_file = String::new();
				out_file.push_str("../expected_outputs/");
				out_file.push_str(&self.out_filename);

				// read and validate all files
				let mut network: Network = match read_nnd(&nn_file) {
					None => {
						self.notification = String::from("ERROR: Could Not Read NND File, Check console output");
						return;
					},
					Some(n) => n
				};

					self.inputs = match read_list_file(&inp_file) {
					None => {
						self.notification = String::from("ERROR: Could Not Read Input File, Check console output");
						return;
					},
					Some(n) => n
				};

					self.expected_outputs = match read_list_file(&out_file) {
					None => {
						self.notification = String::from("ERROR: Could Not Read Output File, Check console output");
						return;
					},
					Some(n) => n
				};

				// update learning rate
				let lr: f32 = self.learning_rate.parse().unwrap();
				network.set_learning_rate(lr);

				// attach inputs
				network.attach_inputs(self.inputs.clone());

				// cycle through epochs
				self.progress = 0.0;
				let epochs: u32 = self.epochs.parse().unwrap();
				for i in 0..epochs {
					network.forward_prop();
					network.back_prop(self.expected_outputs.clone());
					self.progress += ((i as f32) / (epochs as f32)) * 100.0;
				}

				// print output statistics and time
				network.print_output_vals();
				self.notification = String::from("Training Complete!, Check console output for statistics");

				// save network if user wants to
				if self.save_weights {
					let mut new_path = String::new();
					new_path.push_str("../nnd_files/");
					new_path.push_str(&self.nn_filename);
					new_path.push_str("_trained.nnd");
					// TODO: remove old file
					// write new one
					write_nnd(&new_path, network);
				}
			},
			_ => (),
        }
	}
}
