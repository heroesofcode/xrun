use crate::results::Results;
use crate::utils::Utils;
use colored::Colorize;

/// Represents the optional command-line flags that control xrun behavior.
enum ValidationArg {
	Fail,         // Exit with error code if tests fail
	GenerateFile, // Create PDF report of failures
	Other,        // Invalid argument
	None,         // No argument provided
}

impl ValidationArg {
	fn from_option(opt: Option<&str>) -> Self {
		match opt {
			Some("fail") => ValidationArg::Fail,
			Some("generate-file") => ValidationArg::GenerateFile,
			Some(_) => ValidationArg::Other,
			None => ValidationArg::None,
		}
	}
}

pub struct Validator;

impl Validator {
	/// Validates and processes optional command-line arguments (positions 6 and 7).
	/// Handles combinations of "fail" and "generate-file" flags.
	pub fn handle_validation_args(
		args: &[String],
		test_errors: &[(String, String)],
	) -> Result<(), String> {
		let arg6 = ValidationArg::from_option(args.get(6).map(String::as_str));
		let arg7 = ValidationArg::from_option(args.get(7).map(String::as_str));

		// If no errors, show success message and exit normally
		if test_errors.is_empty() {
			Results::validation_show_errors(test_errors.to_vec(), false);
			return Ok(());
		}

		// Handle different combinations of flags
		match (arg6, arg7) {
			// "fail" only: show errors and exit with error
			(ValidationArg::Fail, ValidationArg::None) => {
				Results::validation_show_errors(test_errors.to_vec(), false);
				Err("Tests failed".into())
			}
			// "fail" + "generate-file": create PDF and exit with error
			(ValidationArg::Fail, ValidationArg::GenerateFile) => {
				Utils::show_message_success_with_file(test_errors.to_vec());
				Err("Tests failed (file generated)".into())
			}
			// "generate-file" + "fail": invalid order
			(ValidationArg::GenerateFile, ValidationArg::Fail) => {
				Err("Argument error: conflict between fail and generate-file".into())
			}
			// Invalid argument combinations
			(ValidationArg::Fail, ValidationArg::Other) | (ValidationArg::Other, ValidationArg::None) => {
				Err("Argument error: invalid value".into())
			}
			// "generate-file" only: create PDF but don't fail
			(ValidationArg::GenerateFile, ValidationArg::None) => {
				Utils::show_message_success_with_file(test_errors.to_vec());
				Ok(())
			}
			// No flags: just show errors, don't fail
			(ValidationArg::None, ValidationArg::None) => {
				Results::validation_show_errors(test_errors.to_vec(), false);
				Ok(())
			}
			// Any other combination: show errors and fail
			_ => {
				Results::validation_show_errors(test_errors.to_vec(), false);
				Err("Invalid arguments".into())
			}
		}
	}

	/// Validates the first argument (extension type) and converts it to xcodebuild flag.
	/// Accepts "project" or "workspace" and returns the corresponding xcodebuild flag.
	pub fn validation_arg1(extension_type: &str) -> &str {
		match extension_type {
			"project" => "-project",
			"workspace" => "-workspace",
			_ => {
				eprintln!(
					"{}",
					"Error: First argument must be 'project' or 'workspace'".red()
				);
				std::process::exit(1)
			}
		}
	}
}
