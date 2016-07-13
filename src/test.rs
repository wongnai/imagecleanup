use strategy::*;
use docker::image::Image;

fn image(id: &str) -> Image {
	Image{
		Created: 0,
		Id: id.to_string(),
		ParentId: "mock".to_string(),
		RepoTags: Vec::new(),
		Size: 0,
		VirtualSize: 0,
	}
}

fn image_tag(id: &str, tags: Vec<&str>) -> Image {
	let mut out = image(id);
	out.RepoTags = tags.into_iter().map(|x| x.to_string()).collect();
	out
}

#[test]
fn numbered() {
	let numbered = NumberedCleanup::new("test", 3);
	let result = numbered.filter(vec![
		image_tag("m1", vec!["test:1"]),
		image_tag("m2", vec!["test:2"]),
		image_tag("m3", vec!["test:3"]),
		image_tag("m4", vec!["test:4"]),
		image_tag("m5", vec!["test:5"]),
	]).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(result[0].Id, "m1");
	assert_eq!(result[1].Id, "m2");
}

#[test]
fn numbered_multiple() {
	let numbered = NumberedCleanup::new("test", 1);
	let result = numbered.filter(vec![
		image_tag("m1", vec!["test:1"]),
		image_tag("m2", vec!["test:2", "test:5"]),
		image_tag("m3", vec!["test:3"]),
	]).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(result[0].Id, "m1");
	assert_eq!(result[1].Id, "m3");
}

#[test]
fn numbered_keep_string() {
	let numbered = NumberedCleanup::new("test", 0);
	let result = numbered.filter(vec![
		image_tag("m1", vec!["test:1", "test:keep"]),
	]).unwrap();
	assert_eq!(result.len(), 0);
}

#[test]
fn numbered_other_tag() {
	let numbered = NumberedCleanup::new("test", 0);
	let result = numbered.filter(vec![
		image_tag("m1", vec!["nottest:1"]),
	]).unwrap();
	assert_eq!(result.len(), 0);
}
