use std::{collections::HashMap, io::ErrorKind, path::PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Filesystem {
	tags: HashMap<String, Tag>,
}
impl Filesystem {
	/// Creates a ```Filesystem``` with no ```Tag```s
	pub fn new() -> Filesystem {
		let h: HashMap<String, Tag> = HashMap::new();
		let f: Filesystem = Filesystem {tags: h};

		return f;
	}

	/// Adds a ```Tag``` with ```name``` to the ```Filesystem```
	pub fn create_tag(&mut self, name: &str) -> Result<(), ErrorKind> {
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
	/// path is 0, for example
	/// {path} {tag1} ... {tagn}
	pub fn add_tags_to_file(&mut self, mut tags_vec: Vec<String>) {
	// name is misleading because we arent actually adding tags to the files itself,
	// but it sounds nicer this way. on the fence about changing
		let path = tags_vec.remove(0);

		for t in tags_vec {
			if let Some(tag) = self.tags.get_mut(&t) {
				tag.add_file(&path);
			} else {
				println!("Tag: {} not found.", t);
			}
		}
	}
	/// Removes file to each tag in ```tags```
	/// 
	/// path is 0, for example
	/// {path} {tag1} ... {tagn}
	pub fn remove_tags_from_file(&mut self, mut tags_vec: Vec<String>) {
		let path = tags_vec.remove(0);

		for t in tags_vec {
			if let Some(tag) = self.tags.get_mut(&t) {
				tag.remove_file(&path);
			} else {
				println!("Tag: {} not found.", t);
			}
		}
	}
	/// Removes all tags from a file
	pub fn untag_file(&mut self, tags_vec: Vec<String>) {
		// this is ugly

		// for each tag
		for t in self.tags.iter_mut() {
			// if tag has key
			if t.1.files.contains_key(&Tag::get_filename(&tags_vec[0])) {
				t.1.remove_file(&tags_vec[0]);
			}
		}
	}
	/// Takes tags to filter by, ```tags_vec```,
	/// 
	/// Removes files from each tag that are not in all tags
	/// 
	/// Returns a Tag containing all of those files
	pub fn filter(&self, tags_vec: Vec<String>) -> Result<Tag, ErrorKind> {
		let mut intersection = Tag::new();
		let mut hashmap: HashMap<&String, i32> = HashMap::new();

		// confirm tags exist in Filesystem
		for tags in tags_vec.iter() {
			if let None = self.tags.get(tags) {
				return Err(ErrorKind::NotFound);
			}
		}
		// add to hashmap
		// hashmap contains <files, numbers of times files added>
		for tags in tags_vec.iter() {
			let tag_in_fs = self.tags.get(tags).unwrap();
			for files in tag_in_fs.files.iter() {
				if hashmap.contains_key(files.0) {
					let key = hashmap.get_mut(files.0).unwrap();
					*key += 1;
				} else {
					hashmap.insert(files.0, 1);
				}
			}
		}
		// if number of times = number of tags, they intersect on all tags
		for files in hashmap {
			if files.1 as usize == tags_vec.len() {
				intersection.add_file(files.0);
			}
		}

		return Ok(intersection);
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tag {
	files: HashMap<String, String>,
}
impl Tag {
	/// Returns an empty Tag
	fn new() -> Tag {
		let t: Tag = Tag {files: HashMap::new()};
		return t;
	}
	
	fn add_file(&mut self, path: &str) {
		let absolute_path = Tag::get_abs_path(&path);
		let filename = Tag::get_filename(&path);

		self.files.insert(filename, absolute_path);
	}
	
	fn remove_file(&mut self, path: &str) {
		let filename = Tag::get_filename(&path);

		let _ = self.files.remove(&filename);
	}

	// paths only work with / seperators.... might be a problem but is working for rn
	// might want to find a way to make this not use a PathBuf.display() bc it doesnt look good in tags file
	fn get_abs_path(path: &str) -> String {
		return PathBuf::from(path).canonicalize().unwrap().display().to_string();
	}
	pub fn get_filename(path: &str) -> String {
		let x: Vec<&str> = path.split("/").collect();
		let x = x.last().unwrap().to_owned();

		return String::from(x);
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
		let _ = test.create_tag("test");

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

	#[test]
	fn removing_tags_from_f() {
		let mut filesystem = Filesystem::new();
		let _ = filesystem.create_tag("test1");
		let _ = filesystem.create_tag("test2");
		// add test1 tag
		filesystem.add_tags_to_file(vec!["./test".to_string(), "test1".to_string()]);
	
		let mut test = Filesystem::new();
		let _ = test.create_tag("test1");
		let _ = test.create_tag("test2");
		let test_vec = vec!["./test".into(), "test1".to_string(), "test2".to_string()];
		let test_vec2 = vec!["./test".into(), "test2".to_string()];
		// add test1,test2 tags
		test.add_tags_to_file(test_vec);
		// remove test2 tag
		// left with test1 tag only.
		test.remove_tags_from_file(test_vec2);

		assert_eq!(filesystem, test);
	}

	#[test]
	fn removing_all_tags_from_f() {
		let mut filesystem = Filesystem::new();
		let _ = filesystem.create_tag("test1");
		let _ = filesystem.create_tag("test2");

		let mut test = Filesystem::new();
		let _ = test.create_tag("test1");
		let _ = test.create_tag("test2");
		test.add_tags_to_file(vec!["./test".into(), "test1".to_string(), "test2".to_string()]);
		test.untag_file(vec!["./test".into()]);

		assert_eq!(filesystem, test);
	}

	#[test]
	fn filtering_one_tag() {
		let mut tag = Tag::new();
		tag.add_file("./test");
		tag.add_file("./test2");

		let mut test = Filesystem::new();
		let _ = test.create_tag("tag1");

		let _ = test.add_tags_to_file(vec!["./test".into(), "tag1".into()]); 
		let _ = test.add_tags_to_file(vec!["./test2".into(), "tag1".into()]);
		
		let test_tag = test.filter(vec!["tag1".into()]).unwrap();

		assert_eq!(tag, test_tag);
	}

	/// tag1 = {test, 	test2}
	/// 
	/// tag2 = {test, 	_}
	/// 
	/// tag3 = {_, 		test2}
	/// 
	/// files that are in both tag1 and tag2 is test.
	#[test]
	fn filtering_twoplus_tags() {
		let mut tag = Tag::new();
		tag.add_file("./test");		


		let mut test = Filesystem::new();
		let _ = test.create_tag("tag1");
		let _ = test.create_tag("tag2");
		let _ = test.create_tag("tag3");

		let _ = test.add_tags_to_file(vec!["./test".into(), "tag1".into(), "tag2".into()]);
		let _ = test.add_tags_to_file(vec!["./test2".into(), "tag1".into(), "tag3".into()]);

		let test = test.filter(vec!["tag1".into(), "tag2".into()]).unwrap();

		assert_eq!(tag, test);
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
		let abs = PathBuf::from("./test").canonicalize().unwrap().display().to_string();
		tag.files.insert("test".into(), abs);

		let mut test = Tag::new();
		test.add_file("./test");

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