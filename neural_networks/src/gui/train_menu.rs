use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use crate::neural_network::network::*;
use crate::file_handling::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};
use iced::widget::progress_bar;
use iced::widget::checkbox;

use super::app::{AppPage, Message};
use crate::read_list_file;

#[derive(Default)]
pub struct TrainMenu {
	nn_filepath: String,
	inp_filepath: String,
	out_filepath: String,
	epochs: String,
	learning_rate: String,
	progress: f32,
	notification: String,
	inputs: Vec<f32>,
	expected_outputs: Vec<f32>,
	save_weights: bool,
}

impl AppPage for TrainMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			text("NND file path:"),
			text_input("Enter NND Filepath here ...", &self.nn_filepath).on_input(Message::Train_UpdateNNFilePath),
			vertical_space(),
			text("Input file path:"),
			text_input("Enter Input Filepath Here ...", &self.inp_filepath).on_input(Message::Train_UpdateInpFilePath),
			vertical_space(),
			text("Expected Output file path:"),
			text_input("Enter Expected Outut Filepath Here ...", &self.out_filepath).on_input(Message::Train_UpdateOutFilePath),
			vertical_space(),
			text("Training Epochs:"),
			text_input("Enter Epochs ...", &self.epochs).on_input(Message::Train_UpdateEpochs),
			vertical_space(),
			text("Training Learning Rate:"),
			text_input("Enter Learning Rate ...", &self.learning_rate).on_input(Message::Train_UpdateLearningRate),
			vertical_space(),
			checkbox("Save Weights", self.save_weights).on_toggle(Message::Train_CheckSaveWeights),
			vertical_space(),
			button("Train Network").on_press(Message::Train_TrainNetwork),
			vertical_space(),
			progress_bar(0.0..=100.0, self.progress),
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
			Message::Train_UpdateNNFilePath(content) => {
				self.nn_filepath = String::from(content);
			},
			Message::Train_UpdateInpFilePath(content) => {
				self.inp_filepath = String::from(content);
			},
			Message::Train_UpdateOutFilePath(content) => {
				self.out_filepath = String::from(content);
			},
			Message::Train_UpdateEpochs(content) => {
				self.epochs = String::from(content);
			},
			Message::Train_UpdateLearningRate(content) => {
				self.learning_rate = String::from(content);
			},
			Message::Train_CheckSaveWeights(b) => {
				self.save_weights = *b;
			},
			Message::Train_TrainNetwork => {
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

				let expected_outputs: Vec<f32> = match read_list_file(&self.out_filepath) {
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
				network.attach_inputs(inputs);

				// cycle through epochs
				let epochs: u32 = self.epochs.parse().unwrap();
				for i in 0..epochs {
					network.forward_prop();
					network.back_prop(expected_outputs.clone());
					self.progress += ((i as f32) / (epochs as f32)) * 100.0;
				}

				// print output statistics and time
				network.print_output_vals();
				self.notification = String::from("Training Complete!, Check console output for statistics");

				// save network if user wants to
				if self.save_weights {
					let mut new_path = self.nn_filepath.clone();
					for _ in 0..4 {
						new_path.pop();
					}
					new_path.push_str("_trained.nnd");
					write_nnd(&new_path, network);
				}
			},
			_ => (),
        }
	}
}
