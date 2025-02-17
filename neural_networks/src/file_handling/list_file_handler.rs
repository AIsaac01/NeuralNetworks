use std::fs;
use std::fs::File;
use std::io::Write;

// literally copied and pasted this from nnd_exists(), I'm not ashamed
pub fn file_exists(filepath: &str) -> bool {
	let exists = File::open(filepath);
	match exists {
		Ok(_f) => return true,
		Err(_e) => return false
	}
}

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
