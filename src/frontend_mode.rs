use crate::user_mode;

pub fn give_full_filesystem() {
	let fs = user_mode::get_filesystem().expect("Filesystem not found.");
	let fs = serde_json::to_string(&fs).expect("Error serializing fs into string");
	println!("{}", fs);
}