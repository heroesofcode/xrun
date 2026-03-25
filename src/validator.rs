use crate::results::Results;
use crate::utils::Utils;

pub struct Validator;

impl Validator {
	pub fn handle_validation_args(
		fail: bool,
		generate_file: bool,
		test_errors: &[(String, String)],
	) -> Result<(), String> {
		if test_errors.is_empty() {
			Results::validation_show_errors(test_errors.to_vec(), false);
			return Ok(());
		}

		match (fail, generate_file) {
			(true, true) => {
				Utils::show_message_success_with_file(test_errors.to_vec());
				Err("Tests failed (file generated)".into())
			}
			(true, false) => {
				Results::validation_show_errors(test_errors.to_vec(), false);
				Err("Tests failed".into())
			}
			(false, true) => {
				Utils::show_message_success_with_file(test_errors.to_vec());
				Ok(())
			}
			(false, false) => {
				Results::validation_show_errors(test_errors.to_vec(), false);
				Ok(())
			}
		}
	}
}
