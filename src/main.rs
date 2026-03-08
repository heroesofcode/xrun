use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;

mod header;
mod output;
mod results;
mod utils;
mod validation_lines;
mod validator;

use crate::header::Header;
use crate::output::Output;
use crate::results::Results;
use crate::validator::Validator;

fn main() {
	Header::show_header();

	let start_time = Instant::now();
	let args: Vec<String> = env::args().collect();

	const MIN_REQUIRED_ARGS: usize = 5;

	if args.len() < MIN_REQUIRED_ARGS {
		eprintln!("{}", "Commands not found".red());
		exit(1);
	}

	// Parse command-line arguments: "project" or "workspace"
	let extension_type = &args[1];
	let project_path = &args[2];
	let scheme = &args[3];
	// "macOS" or iOS version like "17.4"
	let platform = &args[4];

	let build_type = Validator::validation_arg1(extension_type);

	let device = if args.len() > 5 {
		Some(&args[5])
	} else if platform != "macOS" {
		eprintln!(
			"{}",
			"Error: Device argument is required for iOS testing".red()
		);
		exit(1);
	} else {
		None
	};

	let mut passed = 0u128;
	let mut failed = 0u128;
	let mut current_module = String::new();
	let mut errors = Vec::new();
	let mut failed_any = false;

	run_with_spinner("Running xcodebuild...", || {
		let output = Output::get_output(build_type, project_path, scheme, platform, device);

		let output_result = output
			.wait_with_output()
			.expect("Failed to wait for xcodebuild process");

		if !output_result.stdout.is_empty() {
			use std::io::Cursor;
			let cursor = Cursor::new(output_result.stdout.clone());

			Output::process_output(
				cursor,
				&mut passed,
				&mut failed,
				&mut errors,
				&mut current_module,
			);
		}

		if !output_result.status.success() {
			failed_any = true;
		}
	});

	Results::show_results(start_time, passed, failed);
	match Validator::handle_validation_args(&args, &errors) {
		Ok(()) => {}
		Err(e) => {
			eprintln!("{}", e.red());
			exit(1);
		}
	}

	if failed_any {
		exit(1);
	}
}

fn run_with_spinner<F: FnOnce()>(_message: &str, job: F) {
	let progress_bar = ProgressBar::new_spinner();
	progress_bar.set_style(
		ProgressStyle::default_spinner()
			.template(&format!(
				"{{spinner:.blue}} {} [{{elapsed_precise}}]",
				_message
			))
			.expect("Failed to set progress bar template"),
	);
	progress_bar.enable_steady_tick(Duration::from_millis(100));
	job();
	progress_bar.finish_and_clear();
}
