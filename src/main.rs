use std::env::args;

mod cli;
mod frontend_mode;
mod user_mode;

fn main() {
	handle_args(args().collect());
}

fn handle_args(mut arguments: Vec<String>) {
	// on windows, the first argument is the path to the executable
	if std::env::consts::OS == "windows" {
		arguments.remove(0);
	}
	if std::env::consts::OS == "linux" {
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