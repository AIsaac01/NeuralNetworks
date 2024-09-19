// const LEARNING_RATE: f32 = 0.25;
//
// type Coord = (usize, usize);
//
// /****************** Enums ******************/
//
// pub enum ActivationFunction {
// 	Sigmoid,
// 	D_Sigmoid,
// 	ReLU,
// 	D_ReLU
// }
//
// pub enum LossFunction {
// 	MeanSquareError
// }
//
// /****************** Structs ******************/
//
// pub struct Neuron {
// 	location: Coord,
// 	actv_func: Option<ActivationFunction>,
// 	loss_func: Option<LossFunction>,
// 	value: f32,
// 	loss: f32
// }
//
// pub struct Link {
// 	from: Coord,
// 	to: Coord,
// 	weight: f32
// }
//
// pub struct Network {
// 	matrix: Vec<Vec<Neuron>>,
// 	links: Vec<Link>,
// }
//
// /****************** Implementations ******************/
//
// impl Neuron {
// 	pub fn new(layer_num: usize, index: usize, actv: Option<ActivationFunction>, loss: Option<LossFunction>) -> Self {
// 		Neuron {
// 			location: (layer_num, index),
// 			actv_func: actv,
// 			loss_func: loss,
// 			value: 0.0,
// 			loss: 0.0
// 		}
// 	}
//
// 	pub fn set_actv_func(&mut self, func: ActivationFunction) {
// 		self.actv_func = Some(func);
// 	}
//
// 	pub fn set_loss_func(&mut self, func: LossFunction) {
// 		self.loss_func = Some(func);
// 	}
//
// 	pub fn update_val(&mut self, new_val: f32) {
// 		self.value = new_val;
// 	}
//
// 	pub fn activate(&mut self) {
// 		match &self.actv_func {
// 			None => return,
// 			Some(func) => {
// 				match func {
// 					ActivationFunction::Sigmoid => {
// 						let mut val = self.value;
// 						// fast sigmoid function
// 						if val < 0.0 {
// 							val *= -1.0;
// 						}
// 						self.value = val / (1.0 + val);
// 					},
// 					ActivationFunction::D_Sigmoid => {
// 						let mut val = self.value;
// 						if val < 0.0 {
// 							self.value = 0.0;
// 						}
// 						else if val > 1.0 {
// 							val = 1.0;
// 						}
// 					},
// 					ActivationFunction::ReLU => {
// 						let mut val = self.value;
// 						if val < 0.0 {
// 							self.value = 0.0;
// 						}
// 						else if val > 1.0 {
// 							val = 1.0;
// 						}
// 					},
// 					ActivationFunction::D_ReLU => {
// 						let mut val = self.value;
// 						if val < 0.0 {
// 							self.value = 0.0;
// 						}
// 						else if val > 1.0 {
// 							val = 1.0;
// 						}
// 					}
// 				}
// 			},
// 		}
// 	}
//
// 	pub fn calculate_loss(&mut self, expected_output: f32) {
// 		match &self.loss_func {
// 			None => {
// 				println!("No Loss Func, Skipping calculate_loss()");
// 				return;
// 			},
// 			Some(func) => {
// 				match func {
// 					LossFunction::MeanSquareError => {
// 						// println!("Executing loss func");
// 						self.loss = 0.5 * (self.value - expected_output) * (self.value - expected_output);
// 					}
// 				}
// 			}
// 		}
// 	}
//
// }
//
//
// impl Link {
// 	pub fn new(f: Coord, t: Coord, wht: f32) -> Self {
// 		Link {
// 			from: f,
// 			to: t,
// 			weight: wht
// 		}
// 	}
// }
//
//
// impl Network {
// 	pub fn new() -> Network {
// 		Network {
// 			matrix: Vec::new(),
// 			links: Vec::new(),
// 		}
// 	}
//
// 	pub fn add_layer(&mut self, layer: Vec<Neuron>) {
// 		self.matrix.push(layer)
// 	}
//
// 	pub fn add_link(&mut self, e: Link) {
// 		self.links.push(e);
// 	}
//
// 	pub fn attach_inputs(&mut self, inputs: Vec<f32>) {
// 		assert_eq!( inputs.len(), self.matrix[0].len() );
// 		for i in 0..inputs.len() {
// 			self.matrix[0][i].value = inputs[i];
// 		}
// 	}
//
// 	// propagates values from input to output, activating
// 	// neurons after when each layer completes propagation
// 	pub fn forward_prop(&mut self) {
// 		let mut layer_num: usize = 0;
//
// 		// iterate through each neuron-neuron connection
// 		for link in self.links.iter() {
// 			// check if previous layer has propagated
// 			if link.from.0 > layer_num {
// 				layer_num += 1;
// 				// activate this layer before propagating to the next
// 				for neuron in self.matrix[layer_num].iter_mut() {
// 					neuron.activate();
// 				}
// 			}
//
// 			// propagation
// 			let from_val = self.matrix[link.from.0][link.from.1].value;
// 			let to_val = from_val * link.weight;
// 			self.matrix[link.to.0][link.to.1].value += to_val;
// 		}
//
// 		// activate output layer
// 		layer_num += 1;
// 		for neuron in self.matrix[layer_num].iter_mut() {
// 			neuron.activate();
// 		}
// 	}
//
// 	pub fn back_prop(&mut self, expected_outputs: Vec<f32>) {
// 		// make sure expected outputs and number of network outputs match
// 		let output_layer_index = self.matrix.len() - 1;
// 		let num_outputs = self.matrix[output_layer_index].len();
// 		assert_eq!(expected_outputs.len(), num_outputs);
//
// 		// clear all loss values
// 		for layer in self.matrix.iter_mut() {
// 			for neuron in layer.iter_mut() {
// 				neuron.loss = 0.0;
// 			}
// 		}
//
// 		// calculate loss
// 		for i in 0..num_outputs {
// 			self.matrix[output_layer_index][i].calculate_loss( expected_outputs[i] );
// 			self.prop_loss( self.matrix[output_layer_index][i].location );
// 			// update weights
// 			self.update_weights( self.matrix[output_layer_index][i].location );
// 		}
//
// 	}
//
// 	pub fn prop_loss(&mut self, loc: Coord) {
// 		let mut to_be_updated: Vec<Coord> = Vec::new();
// 		for link in self.links.iter() {
// 			if link.to == loc {
// 				self.matrix[link.from.0][link.from.1].loss += self.matrix[loc.0][loc.1].loss * link.weight; // this needs to be fixed
// 				to_be_updated.push((link.from.0, link.from.1));
// 			}
// 		}
// 		for i in to_be_updated.iter() {
// 			self.prop_loss(*i);
// 		}
// 	}
//
// 	pub fn update_weights(&mut self, loc: Coord) {
// 		let mut to_be_updated: Vec<Coord> = Vec::new();
// 		for link in self.links.iter_mut() {
// 			if link.to == loc {
// 				link.weight = link.weight - LEARNING_RATE * self.matrix[loc.0][loc.1].loss * self.matrix[link.from.0][link.from.1].value;
// 				to_be_updated.push( link.from );
// 			}
// 		}
//
// 		for i in to_be_updated.iter() {
// 			self.update_weights(*i);
// 		}
// 	}
//
// 	pub fn print_neuron_vals(&self) {
// 		for layer in self.matrix.iter() {
// 			println!("\nLayer");
// 			for neuron in layer.iter() {
// 				println!("{} - loss = {}", neuron.value, neuron.loss);
// 			}
// 		}
// 	}
//
//
// }
