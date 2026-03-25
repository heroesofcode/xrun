use crate::validation_lines::*;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct Output;

impl Output {
	/// Spawns an xcodebuild process with xcpretty formatting.
	/// Returns a Child process handle that can be waited on.
	///
	/// # Arguments
	/// * `build_flag` - "-project" or "-workspace"
	/// * `project_path` - Path to .xcodeproj or .xcworkspace
	/// * `scheme` - Xcode scheme name
	/// * `platform` - "macOS" or iOS version
	/// * `device` - iPhone model number (required for iOS, e.g. `15` → `name=iPhone 15`)
	pub fn get_output(
		build_flag: &str,
		project_path: &String,
		scheme: &String,
		platform: &String,
		device: Option<&String>,
	) -> Child {
		if platform == "macOS" {
			Command::new("sh")
				.arg("-c")
				.arg(format!(
					"set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform={} \
                 clean test | xcpretty",
					build_flag, project_path, scheme, platform
				))
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("Failed to start command")
		} else {
			let device_version = device.expect("Device version is required for iOS testing");

			Command::new("sh")
				.arg("-c")
				.arg(format!(
					"set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
                 clean test | xcpretty",
					build_flag, project_path, scheme, platform, device_version
				))
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("Failed to start command")
		}
	}

	/// Extracts the test module name from xcodebuild output lines.
	/// Updates current_module when a "Test Suite ... started" line is found.
	fn extract_current_module(line: &str, current_module: &mut String) {
		if line.contains("Test Suite")
			&& line.contains("started")
			&& let Some(start) = line.find("Test Suite")
			&& let Some(end) = line.find("started")
		{
			const TEST_SUITE_PREFIX_LEN: usize = 11; // Length of "Test Suite "
			*current_module = line[start + TEST_SUITE_PREFIX_LEN..end].trim().to_string();

			const XCTEST_EXTENSION: &str = ".xctest";
			if current_module.ends_with(XCTEST_EXTENSION) {
				current_module.truncate(current_module.len() - XCTEST_EXTENSION.len());
			}
		}
	}

	/// Determines if the current module name should be printed to the console.
	/// Avoids duplicate module headers in the output.
	fn should_print_module(line: &str, current_module: &str, last_module_printed: &str) -> bool {
		let is_module_line = !current_module.is_empty()
			&& (line.contains("Test Suite") && line.contains("started") || line.trim() == current_module);
		let not_already_printed = current_module != last_module_printed;

		is_module_line && not_already_printed
	}

	/// Processes a single line of xcodebuild output to detect test results.
	/// Updates test counters and error collection based on pass/fail markers.
	fn process_validation_line(
		line: &mut String,
		passed_tests: &mut u128,
		failed_tests: &mut u128,
		test_errors: &mut Vec<(String, String)>,
		current_module: &mut String,
	) {
		// Skip lines that are just the module name itself
		if line.trim() != current_module.as_str() {
			ValidationLine::validation_lines(
				line,
				passed_tests,
				failed_tests,
				test_errors,
				current_module,
			);
		}
	}

	/// Processes the complete xcodebuild output stream line by line.
	/// Extracts test results, module names, and formats console output.
	pub fn process_output<R: std::io::Read>(
		reader: R,
		passed_tests: &mut u128,
		failed_tests: &mut u128,
		test_errors: &mut Vec<(String, String)>,
		current_module: &mut String,
	) {
		let reader = BufReader::new(reader);
		let mut last_module_printed = String::new();
		for line in reader.lines().flatten() {
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
