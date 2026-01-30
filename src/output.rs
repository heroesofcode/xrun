use crate::validation_lines::*;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct Output;

impl Output {
	pub fn get_output(
		arg1: &str,
		arg2: &String,
		arg3: &String,
		arg4: &String,
		arg5: Option<&String>,
	) -> Child {
		if arg4 == "macOS" {
			let output = Command::new("sh")
				.arg("-c")
				.arg(format!(
					"set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform={} \
                 clean test | xcpretty",
					arg1, arg2, arg3, arg4
				))
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("Failed to start command");

			output
		} else {
			let arg5_str = arg5.expect("arg5 is required for non-macOS platforms");

			let output = Command::new("sh")
				.arg("-c")
				.arg(format!(
					"set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
                 clean test | xcpretty",
					arg1, arg2, arg3, arg4, arg5_str
				))
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("Failed to start command");

			output
		}
	}

	fn extract_current_module(line: &str, current_module: &mut String) {
		if line.contains("Test Suite") && line.contains("started") {
			if let Some(start) = line.find("Test Suite") {
				if let Some(end) = line.find("started") {
					*current_module = line[start + 11..end].trim().to_string();

					if current_module.ends_with(".xctest") {
						current_module.truncate(current_module.len() - 7);
					}
				}
			}
		}
	}

	fn should_print_module(
		mutable_line: &str,
		current_module: &str,
		last_module_printed: &str,
	) -> bool {
		(!current_module.is_empty()
			&& (mutable_line.contains("Test Suite") && mutable_line.contains("started")
				|| mutable_line.trim() == current_module))
			&& current_module != last_module_printed
	}

	fn process_validation_line(
		mutable_line: &mut String,
		passed_tests: &mut u128,
		failed_tests: &mut u128,
		test_errors: &mut Vec<(String, String)>,
		current_module: &mut String,
	) {
		if mutable_line.trim() != current_module.as_str() {
			ValidationLine::validation_lines(
				mutable_line,
				passed_tests,
				failed_tests,
				test_errors,
				current_module,
			);
		}
	}

	pub fn process_output<R: std::io::Read>(
		reader: R,
		passed_tests: &mut u128,
		failed_tests: &mut u128,
		test_errors: &mut Vec<(String, String)>,
		current_module: &mut String,
	) {
		let reader = BufReader::new(reader);
		let mut last_module_printed = String::new();
		for line in reader.lines() {
			if let Ok(line) = line {
				let mut mutable_line = line.clone();

				Self::extract_current_module(&mutable_line, current_module);

				if Self::should_print_module(&mutable_line, current_module, &last_module_printed) {
					println!("\n\n{}", current_module.purple());
					last_module_printed = current_module.clone();
				}

				Self::process_validation_line(
					&mut mutable_line,
					passed_tests,
					failed_tests,
					test_errors,
					current_module,
				);
			}
		}
	}
}
