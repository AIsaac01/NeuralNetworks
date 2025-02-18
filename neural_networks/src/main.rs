use neural_networks::gui::app::App;
use iced::Theme;

// entry point for project
fn main() {
	// Runs the GUI
	// Note: using the GUI is completely optional, neural network generation code can be run on its own
	let _ = iced::application("Neural Network App", App::update, App::view)
		.theme(|_| Theme::Dark)
        .centered()
        .run();
}
