use crate::user_mode;

pub fn give_full_filesystem() {
	todo!();
}


pub fn filter_tags(arguments: Vec<String>) {
	let fs = user_mode::get_filesystem().expect("Filesystem not found.");
	if let Ok(tag) = fs.filter(arguments) {
		// output deserialized tag to stdout
	} else {
		// error
	}
}