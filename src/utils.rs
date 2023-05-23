use std::env;

pub fn home_dir() -> String {
	env::var("HOME").unwrap()
}

pub fn current_dir() -> String {
	let current_dir = env::current_dir()
		.unwrap()
		.into_os_string()
		.into_string()
		.unwrap()
		.replace(&home_dir(), "~");

	current_dir
}
