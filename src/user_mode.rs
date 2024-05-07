use std::{env::{current_dir, current_exe}, fs::{self, metadata}, path::PathBuf};

use crate::filesystem::Filesystem;

pub fn setup() {
	if let Some(fs) = get_filesystem() {
		println!("Filesystem found, aborting setup");
		return;
	}
}

pub fn add_tags(mut arguments: Vec<String>) {

}

pub fn remove_tags(mut arguments: Vec<String>) {

}

pub fn remove_tags_all(mut arguments: Vec<String>) {
	
}
/// Gets your Filesystem, if returned None, it couldnt find it, or something went wrong when deserializing.
pub fn get_filesystem() -> Option<Filesystem> {
	let metadata_path = get_metadata_path();

	// check if metadata exists
	if let Err(feeez) = metadata(&metadata_path) {
		return None;
	}
	let tags_path = get_tags_path(&metadata_path);

	let tags = fs::read_to_string(tags_path).unwrap();
	match serde_json::from_str::<Filesystem>(&tags) {
		Ok(fs) => Some(fs),
		_ => None,
	}
}

fn get_metadata_path() -> PathBuf {
	let mut metadata_path: PathBuf = current_exe().unwrap();
	metadata_path.pop();
	metadata_path.push(".tags_meta");

	return metadata_path;
}

fn get_tags_path(metadata_path: &PathBuf) -> PathBuf{
	let tags_path = fs::read_to_string(metadata_path).unwrap();
	let tags_path = PathBuf::from(tags_path);

	return tags_path;
}

fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}