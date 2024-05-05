use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
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

	#[test]
	fn adding_tags() {
		let filesystem = Filesystem::new();
		filesystem.tags.insert("test".to_string(), Tag::new());
	}
}

#[cfg(test)]
mod tag_tests{
	use super::*;

	#[test]
	fn empty_tag() {

	}

}