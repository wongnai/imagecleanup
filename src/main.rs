#[macro_use]
extern crate log;
extern crate env_logger;
extern crate getopts;
extern crate docker;
extern crate imagecleanup;

use std::env;
use getopts::Options;
use docker::Docker;
use imagecleanup::*;

const DEFAULT_DOCKER_UNIX: &'static str = "/var/run/docker.sock";

fn usage(name: &str, options: &Options){
	let usage = format!("Usage: {} --numbered gcr.io/image --numbered-keep 1\n\nDefault to connect via unix socket", name);
	println!("{}", options.usage(&usage))
}

fn main(){
	env_logger::init().unwrap();

	let mut options = Options::new();
	options.optopt("H", "docker-tcp", "Docker address (TCP)", "http://localhost:2375/");
	options.optopt("S", "docker-unix", "Docker unix path", DEFAULT_DOCKER_UNIX);
	options.optflag("h", "help", "Print help menu");
	options.optopt("", "numbered", "Cleanup numbered image", "IMAGE_NAME");
	options.optopt("", "numbered-keep", "Number of numbered images to keep (default to 1)", "1");

	let args: Vec<String> = env::args().collect();
	let name = &args[0];
	let matches = match options.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => {
			println!("{}", f);
			usage(name, &options);
			return;
		}
    };
    if matches.opt_present("h") {
        usage(name, &options);
        return;
    }

	let docker = if matches.opt_present("H") {
		let url = matches.opt_str("H").unwrap();
		debug!("Connecting to Docker via TCP {}", url);
		Docker::connect_with_http(url).unwrap()
	} else {
		let unix_path = matches.opt_str("S").unwrap_or(DEFAULT_DOCKER_UNIX.to_string());
		debug!("Connecting to Docker via Unix {}", unix_path);
		Docker::connect_with_unix(unix_path).unwrap()
	};

	let cleanup = ImageCleanup::new(docker);
	let mut done_action = false;

	if matches.opt_present("numbered") {
		done_action = true;
		let keep_str = matches.opt_str("numbered-keep").unwrap_or("1".to_string());
		let keep = keep_str.parse::<usize>().expect("Number of containers to keep cannot be parsed");

		debug!("Using numbered strategy. Keeping {} containers", keep);

		for name in matches.opt_strs("numbered") {
			cleanup.cleanup(&NumberedCleanup::new(&name, keep)).expect("Removal failed");
		}
	}

	if !done_action {
		usage(name, &options);
	}
}
