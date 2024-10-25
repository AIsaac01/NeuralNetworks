use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use crate::neural_network::network::*;
use crate::file_handling::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct ModifyMenu;

impl AppPage for ModifyMenu {
	fn view(&self) -> Column<Message> {
		column![
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
			Message::GoToCreateNetwork => {
				println!("Navigtaing to Create Menu!");
			},
			Message::GoToTrainNetwork => {
				println!("Navigtaing to Train Network Window!");
			},
			Message::GoToTestNetwork => {
				println!("Navigtaing to Test Network Window!");
			},
			Message::GoToModifyNetwork => (),  // already in modify menu, this conditional will never execute
			_ => (),
        }
	}
}
