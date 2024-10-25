mod gui;
mod math;
mod neural_network;
mod file_handling;

use gui::app::App;
use gui::main_menu::*;
use math::functions;
use neural_network::network::*;
use file_handling::nnd_file_handler::*;
use file_handling::list_file_handler::*;

use std::fs;
use std::fs::File;
use std::io::Write;

use iced::Theme;

fn main() {
	// let path: &str = r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\neural_networks\inputs\test";
	// let path_out: &str = r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\neural_networks\expected_outputs\test_output";
	// let inp: Vec<f32> = vec![0.0, 1.0];
	// let out: Vec<f32> = vec![1.0];
	// create_list_file(path, inp);
	// create_list_file(path_out, out);
	// let n: Vec<f32> = read_list_file(path).unwrap();
	// println!("{:?}", n);
	// n.print_neuron_vals();

	let _ = iced::application("Neural Network App", App::update, App::view)
		.theme(|_| Theme::Dark)
        .centered()
        .run();
	// //
	// // read inputs for 3
	// let contents = match fs::read_to_string(r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\neural_networks\inputs\three") {
	// 	Ok(c) => c,
    //     Err(e) => {
	// 		println!("ERROR: Invalid File Path!");
	// 		return;
	// 	},
	// };
	//
	// let mut inputs: Vec<f32> = Vec::new();
	// for num in contents.split(" ") {
	// 	// let n = num.parse();
	// 	match num.parse::<u8>() {
	// 		Err(err) => (),
	// 		Ok(num) => inputs.push( num as f32 )
	// 	};
	// }
	//
	// // read inputs and outputs for 3
	// let contents = match fs::read_to_string(r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\neural_networks\expected_outputs\three") {
	// 	Ok(c) => c,
    //     Err(e) => {
	// 		println!("ERROR: Invalid File Path!");
	// 		return;
	// 	},
	// };
	//
	// let mut expected_outputs: Vec<f32> = Vec::new();
	// for num in contents.split(" ") {
	// 	match num.parse::<u8>() {
	// 		Err(err) => (),
	// 		Ok(num) => expected_outputs.push( num as f32 )
	// 	};
	// }
	//
	// let mut network = read_nnd(r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\nnd_files\three.nnd").unwrap();
	// //network.print_neuron_vals();
	// network.attach_inputs(inputs);
	// network.set_learning_rate(0.05);
	// //
	// for epoch in 0..10 {
	// 	network.forward_prop();
	// 	network.back_prop(expected_outputs.clone());
	// 	println!("Finished Epoch {}!", epoch);
	// }
	//
	// network.forward_prop();
	// network.print_output_vals();
	// // initialize network
	// let mut my_net = Network::new(0.05);
	//
	// // create input layer
	// let mut input_layer: Vec<Neuron> = Vec::new();
	// input_layer.push( Neuron::new(0, 0, None, None) );
	// input_layer.push( Neuron::new(0, 1, None, None) );
	//
	// // create first hidden layer
	// let mut hidden_layer_one: Vec<Neuron> = Vec::new();
	// hidden_layer_one.push( Neuron::new(1, 0, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_one.push( Neuron::new(1, 1, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_one.push( Neuron::new(1, 2, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_one.push( Neuron::new(1, 3, Some(ActivationFunction::ReLU), None) );
	//
	// // create second hidden layer
	// let mut hidden_layer_two: Vec<Neuron> = Vec::new();
	// hidden_layer_two.push( Neuron::new(2, 0, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_two.push( Neuron::new(2, 1, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_two.push( Neuron::new(2, 2, Some(ActivationFunction::ReLU), None) );
	// hidden_layer_two.push( Neuron::new(2, 3, Some(ActivationFunction::ReLU), None) );
	//
	// // create output layer
	// let mut output_layer: Vec<Neuron> = Vec::new();
	// output_layer.push( Neuron::new(3, 0, Some(ActivationFunction::ReLU), Some(LossFunction::MeanSquareError)) );
	//
	// // attack all layers to network
	// my_net.add_layer(input_layer);
	// my_net.add_layer(hidden_layer_one);
	// my_net.add_layer(hidden_layer_two);
	// my_net.add_layer(output_layer);
	//
	// // inialize list of connections
	// let mut links: Vec<Link> = Vec::new();
	//
	// // inputs to hidden one
	// my_net.add_link( Link::new( (0,0), (1,0), 0.5) );
	// my_net.add_link( Link::new( (0,0), (1,1), 0.5) );
	// my_net.add_link( Link::new( (0,0), (1,2), 0.5) );
	// my_net.add_link( Link::new( (0,0), (1,3), 0.5) );
	//
	// my_net.add_link( Link::new( (0,1), (1,0), 0.5) );
	// my_net.add_link( Link::new( (0,1), (1,1), 0.5) );
	// my_net.add_link( Link::new( (0,1), (1,2), 0.5) );
	// my_net.add_link( Link::new( (0,1), (1,3), 0.5) );
	//
	// // hidden one to hidden two
	// my_net.add_link( Link::new( (1,0), (2,0), 0.5) );
	// my_net.add_link( Link::new( (1,0), (2,1), 0.5) );
	// my_net.add_link( Link::new( (1,0), (2,2), 0.5) );
	// my_net.add_link( Link::new( (1,0), (2,3), 0.5) );
	//
	// my_net.add_link( Link::new( (1,1), (2,0), 0.5) );
	// my_net.add_link( Link::new( (1,1), (2,1), 0.5) );
	// my_net.add_link( Link::new( (1,1), (2,2), 0.5) );
	// my_net.add_link( Link::new( (1,1), (2,3), 0.5) );
	//
	// my_net.add_link( Link::new( (1,2), (2,0), 0.5) );
	// my_net.add_link( Link::new( (1,2), (2,1), 0.5) );
	// my_net.add_link( Link::new( (1,2), (2,2), 0.5) );
	// my_net.add_link( Link::new( (1,2), (2,3), 0.5) );
	//
	// my_net.add_link( Link::new( (1,3), (2,0), 0.5) );
	// my_net.add_link( Link::new( (1,3), (2,1), 0.5) );
	// my_net.add_link( Link::new( (1,3), (2,2), 0.5) );
	// my_net.add_link( Link::new( (1,3), (2,3), 0.5) );
	//
	// // hidden two to output
	// my_net.add_link( Link::new( (2,0), (3,0), 0.5) );
	// my_net.add_link( Link::new( (2,1), (3,0), 0.5) );
	// my_net.add_link( Link::new( (2,2), (3,0), 0.5) );
	// my_net.add_link( Link::new( (2,3), (3,0), 0.5) );
	//
	// // train to do XOR
	// // 1, 0 -> 1
	// let inputs: Vec<f32> = vec![0.0, 1.0];
	// my_net.attach_inputs(inputs);
	// let expected_output: Vec<f32> = vec!(1.0);
	//
	// my_net.forward_prop();
	// my_net.print_neuron_vals();
	//
	// for _ in 0..100000 {
	// 	my_net.forward_prop();
	// 	my_net.back_prop(expected_output.clone());
	// }
	// let w1 = my_net.save_weights();
	// my_net.print_neuron_vals();
	//
	//
	// // 0, 1 -> 1
	// let inputs: Vec<f32> = vec![0.0, 1.0];
	// my_net.attach_inputs(inputs);
	// let expected_output: Vec<f32> = vec!(1.0);
	//
	// for _ in 0..100 {
	// 	my_net.forward_prop();
	// 	my_net.back_prop(expected_output.clone());
	// }
	// let w2 = my_net.save_weights();
	// // my_net.print_neuron_vals();
	//
	//
	// // 0, 0 -> 0
	// let inputs: Vec<f32> = vec![0.0, 0.0];
	// my_net.attach_inputs(inputs);
	// let expected_output: Vec<f32> = vec!(0.0);
	//
	// for _ in 0..100 {
	// 	my_net.forward_prop();
	// 	my_net.back_prop(expected_output.clone());
	// }
	// let w3 = my_net.save_weights();
	// // my_net.print_neuron_vals();
	//
	//
	// // 1, 1 -> 0
	// // 0, 0 -> 0
	// let inputs: Vec<f32> = vec![1.0, 1.0];
	// my_net.attach_inputs(inputs);
	// let expected_output: Vec<f32> = vec!(0.0);
	//
	// for _ in 0..100 {
	// 	my_net.forward_prop();
	// 	my_net.back_prop(expected_output.clone());
	// }
	// let w4 = my_net.save_weights();
	// // my_net.print_neuron_vals();
	//
	// let mut final_weights: Vec<f32> = Vec::new();
	//
	// for i in 0..w1.len() {
	// 	final_weights.push( (w1[i] + w2[i] + w3[i] + w4[i]) / 4.0 );
	// }
	// my_net.attach_weights(final_weights);
	//
	// let inputs: Vec<f32> = vec![0.0, 0.0];
	// my_net.attach_inputs(inputs);
	// my_net.forward_prop();
	// my_net.print_neuron_vals();
	// let inputs: Vec<f32> = vec![1.0, 0.0];
	// my_net.attach_inputs(inputs);
	// my_net.forward_prop();
	// my_net.print_neuron_vals();
	// let inputs: Vec<f32> = vec![0.0, 1.0];
	// my_net.attach_inputs(inputs);
	// my_net.forward_prop();
	// my_net.print_neuron_vals();
	// let inputs: Vec<f32> = vec![1.0, 1.0];
	// my_net.attach_inputs(inputs);
	// my_net.forward_prop();
	// my_net.print_neuron_vals();

}
