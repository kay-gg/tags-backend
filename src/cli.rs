use crate::{frontend_mode, user_mode};

pub fn help() {
	println!("TODO!
	Print all of the args");
}

pub fn user_mode(mut arguments: Vec<String>) {
	arguments.remove(0);
	
	match arguments.remove(0).as_str() {

		"-S" => user_mode::setup(),
		"-at" => user_mode::add_tags(arguments),
		"-rt" => user_mode::remove_tags(arguments),
		"-rta" => user_mode::remove_tags_all(arguments),
		_ => unimplemented!(),
	}
}

pub fn frontend_mode(mut arguments: Vec<String>) {
	arguments.remove(0);

	if arguments.is_empty() {
		frontend_mode::give_full_filesystem();
		
	}
}

