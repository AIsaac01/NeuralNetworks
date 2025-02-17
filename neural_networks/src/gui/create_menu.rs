use iced::widget::{Column, column, text_input, row, button, text, pick_list, vertical_space, horizontal_space};

use crate::neural_network::network::*;
use crate::file_handling::nnd_file_handler::*;

use super::app::*;

const MAX_BIG_BOX_LINES: u8 = 10;

#[derive(Default)]
pub struct CreateMenu {
	layers: Vec<u32>,
	big_box_text: String,
	big_box_contents: Vec<String>,
	layer_size_input: String,
	activation_func: Option<ActivationFunction>,
	loss_func: Option<LossFunction>,
	filename: String,
	notification: String
}

impl AppPage for CreateMenu {
	fn view(&self) -> Column<Message> {
		let actv_funcs = [
			ActivationFunction:: Sigmoid,
			ActivationFunction:: ReLU,
		];
		let loss_funcs = [
			LossFunction::MeanSquareError
		];
		column![
			text(self.big_box_text.clone()).height(220),
			column![
				button("Scroll Up"),
				button("Scroll Down"),
				button("Remove Most Recent Layer").on_press(Message::CreateRemoveRecentLayer),
				button("Clear All").on_press(Message::CreateClearAll),
			],
			vertical_space(),
			row![
				horizontal_space(),
				column![
					text("Enter layer size (number of neurons)"),
					text_input("Enter layer size here ..." , &self.layer_size_input).on_input(Message::CreateUpdateLayerSizeInput),
					button("add layer").on_press(Message::CreateUpdateBigBox),
				],
				horizontal_space(),
				column![
					text("Enter Activation Function"),
					pick_list(
				        actv_funcs,
				        self.activation_func,
				        Message::CreatePickActvFunc,
				    ).placeholder("Select Activation Function"),
				],
				horizontal_space(),
				column![
					text("Enter Loss Function"),
					pick_list(
				        loss_funcs,
				        self.loss_func,
				        Message::CreatePickLossFunc,
				    ).placeholder("Select Loss Function"),
				],
			],
			vertical_space(),
			text_input("Enter File name,.." , &self.filename).on_input(Message::CreateChooseFilename),
			button("Create Network").on_press(Message::CreateCreateNetwork),
			vertical_space(),
			// main menu button
			row![
				horizontal_space(),
				button("Main Menu").on_press(Message::GoToMainMenu),
				horizontal_space()
			],
			vertical_space(),
			text(self.notification.clone()),
			vertical_space(),
		]
	}

	fn update(&mut self, message: &Message) {
		match message {
			Message::GoToMainMenu => {
				println!("Navigtaing to Main Menu!");
			},
			Message::CreateUpdateBigBox => {
				println!("{}", self.layer_size_input);

				// read layer size input (unsigned int)
				let layer_size: u32 = match self.layer_size_input.parse::<u32>() {
					Err(_e) => {
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
			Message::CreateRemoveRecentLayer => {
				self.layers.pop();
				self.big_box_contents.pop();

				// reused code: BAD, U LAZY LITTLE SHIT
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
			Message::CreateClearAll => {
				self.layers.clear();
				self.big_box_contents.clear();
				self.big_box_text = String::new();
			},
			Message::CreateUpdateLayerSizeInput(content) => {
				self.layer_size_input = String::from(content);
			},
			Message::CreatePickActvFunc(content) => {
				self.activation_func = Some(*content);
			},
			Message::CreatePickLossFunc(content) => {
				self.loss_func = Some(*content);
			},
			Message::CreateChooseFilename(content) => {
				self.filename = String::from(content);
			},
			Message::CreateCreateNetwork => {
				// heavy lifting here

				// check if filepath is right
				let mut f: String = String::new();
				f.push_str("../nnd_files/");
				f.push_str(self.filename.as_str());
				f.push_str(".nnd");
				println!("Path = {}", f);
				let exists = nnd_exists(f.as_str());
				if exists {
					self.notification = String::from("Error: NND File already Exists!");
					return;
				}

				// construct Network, initial learning rate is 0.0
				let mut new_network: Network = Network::new(0.0);

				// create neurons from each layer
				for layer_num in 0..self.layers.len() {
					let mut neurons: Vec<Neuron> = Vec::new();
					for index in 0..self.layers[layer_num] {
						neurons.push( Neuron::new(layer_num, index as usize, self.activation_func, None) );
					}
					if (layer_num == self.layers.len() - 1) && self.loss_func.is_some() {
						for neuron in &mut neurons {
							neuron.set_loss_func(self.loss_func);
						}
					}
					new_network.add_layer(neurons);
				}

				// fully connect the network through links
				for layer_num in 0..self.layers.len()-1 {
					let num_neurons_cur = self.layers[layer_num];
					let num_neurons_next = self.layers[layer_num+1];

					for i in 0..num_neurons_cur {
						for j in 0..num_neurons_next {
							new_network.add_link( Link::new((layer_num as usize, i as usize), (layer_num+1 as usize, j as usize), 0.5) );
						}
					}
				}

				// create the file
				write_nnd(f.as_str(), new_network);

				// nothing caught on fire
				self.notification = String::from("Success!");
			}
			_ => ()
        }
	}
}
