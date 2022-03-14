use rand::{thread_rng, Rng};
use std::fs::read_to_string;

pub struct Words(Vec<String>);
impl Words {
	pub fn new() -> Self {
		let words = if let Ok(s) = read_to_string("assets/words.txt") {
			s
		} else {
			eprintln!("assets/words.txt not found");
			std::process::exit(1);
		};
		let w_vec: Vec<String> = words.split("\n").collect::<Vec<&str>>().iter().map(|s| {
			s.to_string()
		}).collect();

		Self(w_vec)
	}

	pub fn get_new_word(&self) -> String {
		let mut rng = thread_rng();

		self.0.get(rng.gen_range(0..self.0.len())).unwrap().to_string()
	}
}
