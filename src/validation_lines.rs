use colored::Colorize;

pub struct ValidationLine {}

impl ValidationLine {
	/// Parses a test result line and updates test counters.
	/// Recognizes ✓ (pass) and ✗ (fail) markers from xcpretty output.
	pub fn validation_lines(
		line: &mut String,
		passed_tests: &mut u128,
		failed_tests: &mut u128,
		test_errors: &mut Vec<(String, String)>,
		current_module: &mut String,
	) {
		const PASS_MARKER: char = '✓';
		const FAIL_MARKER: char = '✗';

		let is_pass = line.contains(PASS_MARKER);
		let is_fail = line.contains(FAIL_MARKER);

		if is_pass {
			*passed_tests += 1;
			let cleaned = line.replace(PASS_MARKER, "").trim().to_string();
			println!("    ✅ {}", cleaned.green());
		} else if is_fail {
			*failed_tests += 1;
			let cleaned = line.replace(FAIL_MARKER, "").trim().to_string();
			println!("    ❌ {}", cleaned.red());
			test_errors.push((current_module.clone(), line.clone()));
		}
	}
}
