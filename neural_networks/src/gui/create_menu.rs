use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct CreateMenu {
	big_box_text: String
}

impl AppPage for CreateMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			// big box
			scrollable(column![
				text(self.big_box_text.clone()),
				]),
			vertical_space(),
			row![
				horizontal_space(),
				column![
					text("Enter layer size (number of neurons)"),
					text_input("Enter layer size here ..." ,&self.big_box_text).on_input(Message::Create_UpdateBigBox),
					button("add layer")
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

	fn update(&self, message: &Message) {
		match message {
			Message::GoToMainMenu => {
				println!("Navigtaing to Main Menu!");
			},
			Message::Create_UpdateBigBox(content) => {
				println!("{}", content);
				// handle input
			},
			_ => ()
        }
	}
}
