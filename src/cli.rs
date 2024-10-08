use crate::{frontend_mode, user_mode};

pub fn help() {
	println!("TODO!
	Print all of the args");
}

pub fn user_mode(mut arguments: Vec<String>) {
	match arguments.remove(0).as_str() {
		"-S" => user_mode::setup(),
		"-ct" => user_mode::create_tag(arguments),
		"-at" => user_mode::add_tags(arguments),
		"-rt" => user_mode::remove_tags(arguments),
		"-ut" => user_mode::untag(arguments),
		_ => unimplemented!(),
	};
}

pub fn frontend_mode(mut arguments: Vec<String>) {
	arguments.remove(0);

	if arguments.is_empty() {
		frontend_mode::give_full_filesystem();
		return;
	}
	match arguments.remove(0).as_str() {
		_ => unimplemented!(),
	};
}

