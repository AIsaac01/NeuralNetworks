use std::fs;
use std::fs::File;
use std::io::Write;
use crate::neural_network::network::*;

pub fn nnd_exists(filepath: &str) -> bool {
	let exists = File::open(filepath);
	match exists {
		Ok(f) => return true,
		Err(e) => return false
	}
}

pub fn read_nnd(filepath: &str) -> Option<Network> {
	let contents = match fs::read_to_string(filepath) {
		Ok(c) => c,
        Err(e) => {
			println!("ERROR: Invalid File Path!");
			return None;
		},
	};
	let mut network = Network::new(0.0);

	let mut reading_neuron: bool = false;
	let mut reading_link: bool = false;
	let mut reading_learning_rate = false;
	// split string into lines
	for line in contents.lines() {
		let mut layer: Vec<Neuron> = Vec::new();

		for word in line.split(" ") {
			match word {
				"Neurons" => {
					reading_link = false;
					reading_neuron = true;
					reading_learning_rate = false;
					continue;
				},
				"Links" => {
					reading_neuron = false;
					reading_link = true;
					reading_learning_rate = false;
					continue;
				},
				"LearningRate" => {
					reading_neuron = false;
					reading_link = false;
					reading_learning_rate = true;
					continue;
				},
				_ => {
					if reading_neuron == true {
						let neuron = match read_neuron(String::from(word)) {
							None => {
								println!("ERROR: read_nnd(): error reading neuron");
								return None;
							}
							Some(n) => n
						};
						layer.push(neuron);
					}
					if reading_link == true {
						let link = match read_link(String::from(word)) {
							None => {
								println!("ERROR: read_nnd(): error reading link");
								return None;
							}
							Some(l) => l
						};
						network.add_link(link);
					}
					if reading_learning_rate == true {
						network.set_learning_rate( word.parse().unwrap() );
					}
				}
			}
		}

		if reading_neuron {
			network.add_layer( layer );
			layer = Vec::new();
		}
	}
	Some(network)
}

pub fn write_nnd(filepath: &str, network: Network) {
	let mut file = match File::create_new(filepath) {
		Err(e) => {
			println!("ERROR: write_nnd(): file already exists!");
			return;
		}
		Ok(f) => f
	};

	file.write_all( network.to_string().as_str().as_bytes() ).unwrap();

}

fn read_neuron(str: String) -> Option<Neuron> {
	let mut s = str.split("-");

	let layer: usize = match s.next() {
		None => {
			println!("ERROR: read_neuron(): No Layer Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_neuron(): Cannot Read Neuron Layer Number");
				return None;
			}
		}
	};

	let index: usize = match s.next() {
		None => {
			println!("ERROR: read_neuron(): No Index Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_neuron(): Cannot Read Neuron Index Number");
				return None;
			}
		}
	};

	let actv: Option<ActivationFunction> = match s.next() {
		None => {
			println!("ERROR: read_neuron(): No Activation Function Found");
			return None;
		},
		Some(s) => match s {
			"Sigmoid" => Some(ActivationFunction::Sigmoid),
			"ReLU" => Some(ActivationFunction::ReLU),
			"None" => None,
			_ => {
				println!("ERROR: read_neuron(): Invalid Activation Function");
				return None;
			}
		}
	};

	let loss: Option<LossFunction> = match s.next() {
		None => {
			println!("ERROR: read_neuron(): No Loss Function Found");
			return None;
		},
		Some(s) => match s {
			"MeanSquareError" => Some(LossFunction::MeanSquareError),
			"None" => None,
			_ => {
				println!("ERROR: read_neuron(): Invalid Loss Function");
				return None;
			}
		}
	};

	Some(Neuron::new(layer, index, actv, loss))
}

fn read_link(str: String) -> Option<Link> {
	let mut s = str.split("-");

	let from_layer: usize = match s.next() {
		None => {
			println!("ERROR: read_link(): No From_Layer Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_link(): Cannot Read Link From_Layer");
				return None;
			}
		}
	};

	let from_index: usize = match s.next() {
		None => {
			println!("ERROR: read_link(): No From_Index Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_link(): Cannot Read Link From_Index");
				return None;
			}
		}
	};

	let to_layer: usize = match s.next() {
		None => {
			println!("ERROR: read_link(): No To_layer Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_link(): Cannot Read To_layer");
				return None;
			}
		}
	};

	let to_index: usize = match s.next() {
		None => {
			println!("ERROR: read_link(): No To_Index Number Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: read_link(): Cannot Read To_Index");
				return None;
			}
		}
	};

	let weight: f32 = match s.next() {
		None => {
			println!("ERROR: read_link(): No Weight Found!");
			return None;
		},
		Some(s) => match s.parse() {
			Ok(num) => num,
			Err(e) => {
				println!("ERROR: Cannot Read Neuron Weight");
				return None;
			}
		}
	};

	Some(Link::new( (from_layer, from_index), (to_layer, to_index), weight))
}
