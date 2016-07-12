use std::io;
use docker::Docker;

use strategy::CleanupStrategy;

pub struct ImageCleanup{
	docker: Docker
}

impl ImageCleanup {
	pub fn new(docker: Docker) -> ImageCleanup{
		ImageCleanup {
			docker: docker
		}
	}

	pub fn cleanup(&self, strategy: &CleanupStrategy) -> Result<(), io::Error>{
		let images = try!(self.docker.get_images(false));
		let remove = try!(strategy.filter(images));

		for image in remove {
			for tag in image.RepoTags {
				info!("Remove {}", tag);
				let status = try!(self.docker.delete_image(tag, false, false));
				debug!("{:?}", status);
			}
		}
		Ok(())
	}
}
