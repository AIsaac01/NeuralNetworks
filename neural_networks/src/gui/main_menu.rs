use iced::widget::{Column, TextInput, Row, column, text_input, row, button, text, scrollable};
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;
use iced::widget::{vertical_space, horizontal_space};

use super::app::{AppPage, Message};

#[derive(Default)]
pub struct MainMenu;

impl AppPage for MainMenu {
	fn view(&self) -> Column<Message> {
		column![
			vertical_space(),
			// create network button
			row![
				horizontal_space(),
				button("Create Network").on_press(Message::GoToCreateNetwork),
				horizontal_space()
			],
			vertical_space(),
			// train network button
			row![
				horizontal_space(),
				button("Train Network").on_press(Message::GoToTrainNetwork),
				horizontal_space()
			],
			vertical_space(),
			// test network button
			row![
				horizontal_space(),
				button("Test Network").on_press(Message::GoToTestNetwork),
				horizontal_space()
			],
			vertical_space(),
			// modify network button
			row![
				horizontal_space(),
				button("Modify Network").on_press(Message::GoToModifyNetwork),
				horizontal_space()
			],
			vertical_space()
		]
	}

	fn update(&mut self, message: &Message) {
		match message {
			Message::GoToMainMenu => (), // already in main menu, this conditional will never execute
			Message::GoToCreateNetwork => {
				println!("Navigtaing to Create Network Window!");
			},
			Message::GoToTrainNetwork => {
				println!("Navigtaing to Train Network Window!");
			},
			Message::GoToTestNetwork => {
				println!("Navigtaing to Test Network Window!");
			},
			Message::GoToModifyNetwork => {
				println!("Navigtaing to Modify Network Window!");
			},
			_ => (),
        }
	}
}
