use crate::functions::*;

const LEARNING_RATE: f32 = 0.05;

// coordinate for neuron is (layer number, neuron number), 0-indexed
type Coord = (usize, usize);

/****************** Enums ******************/

pub enum ActivationFunction {
	Sigmoid,
	ReLU
}

pub enum LossFunction {
	MeanSquareError
}

/****************** Structs ******************/

pub struct Neuron {
	location: Coord,
	actv_func: Option<ActivationFunction>,
	loss_func: Option<LossFunction>,
	value: f32,
	activated_value: f32,
	loss: f32
}

pub struct Link {
	from: Coord,
	to: Coord,
	weight: f32
}

pub struct Network {
	matrix: Vec<Vec<Neuron>>,
	links: Vec<Link>,
}

/****************** Implementations ******************/

impl Neuron {
	pub fn new(layer_num: usize, index: usize, actv: Option<ActivationFunction>, loss: Option<LossFunction>) -> Self {
		Neuron {
			location: (layer_num, index),
			actv_func: actv,
			loss_func: loss,
			value: 0.0,
			activated_value: 0.0,
			loss: 0.0
		}
	}

	pub fn set_actv_func(&mut self, func: ActivationFunction) {
		self.actv_func = Some(func);
	}

	pub fn set_loss_func(&mut self, func: LossFunction) {
		self.loss_func = Some(func);
	}

	pub fn update_val(&mut self, new_val: f32) {
		self.value = new_val;
	}

	pub fn activate(&mut self) {
		match &self.actv_func {
			None => {
				self.activated_value = self.value;
				//println!("No activation function, activated value = {}", self.activated_value);
			},
			Some(func) => {
				match func {
					ActivationFunction::Sigmoid => self.activated_value = sigmoid(self.value),
					ActivationFunction::ReLU => self.activated_value = relu(self.value),
				}
			},
		}
	}

	pub fn d_activate(&mut self) -> f32 {
		match &self.actv_func {
			None => return 1.0,
			Some(func) => {
				match func {
					ActivationFunction::Sigmoid => return d_sigmoid(self.value),
					ActivationFunction::ReLU => return d_relu(self.value),
				}
			},
		}
	}

	pub fn calculate_loss(&mut self, expected_output: f32) {
		match &self.loss_func {
			None => {
				println!("No Loss Func, Skipping calculate_loss()");
				return;
			},
			Some(func) => {
				match func {
					LossFunction::MeanSquareError => {
						self.loss = d_mean_squared_error(self.value, expected_output);
					}
				}
			}
		}
	}

}


impl Link {
	pub fn new(f: Coord, t: Coord, wht: f32) -> Self {
		Link {
			from: f,
			to: t,
			weight: wht
		}
	}
}


impl Network {
	pub fn new() -> Network {
		Network {
			matrix: Vec::new(),
			links: Vec::new(),
		}
	}

	pub fn add_layer(&mut self, layer: Vec<Neuron>) {
		self.matrix.push(layer)
	}

	pub fn add_link(&mut self, e: Link) {
		self.links.push(e);
	}

	pub fn attach_inputs(&mut self, inputs: Vec<f32>) {
		assert_eq!( inputs.len(), self.matrix[0].len() );
		for i in 0..inputs.len() {
			self.matrix[0][i].value = inputs[i];
		}
	}

	// propagates values from input to output, activating
	// neurons after when each layer completes propagation
	pub fn forward_prop(&mut self) {
		// activate input layer
		for neuron in self.matrix[0].iter_mut() {
			neuron.activate();
		}

		// clear all values except input
		for layer in self.matrix.iter_mut() {
			for neuron in layer.iter_mut() {
				// skip input neurons
				if neuron.location.0 != 0 {
					neuron.value = 0.0;
					neuron.activated_value = 0.0;
				}
			}
		}

		// iterate through each neuron-neuron connection
		let mut layer_num: usize = 0;
		for link in self.links.iter() {
			// check if previous layer has propagated
			if link.from.0 > layer_num {
				layer_num += 1;
				// activate this layer before propagating to the next
				for neuron in self.matrix[layer_num].iter_mut() {
					// neuron.loss = neuron.d_activate();
					neuron.activate();
				}
			}

			// propagation
			let from_val = self.matrix[link.from.0][link.from.1].activated_value;
			let to_val = from_val * link.weight;
			self.matrix[link.to.0][link.to.1].value += to_val;
		}

		// activate output layer
		layer_num += 1;
		for neuron in self.matrix[layer_num].iter_mut() {
			neuron.activate();
		}
	}

	pub fn back_prop(&mut self, expected_outputs: Vec<f32>) {
		// make sure expected outputs and number of network outputs match
		let output_layer_index = self.matrix.len() - 1;
		let num_outputs = self.matrix[output_layer_index].len();
		assert_eq!(expected_outputs.len(), num_outputs);

		// calculate loss for each output neuron and backpropagate loss
		for i in 0..num_outputs {
			// clear all loss values
			for layer in self.matrix.iter_mut() {
				for neuron in layer.iter_mut() {
					neuron.loss = 0.0;
				}
			}
			self.matrix[output_layer_index][i].calculate_loss( expected_outputs[i] );
			self.prop_loss( (output_layer_index, i) );
			self.update_weights( (output_layer_index, i) );
		}

	}

	// recursively back-propagate loss with respect to an output neuron
	pub fn prop_loss(&mut self, loc: Coord) {
		let mut to_be_updated: Vec<Coord> = Vec::new();
		self.matrix[loc.0][loc.1].loss *= self.matrix[loc.0][loc.1].d_activate();
		for link in self.links.iter() {
			if link.to == loc {
				self.matrix[link.from.0][link.from.1].loss += self.matrix[loc.0][loc.1].loss * link.weight;
				to_be_updated.push((link.from.0, link.from.1));
			}
		}
		for i in to_be_updated.iter() {
			self.prop_loss(*i);
		}
	}

	// update weights attached to neuron at 'coord', and recursively
	// do that to each neuron connected to 'coord' from previous layer
	pub fn update_weights(&mut self, loc: Coord) {
		let mut to_be_updated: Vec<Coord> = Vec::new();
		for link in self.links.iter_mut() {
			if link.to == loc {
				// update weight
				link.weight = link.weight - LEARNING_RATE * self.matrix[loc.0][loc.1].loss * self.matrix[link.from.0][link.from.1].activated_value;
				to_be_updated.push( link.from );
			}
		}

		for i in to_be_updated.iter() {
			self.update_weights(*i);
		}
	}

	pub fn print_neuron_vals(&self) {
		println!("\n\n***************** ITERATION *****************");
		for layer in self.matrix.iter() {
			println!("\n***************** Layer *****************");
			for neuron in layer.iter() {
				println!("{} - activated value = {} - loss = {}", neuron.value, neuron.activated_value, neuron.loss);
			}

		}
	}


}
