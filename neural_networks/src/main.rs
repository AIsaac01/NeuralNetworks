use neural_networks::gui::app::App;
use iced::Theme;

fn main() {
	let _ = iced::application("Neural Network App", App::update, App::view)
		.theme(|_| Theme::Dark)
        .centered()
        .run();
}
