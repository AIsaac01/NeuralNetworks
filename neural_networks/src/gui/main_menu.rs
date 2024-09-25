use iced::widget::{Column, TextInput, column, text_input, button, text};

#[derive(Default)]
pub struct App {
	file: String,
	notification: String
}

#[derive(Debug, Clone)]
pub enum Message {
	FileChange(String),
	LoadFile
}

impl App {
	pub fn view(&self) -> Column<Message> {
        column![
			text_input("Type Here", &self.file).on_input(Message::FileChange),
			button("Load File").on_press(Message::LoadFile),
            text(self.notification.clone()),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
			Message::FileChange(new_str) => {
				self.file = new_str;
			},
			Message::LoadFile => {
				// validate and load nnd file
				let strlen = self.file.len();
				let file_type = &self.file[strlen-4..];
				match file_type {
					".nnd" => { self.notification = String::from("Loading File!") }
					_ => { self.notification = String::from("Error, must use .nnd file!") }
				}
			}
        }
    }
}
