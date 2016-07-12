use std::io::Error;
use docker::image::Image;

pub trait CleanupStrategy {
	fn filter(&self, Vec<Image>) -> Result<Vec<Image>, Error>;
}

pub struct NumberedCleanup<'a>{
	name: &'a String,
	num_to_keep: usize,
}

impl<'a> NumberedCleanup<'a> {
	pub fn new(name: &String, num_to_keep: usize) -> NumberedCleanup{
		NumberedCleanup {
			name: name,
			num_to_keep: num_to_keep,
		}
	}
}

impl<'a> CleanupStrategy for NumberedCleanup<'a> {
	fn filter(&self, images: Vec<Image>) -> Result<Vec<Image>, Error>{
		let mut result = Vec::new();

		'outer: for image in images {
			let mut tags = Vec::with_capacity(image.RepoTags.len());

			for tag in &image.RepoTags {
				let tag_parts: Vec<&str> = tag.split(":").collect();
				let name = tag_parts[0];
				let tag = tag_parts[1];

				if name != self.name {
					continue 'outer;
				}

				let tag_no = match tag.parse::<i64>() {
					Ok(x) => x,
					Err(_) => continue 'outer,
				};

				tags.push(tag_no);
			}

			if tags.len() == 0 {
				continue
			}

			result.push(ImageNumbered{
				image: image,
				number: *tags.iter().max().unwrap(),
			});
		}

		result.sort_by_key(|item| item.number);
		let len = result.len();
		if len < self.num_to_keep {
			// since len is usize, we can't compare to <0
			result.clear();
		} else {
			result.truncate(len - self.num_to_keep);
		}

		Ok(result.into_iter().map(|item| item.image).collect())
	}
}

struct ImageNumbered{
	image: Image,
	number: i64,
}
