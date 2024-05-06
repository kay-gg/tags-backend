use std::{collections::HashMap, ffi::OsString, io::ErrorKind, path::{Path, PathBuf}};
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
	fn create_tag(&mut self, name: &str) -> Result<(), ErrorKind> {
		match self.tags.contains_key(name) {
			false => {
				let t: Tag = Tag::new();
				self.tags.insert(name.to_owned(), t);
				return Ok(());
			}
			true => Err(ErrorKind::AlreadyExists)
		}
	}
	/// Adds file to each tag in ```tags```
	/// 
	/// name is misleading because we arent actually adding tags to the files itself,
	/// but it sounds nicer this way. on the fence about changing
	fn add_tags_to_file(&mut self, mut tags_vec: Vec<String>) {
		// path is 0, for example
		// {path} {tag1} ... {tagn}
		let path = tags_vec.remove(0);

		for t in tags_vec {
			if let Some(tag) = self.tags.get_mut(&t) {
				tag.add_file(&path);
			} else {
				println!("Tag: {} not found.", t);
			}
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Tag {
	files: HashMap<OsString, PathBuf>,
}
impl Tag {
	/// Returns an empty Tag
	fn new() -> Tag {
		let t: Tag = Tag {files: HashMap::new()};
		return t;
	}
	
	fn add_file(&mut self, path: &str) {
		let absolute_path = PathBuf::from(path).canonicalize().unwrap();
		let filename = absolute_path.file_name().unwrap().to_owned();

		self.files.insert(filename, absolute_path);
	}
	
	fn remove_file(&mut self, path: &str) {
		let absolute_path = PathBuf::from(path).canonicalize().unwrap();
		let filename = absolute_path.file_name().unwrap().to_owned();

		let _ = self.files.remove(&filename);
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
		let _ = f.create_tag("test");
		let test = f.create_tag("test");

		assert_eq!(Err(ErrorKind::AlreadyExists), test);
	}
// again, misleading
	#[test]
	fn adding_tag_to_f() {
		let mut filesystem = Filesystem::new();
		let _ = filesystem.create_tag("test1");
		let _ = filesystem.create_tag("test2");
		let mut tag = filesystem.tags.remove("test2").unwrap();
		tag.add_file("./test");
		filesystem.tags.insert("test2".into(), tag);

		let mut test = Filesystem::new();
		let _ = test.create_tag("test1");
		let _ = test.create_tag("test2");
		test.add_tags_to_file(vec!["./test".to_string(), "test2".to_string()]);

		assert_eq!(filesystem, test);
	}
	#[test]
	fn adding_tags_to_f() {
		let mut filesystem = Filesystem::new();
		let _ = filesystem.create_tag("test1");
		let _ = filesystem.create_tag("test2");

		let mut tag = filesystem.tags.remove("test1").unwrap();
		tag.add_file("./test");
		filesystem.tags.insert("test1".into(), tag);

		let mut tag = filesystem.tags.remove("test2").unwrap();
		tag.add_file("./test");
		filesystem.tags.insert("test2".into(), tag);


		let mut test = Filesystem::new();
		let _ = test.create_tag("test1");
		let _ = test.create_tag("test2");
		test.add_tags_to_file(vec!["./test".to_string() ,"test1".to_string(), "test2".to_string()]);

		assert_eq!(filesystem, test);
	}
}

#[cfg(test)]
mod tag_tests{
	use super::*;

	#[test]
	fn empty_tag() {
		let empty_tag = Tag {files: HashMap::new()};

		assert_eq!(empty_tag.files.is_empty(), Tag::new().files.is_empty());
	}

	#[test]
	fn adding_file_to_tag() {
		let mut tag = Tag::new();
		let abs = PathBuf::from("./test/").canonicalize().unwrap();
		tag.files.insert(abs.file_name().unwrap().to_owned(), abs);

		let mut test = Tag::new();
		test.add_file("./test/");

		assert_eq!(tag, test);
	}

	#[test]
	fn adding_file_twice() {
		let mut tag = Tag::new();
		tag.add_file("./test");

		let mut test = Tag::new();
		test.add_file("./test");
		test.add_file("./test");

		assert_eq!(tag, test);
	}

	#[test]
	fn removing_file() {
		let mut tag = Tag::new();
		tag.add_file("./test");

		let mut test = Tag::new();
		test.add_file("./test");
		test.add_file("./test2");
		test.remove_file("./test2");

		assert_eq!(tag, test);
	}
	#[test]
	fn removing_file_twice() {
		let mut tag = Tag::new();
		tag.add_file("./test");

		let mut test = Tag::new();
		test.add_file("./test");
		test.add_file("./test2");

		test.remove_file("./test2");
		test.remove_file("./test2");

		assert_eq!(tag, test);
	}
}