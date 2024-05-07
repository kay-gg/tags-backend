use std::{env::{current_dir, current_exe}, fs::{self, metadata}, io::{self, Read}, path::PathBuf};

use crate::filesystem::Filesystem;

pub fn setup() {
	if let Some(_fs) = get_filesystem() {
		println!("Filesystem found, aborting setup");
		return;
	}

	println!("Path to store tags at:");
	let mut tags_path = String::new();
	match io::stdin().read_line(&mut tags_path) {
		Ok(okthumbsup) => {},
		Err(e) => panic!("Error reading input"),
	}

	let mut tags_pathbuf = PathBuf::from(&tags_path.trim());
	if tags_pathbuf.is_dir() {
		println!("{}", colored(255, 0, 0, "ERROR: path is a directory. Nothing written."));
		return;
	}
	//tags_pathbuf.push("tags");
	let meta_path: PathBuf = {
		let mut x: PathBuf = current_exe().unwrap();
		x.pop();
		x.push(".tags_meta"); 
		x
	};

	fs::write(&meta_path, tags_path).expect("error writing .tags_meta");
	println!("{} {}", colored(36, 140, 54, ".tags_meta written to:"), meta_path.display());

	let mut fs = Filesystem::new();
	// testing. remove __________________________________________________________________________________________
	fs.create_tag("art");
	fs.create_tag("videos");
	fs.create_tag("music");

	// this is crashing bc of OsString and PathBuf arent strings/convertable.
	// fs.add_tags_to_file(vec!["./test".to_string(), "art".to_string()]);
	// fs.add_tags_to_file(vec!["./test2".to_string(), "music".to_string(), "videos".to_string()]);
	// __________________________________________________________________________________________________________
	let serialized = serde_json::to_string(&fs).unwrap();
	if let Ok(()) = fs::write(&tags_pathbuf, serialized) {
		println!("{} {}", colored(36, 140, 54, "Filesystem written to"), tags_pathbuf.display());
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

	let tags = fs::read_to_string(tags_path);
	if let Err(error) = tags {
		return None;
	}
	let tags = tags.unwrap();
	match serde_json::from_str::<Filesystem>(&tags) {
		Ok(fs) => Some(fs),
		_ => None,
	}
}

/// Metadata must be stored in the same folder as the exe
fn get_metadata_path() -> PathBuf {
	let mut metadata_path: PathBuf = current_exe().unwrap();
	metadata_path.pop();
	metadata_path.push(".tags_meta");

	return metadata_path;
}

/// tags path is stored in the .tags_meta file
fn get_tags_path(metadata_path: &PathBuf) -> PathBuf{
	let tags_path = fs::read_to_string(metadata_path).unwrap();
	let tags_path = PathBuf::from(tags_path);

	return tags_path;
}

fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}