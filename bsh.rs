mod style;
mod utils;

use std::{env, io::{self, Write}, process::Command};
use style::*;

fn main() {
	let mut shell = Shell {
		last_exit_code: 0,
	};

	loop {
		shell.print_prompt();

		// Get input
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		// Get command and arguments
		let mut args = input.trim().split_whitespace();
		let cmd = args.next().unwrap();

		shell.process_command(cmd, args.collect());
		if shell.last_exit_code == -999 {
			break;
		}
	}
}

struct Shell {
	last_exit_code: i32,
}

impl Shell {
	fn print_prompt(&self) {
		let color = if self.last_exit_code == 0 {GREEN} else {RED};
		let dir = utils::current_dir();

		print!(" {BOLD}{CYAN}{dir}{color} > {NORMAL}{REGULAR}");
		io::stdout().flush().unwrap();
	}

	fn process_command(&mut self, cmd: &str, args: Vec<&str>) {
		match cmd {
			// -999 Indicates that we want to break a loop
			"exit" => self.last_exit_code = -999,
			"cd" => {
				let dir = args
					.get(0)
					.unwrap_or(&utils::home_dir().as_str())
					.replace("~", &utils::home_dir());

				self.last_exit_code = 0;

				env::set_current_dir(&dir).unwrap_or_else(|error| {
					eprintln!("cd: {dir}: {error}");
					self.last_exit_code = 1;
				});
			},
			cmd => match Command::new(cmd).args(args).spawn() {
				Ok(mut child) => self.last_exit_code = child.wait().unwrap().code().unwrap_or(1),
				Err(error) => {
					eprintln!("{cmd}: {error}");
					self.last_exit_code = 1;
				},
			},
		};
	}
}
