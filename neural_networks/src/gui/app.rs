use iced::widget::{Column, column};
use crate::neural_network::network::*;
use crate::nnd::nnd_file_handler::*;

use super::main_menu::*;
use super::create_menu::*;
use super::train_menu::*;
use super::test_menu::*;
use super::modify_menu::*;


// describes what functions a page, such as main menu, should include
pub trait AppPage {
	fn view(&self) -> Column<Message>;
	fn update(&self, message: &Message);
}

// describes which menu/sub-menu the app is in
#[derive(Default)]
pub enum AppState {
	#[default]
	InMainMenu,
	InCreateMenu,
	InTrainMenu,
	InTestMenu,
	InModifyMenu
}

#[derive(Debug, Clone)]
pub enum Message {
	GoToMainMenu,
	GoToCreateNetwork,
	GoToTrainNetwork,
	GoToTestNetwork,
	GoToModifyNetwork,
}

pub struct App {
	pub state: AppState,
	main_menu: MainMenu,
	create_menu: Option<CreateMenu>,
	train_menu: Option<TrainMenu>,
	test_menu: Option<TestMenu>,
	modify_menu: Option<ModifyMenu>

}

impl Default for App {
	fn default() -> Self {
		App {
			state: AppState::InMainMenu,
			main_menu: MainMenu::default(),
			create_menu: None,
			train_menu: None,
			test_menu: None,
			modify_menu: None

		}
	}
}

impl App {
	pub fn view(&self) -> Column<Message> {
		match self.state {
			AppState::InMainMenu => self.main_menu.view(),
			AppState::InCreateMenu => self.create_menu.as_ref().unwrap().view(),
			AppState::InTrainMenu => self.train_menu.as_ref().unwrap().view(),
			AppState::InTestMenu => self.test_menu.as_ref().unwrap().view(),
			AppState::InModifyMenu => self.modify_menu.as_ref().unwrap().view(),
		}
    }

    pub fn update(&mut self, message: Message) {
		match message {
			Message::GoToMainMenu => {
				self.state = AppState::InMainMenu;
			},
			Message::GoToCreateNetwork => {
				if self.create_menu.is_none() {
					self.create_menu = Some(CreateMenu::default());
				}
				self.state = AppState::InCreateMenu;
			},
			Message::GoToTrainNetwork => {
				if self.train_menu.is_none() {
					self.train_menu = Some(TrainMenu::default());
				}
				self.state = AppState::InTrainMenu;
			},
			Message::GoToTestNetwork => {
				if self.test_menu.is_none() {
					self.test_menu = Some(TestMenu::default());
				}
				self.state = AppState::InTestMenu;
			},
			Message::GoToModifyNetwork => {
				if self.modify_menu.is_none() {
					self.modify_menu = Some(ModifyMenu::default());
				}
				self.state = AppState::InModifyMenu;
			},
			_ => () // do nothing
		};

		match self.state {
			AppState::InMainMenu => self.main_menu.update(&message),
			AppState::InCreateMenu => self.create_menu.as_ref().unwrap().update(&message),
			AppState::InTrainMenu => self.train_menu.as_ref().unwrap().update(&message),
			AppState::InTestMenu => self.test_menu.as_ref().unwrap().update(&message),
			AppState::InModifyMenu => self.modify_menu.as_ref().unwrap().update(&message)
		};
    }
}
