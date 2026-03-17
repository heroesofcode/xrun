use crate::validator::Validator;
use colored::Colorize;
use std::env;
use std::process::exit;

pub struct AppArgs {
	pub build_type: String,
	pub project_path: String,
	pub scheme: String,
	pub platform: String,
	pub device: Option<String>,
	pub raw: Vec<String>,
}

impl AppArgs {
	/// Parses and validates command-line arguments, exiting on invalid input.
	pub fn parse() -> Self {
		let args: Vec<String> = env::args().collect();

		const MIN_REQUIRED_ARGS: usize = 5;
		if args.len() < MIN_REQUIRED_ARGS {
			eprintln!("{}", "Commands not found".red());
			exit(1);
		}

		let build_type = Validator::validation_arg1(&args[1]).to_string();
		let project_path = args[2].clone();
		let scheme = args[3].clone();
		let platform = args[4].clone();

		let device = if args.len() > 5 {
			Some(args[5].clone())
		} else if platform != "macOS" {
			eprintln!(
				"{}",
				"Error: Device argument is required for iOS testing".red()
			);
			exit(1);
		} else {
			None
		};

		AppArgs {
			build_type,
			project_path,
			scheme,
			platform,
			device,
			raw: args,
		}
	}
}
