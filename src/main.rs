use std::{collections::HashMap, env::args};
use serde::{Serialize, Deserialize};

mod cli;
mod frontend_mode;
mod user_mode;

fn main() {
	handle_args(args().collect());
}

fn handle_args(mut arguments: Vec<String>) {
	// on windows, the first argument is the path to the executable
	// on linux, i believe it doesnt do this, dont know about other OS
	if std::env::consts::OS == "windows" {
		arguments.remove(0);
	}

	if arguments.is_empty() {
		// help 
		cli::help();
	} else {
		if arguments.get(0).unwrap() == "-F" {
			cli::frontend_mode(arguments);
		} else {
			cli::user_mode(arguments);	
		}
	}
}

#[derive(Serialize, Deserialize)]
struct Filesystem {
	tags: HashMap<String, Tag>,
}

impl Filesystem {
	/// Creates a Filesystem with no Tags
	/// 
	/// Used for setup only
	fn new() -> Filesystem {
		let h: HashMap<String, Tag> = HashMap::new();
		let f = Filesystem {tags: h};

		return f;
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
	files: Vec<String>,
}

#[cfg(test)]
mod filesystem_tests {
	use super::*;

	#[test]
	fn empty_filesystem() {
		let h: HashMap<String, Tag> = HashMap::new();
		let empty = Filesystem {tags: h};

		assert_eq!(empty.tags.is_empty(), Filesystem::new().tags.is_empty());
	}
}