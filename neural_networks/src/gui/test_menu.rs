use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct TestMenu;

impl AppPage for TestMenu {
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

	fn update(&self, message: &Message) {
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
			Message::GoToTestNetwork => (),  // already in test menu, this conditional will never execute,
			Message::GoToModifyNetwork => {
				println!("Navigtaing to Modify Network Window!");
			},
        }
	}
}
