use colored::Colorize;
use std::process::exit;
use std::time::Instant;

mod args;
mod header;
mod output;
mod pdf_report;
mod results;
mod spinner;
mod text_utils;
mod utils;
mod validation_lines;
mod validator;

use crate::args::AppArgs;
use crate::header::Header;
use crate::output::Output;
use crate::results::Results;
use crate::validator::Validator;

fn run() {
	let start_time = Instant::now();
	let app_args = AppArgs::parse();

	let mut passed = 0u128;
	let mut failed = 0u128;
	let mut current_module = String::new();
	let mut errors = Vec::new();
	let mut failed_any = false;

	spinner::run_with_spinner("Running xcodebuild...", || {
		let output = Output::get_output(
			&app_args.build_type,
			&app_args.project_path,
			&app_args.scheme,
			&app_args.platform,
			app_args.device.as_ref(),
		);

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
	match Validator::handle_validation_args(&app_args.raw, &errors) {
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

fn main() {
	Header::show_header();
	run();
}
