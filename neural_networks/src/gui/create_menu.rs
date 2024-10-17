use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use iced::Theme;
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct CreateMenu {
	num_layers: u32,
	big_box_text: Vec<String>,
	layer_size_input: String
}

impl AppPage for CreateMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),

			// big box, add up and down buttons for scrolling, instead of a scrollbar, because the scrollbar doesn't fucking work in Iced
			text(self.big_box_text.clone()),
			column![
				button("Scroll Up"),
				button("Scroll Down"),
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
			],
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
				// TODO
			},
			Message::Create_UpdateLayerSizeInput(content) => {
				self.layer_size_input = String::from(content);
			},
			_ => ()
        }
	}
}
