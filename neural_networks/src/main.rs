mod gui;
mod math;
mod neural_network;
mod nnd;

use gui::main_menu::*;
use math::functions;
use neural_network::network::*;
use nnd::nnd_file_handler::*;

use iced::Theme;

fn main() {
	// let path: &str = r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\nnd_files\my_network_2.nnd";
	// let n: Network = read_nnd(path).unwrap();
	// //n.print_neuron_vals();
	// write_nnd(r"C:\Users\Ayham\Programming\NeuralNetworks\NeuralNetworks\nnd_files\my_network.nnd", n);
	let _ = iced::application("Neural Network App", App::update, App::view)
		.theme(|_| Theme::Dark)
        .centered()
        .run();

	//
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
	// let inputs: Vec<f32> = vec![1.0, 0.0];
	// my_net.attach_inputs(inputs);
	// let expected_output: Vec<f32> = vec!(1.0);
	//
	// for _ in 0..100 {
	// 	my_net.forward_prop();
	// 	my_net.back_prop(expected_output.clone());
	// }
	// let w1 = my_net.save_weights();
	// // my_net.print_neuron_vals();
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
