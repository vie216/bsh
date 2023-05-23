mod style;
mod utils;

use std::{
	collections::HashMap,
	env,
	fs::read_to_string,
	io::{self, Write as _},
	process::Command
};
use style::*;

fn main() {
	let mut shell = Shell {
		last_exit_code: 0,
		aliases: HashMap::new(),
	};

	let run_commands = format!("{}/.bshrc", utils::home_dir());
	if let Ok(data) = read_to_string(run_commands) {
		for line in data.lines() {
			if let Some((cmd, args)) = parse_command(line) {
				shell.process_command(cmd, args);
			}
		}
	}

	loop {
		shell.print_prompt();

		// Get input
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		if let Some((cmd, args)) = parse_command(&input) {
			shell.process_command(cmd, args);
			if shell.last_exit_code == -999 {
				break;
			}
		}
	}
}

fn parse_command(cmd: &str) -> Option<(String, Vec<String>)> {
	let mut args = cmd
		.trim()
		.split_whitespace()
		.filter(|s| !s.is_empty())
		.map(|s| s.to_string());

	if let Some(cmd) = args.next() {
		return Some((cmd, args.collect()));
	}

	None
}

struct Shell {
	last_exit_code: i32,
	aliases: HashMap<String, String>,
}

impl Shell {
	fn print_prompt(&self) {
		let color = if self.last_exit_code == 0 {GREEN} else {RED};
		let dir = utils::current_dir();

		print!(" {BOLD}{CYAN}{dir}{color} > {NORMAL}{REGULAR}");
		io::stdout().flush().unwrap();
	}

	fn process_command(&mut self, mut cmd: String, args: Vec<String>) {
		if let Some(alias) = self.aliases.get(&cmd) {
			cmd = alias.clone();
		}

		match cmd.as_str() {
			// -999 Indicates that we want to break a loop
			"exit" => self.last_exit_code = -999,
			"cd" => {
				let dir = args
					.get(0)
					.unwrap_or(&utils::home_dir())
					.replace("~", &utils::home_dir());

				self.last_exit_code = 0;

				env::set_current_dir(&dir).unwrap_or_else(|error| {
					eprintln!("cd: {dir}: {error}");
					self.last_exit_code = 1;
				});
			},
			"alias" => {
				if args.len() != 2 {
					eprintln!("alias: wrong arguments count: expected 2, found {}", args.len());
					self.last_exit_code = 1;
					return;
				}

				self.aliases.insert(args[0].to_string(), args[1].to_string());
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
