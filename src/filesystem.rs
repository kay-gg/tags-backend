use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Filesystem {
	tags: HashMap<String, Tag>,
}
impl Filesystem {
	/// Creates a ```Filesystem``` with no ```Tag```s
	fn new() -> Filesystem {
		let h: HashMap<String, Tag> = HashMap::new();
		let f: Filesystem = Filesystem {tags: h};

		return f;
	}

	/// Adds a ```Tag``` with ```name``` to the ```Filesystem```
	fn create_tag(&mut self, name: &str) {
		let t: Tag = Tag::new();
		self.tags.insert(name.to_owned(), t);
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Tag {
	files: Vec<String>,
}
impl Tag {
	/// Returns an empty Tag
	fn new() -> Tag {
		let t = Tag {files: Vec::new()};
		return t;
	}
	
	fn add_file(&mut self, path: &str) {
		self.files.push(path.to_string());
	}
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

	#[test]
	fn creating_tags() {
		let mut filesystem = Filesystem::new();
		filesystem.tags.insert("test".to_string(), Tag::new());

		let mut test = Filesystem::new();
		test.create_tag("test");

		assert_eq!(filesystem, test);
	}
	#[test]
	fn creating_two_same_tags() {
		let mut f = Filesystem::new();
		f.create_tag("test");
	}
}

#[cfg(test)]
mod tag_tests{
	use super::*;

	#[test]
	fn empty_tag() {
		let empty_tag = Tag {files: Vec::new()};

		assert_eq!(empty_tag.files.is_empty(), Tag::new().files.is_empty());
	}

	#[test]
	fn adding_files_to_tag() {
		let mut tag = Tag::new();
		tag.files.push("test".to_string());

		let mut test = Tag::new();
		test.add_file("test");
		
		assert_eq!(tag, test);
	}
}