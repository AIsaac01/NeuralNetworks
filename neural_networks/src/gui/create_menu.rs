use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use iced::Theme;
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

const MAX_BIG_BOX_LINES: u8 = 10;

#[derive(Default)]
pub struct CreateMenu {
	layers: Vec<u32>,
	big_box_text: String,
	big_box_contents: Vec<String>,
	layer_size_input: String
}

impl AppPage for CreateMenu {
	fn view(&self) -> Column<Message> {
		column![
			text(self.big_box_text.clone()).height(220),
			column![
				button("Scroll Up"),
				button("Scroll Down"),
				button("Remove Most Recent Layer"),
				button("Clear All"),
			],
			vertical_space(),
			row![
				horizontal_space(),
				column![
					text("Enter layer size (number of neurons)"),
					text_input("Enter layer size here ..." , &self.layer_size_input).on_input(Message::Create_UpdateLayerSizeInput),
					button("add layer").on_press(Message::Create_UpdateBigBox),
				],
				horizontal_space(),
				column![
					text("Enter Activation Function"),
					// Use Drop Down List
					// text_input("Enter layer size here ..." , &self.layer_size_input).on_input(Message::Create_UpdateLayerSizeInput),
					button("update").on_press(Message::Create_UpdateBigBox),
				],
				horizontal_space(),
			],
			vertical_space(),
			button("Create Network"),
			vertical_space(),
			// main menu button
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
			Message::Create_UpdateBigBox => {
				println!("{}", self.layer_size_input);

				// read layer size input (unsigned int)
				let layer_size: u32 = match self.layer_size_input.parse::<u32>() {
					Err(e) => {
						println!("ERROR: Layer Size Input  must be unsigned integer");
						return;
					},
					Ok(num) => num,
				};

				// add layer to Vec of layers
				self.layers.push(layer_size);
				let index: usize = self.layers.len();

				// update big box text
				let added_layer_message = format!("Added Layer Number {} with {} neurons\n", self.layers.len(), self.layers[index-1]);
				self.big_box_contents.push(added_layer_message);

				// does this leak memory???????????????????????????????????????????????????
				self.big_box_text = String::new();
				let mut i: u8 = 0;
				for line in self.big_box_contents.iter().rev() {
					if i < MAX_BIG_BOX_LINES {
						self.big_box_text.push_str(line.as_str());
						i += 1;
					}
					else {
						break;
					}
				}


			},
			Message::Create_UpdateLayerSizeInput(content) => {
				self.layer_size_input = String::from(content);
			},
			_ => ()
        }
	}
}
