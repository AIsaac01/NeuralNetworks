// use functions defined in src/math/functions.rs
use crate::math::functions::*;

// defines neuron coordinate within a network: (layer number, neuron number), 0-indexed
type Coord = (usize, usize);

// Possible neuron activation functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationFunction {
	Sigmoid,
	ReLU
}

// Implementing Display trait for printing
impl std::fmt::Display for ActivationFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Sigmoid => "Sigmoid",
            Self::ReLU => "ReLU",
        })
    }
}

// Possible neuron loss functions, typically used in outputs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossFunction {
	MeanSquareError
}

impl std::fmt::Display for LossFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MeanSquareError => "Mean Square Error (MSE)",
        })
    }
}

/****************** Structs ******************/

pub struct Neuron {
	location: Coord,
	actv_func: Option<ActivationFunction>,
	loss_func: Option<LossFunction>,
	value: f32, // value before activation
	activated_value: f32,
	loss: f32
}

// A link is a connection between two neurons
pub struct Link {
	from: Coord,
	to: Coord,
	weight: f32
}

// Defines an entire neural network
pub struct Network {
	matrix: Vec<Vec<Neuron>>, // 2d array of [layer][neuron]
	links: Vec<Link>,
	learning_rate: f32
}

/****************** Implementations ******************/

impl Neuron {
	// returns a new Neuron object
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

	pub fn set_actv_func(&mut self, func: Option<ActivationFunction>) {
		self.actv_func = func;
	}

	pub fn set_loss_func(&mut self, func: Option<LossFunction>) {
		self.loss_func = func;
	}

	pub fn update_val(&mut self, new_val: f32) {
		self.value = new_val;
	}

	// applies activation function to neuron value and updates 'activated_value'
	pub fn activate(&mut self) {
		match &self.actv_func {
			None => {
				self.activated_value = self.value;
			},
			Some(func) => {
				match func {
					ActivationFunction::Sigmoid => self.activated_value = sigmoid(self.value),
					ActivationFunction::ReLU => self.activated_value = relu(self.value),
				}
			},
		}
	}

	// applies the derivative of activation function, used in backprop
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

	// calculates loss of this neuron against a given value
	pub fn calculate_loss(&mut self, expected_output: f32) {
		match &self.loss_func {
			None => {
				println!("No Loss Func, Skipping calculate_loss()");
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

	// returns the string representatino of given neuron
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		s.push_str(self.location.0.to_string().as_str());
		s.push_str(" ");
		s.push_str(self.location.1.to_string().as_str());
		s.push_str(" ");
		let act: &str = match &self.actv_func {
			None => "None",
			Some(a) => match a {
				ActivationFunction::Sigmoid => "Sigmoid",
				ActivationFunction::ReLU => "ReLU"
			}
		};
		s.push_str(act);
		s.push_str(" ");
		let loss: &str = match &self.loss_func {
			None => "None",
			Some(a) => match a {
				LossFunction::MeanSquareError => "MeanSquareError"
			}
		};
		s.push_str(loss);
		s
	}

}


impl Link {
	// returns a new link object
	pub fn new(f: Coord, t: Coord, wht: f32) -> Self {
		Link {
			from: f,
			to: t,
			weight: wht
		}
	}

	// string representation of link
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		s.push_str(self.from.0.to_string().as_str());
		s.push_str(" ");
		s.push_str(self.from.1.to_string().as_str());
		s.push_str(" ");
		s.push_str(self.to.0.to_string().as_str());
		s.push_str(" ");
		s.push_str(self.to.1.to_string().as_str());
		s.push_str(" ");
		s.push_str(self.weight.to_string().as_str());
		s
	}
}


impl Network {
	// creates a new Network object
	pub fn new(lr: f32) -> Network {
		Network {
			matrix: Vec::new(),
			links: Vec::new(),
			learning_rate: lr

		}
	}

	pub fn set_learning_rate(&mut self, lr: f32) {
		self.learning_rate = lr;
	}

	// appends a new layer to the network
	pub fn add_layer(&mut self, layer: Vec<Neuron>) {
		self.matrix.push(layer);
	}

	// creates a new connection in the network
	pub fn add_link(&mut self, e: Link) {
		self.links.push(e);
	}

	// copies values from an input array to the input layer, as long as they match in size
	pub fn attach_inputs(&mut self, inputs: Vec<f32>) {
		if self.matrix[0].len() == 0 {
			self.matrix.remove(0);
		}
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
			// This commented out code might not be necessary because I dont think loss values are overwritten anyways
			// // clear all loss values
			// for layer in self.matrix.iter_mut() {
			// 	for neuron in layer.iter_mut() {
			// 		neuron.loss = 0.0;
			// 	}
			// }
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
				link.weight = link.weight - self.learning_rate * self.matrix[loc.0][loc.1].loss * self.matrix[link.from.0][link.from.1].activated_value;
				to_be_updated.push( link.from );
			}
		}

		for i in to_be_updated.iter() {
			self.update_weights(*i);
		}
	}

	// returns a list of all network weights
	pub fn save_weights(&self) -> Vec<f32> {
		let mut weights: Vec<f32> = Vec::new();
		for link in self.links.iter() {
			weights.push( link.weight );
		}
		weights
	}

	// updates all of the weights in the network from a list of weights
	pub fn attach_weights(&mut self, weights: Vec<f32>) {
		// make sure number of input weights matches number of network weights
		assert_eq!(self.links.len(), weights.len());
		for i in 0..weights.len() {
			self.links[i].weight = weights[i];
		}
	}

	// for debugging, print all neuron values in a network
	pub fn print_neuron_vals(&self) {
		println!("\n\n***************** ITERATION *****************");
		for layer in self.matrix.iter() {
			println!("\n***************** Layer *****************");
			for neuron in layer.iter() {
				println!("{} - activated value = {} - loss = {}", neuron.value, neuron.activated_value, neuron.loss);
			}

		}
	}

	// for debugging, print output neuron values
	pub fn print_output_vals(&self) {
		println!("\n\n***************** Outputs *****************");
		for layer in self.matrix.iter().rev() {
			for neuron in layer.iter() {
				println!("{} - activated value = {} - loss = {}", neuron.value, neuron.activated_value, neuron.loss);
			}
			return;
		}
	}

	// string representatino of a neural network
	pub fn to_string(&self) -> String {
		let mut contents = String::new();
		contents.push_str("Neurons\n");
		for layer in self.matrix.iter() {
			for neuron in layer.iter() {
				contents.push_str( &neuron.to_string().as_str() );
				contents.push('|');
			}
			contents.pop();
			contents.push('\n');
		}

		contents.push_str("Links\n");
		let mut layer_num: usize = 0;
		for link in self.links.iter() {
			if link.from.0 > layer_num {
				contents.pop();
				contents.push('\n');
				layer_num += 1;
			}
			contents.push_str( &link.to_string().as_str() );
			contents.push('|');
		}
		contents.pop();
		contents
	}
}
