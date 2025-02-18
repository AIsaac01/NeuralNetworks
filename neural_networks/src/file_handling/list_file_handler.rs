use std::fs;
use std::fs::File;
use std::io::Write;

// checks to see if file exists
pub fn file_exists(filepath: &str) -> bool {
	let exists = File::open(filepath);
	match exists {
		Ok(_f) => return true,
		Err(_e) => return false
	}
}

// creates a list file from a given filepath and vector of f32's
pub fn create_list_file(filepath: &str, list: Vec<f32>) {
	let mut file = match File::create_new(filepath) {
		Err(_e) => {
			println!("ERROR: write_list_file(): file already exists!");
			return;
		}
		Ok(f) => f
	};

	let mut output = String::new();

	for num in list.iter() {
		output.push_str( num.to_string().as_str() );
		output.push('\n');
	}
	file.write_all( output.as_bytes() ).unwrap();

}

// Reads list file, returns None for Error (NOTE: Replace Option with Result)
pub fn read_list_file(filepath: &str) -> Option<Vec<f32>> {
	let contents = match fs::read_to_string(filepath) {
		Ok(c) => c,
        Err(_e) => {
			println!("ERROR: Invalid List File Path!");
			return None;
		},
	};

	let mut list: Vec<f32> = Vec::new();
	for num in contents.lines() {
		match num.parse() {
			Err(_e) => {
				println!("ERROR READING {}", filepath);
				return None;
			}
			Ok(n) => list.push(n),
		}
	}
	Some(list)
}
