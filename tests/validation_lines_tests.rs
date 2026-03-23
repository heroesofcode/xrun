use xrun::validation_lines::*;

#[test]
fn test_validation_lines() {
	let mut line = String::from("✓ Test 'sample' passed!");
	let mut passed_tests = 0;
	let mut failed_tests = 0;
	let mut test_errors = Vec::new();
	let mut current_module = String::new();

	ValidationLine::validation_lines(
		&mut line,
		&mut passed_tests,
		&mut failed_tests,
		&mut test_errors,
		&mut current_module,
	);

	assert_eq!(passed_tests, 1);
	assert_eq!(failed_tests, 0);
	assert_eq!(test_errors.len(), 0);
}

#[test]
fn test_failed_line() {
	let mut line = String::from("✗ Test 'sampleFailing' failed!");
	let mut passed_tests = 0;
	let mut failed_tests = 0;
	let mut test_errors = Vec::new();
	let mut current_module = String::from("SampleModule");

	ValidationLine::validation_lines(
		&mut line,
		&mut passed_tests,
		&mut failed_tests,
		&mut test_errors,
		&mut current_module,
	);

	assert_eq!(passed_tests, 0);
	assert_eq!(failed_tests, 1);
	assert_eq!(test_errors.len(), 1);
	assert_eq!(test_errors[0].0, "SampleModule");
}

#[test]
fn test_plain_line_is_ignored() {
	let mut line = String::from("Some build output without markers");
	let mut passed_tests = 0;
	let mut failed_tests = 0;
	let mut test_errors = Vec::new();
	let mut current_module = String::new();

	ValidationLine::validation_lines(
		&mut line,
		&mut passed_tests,
		&mut failed_tests,
		&mut test_errors,
		&mut current_module,
	);

	assert_eq!(passed_tests, 0);
	assert_eq!(failed_tests, 0);
	assert_eq!(test_errors.len(), 0);
}

#[test]
fn test_multiple_passes_and_failures() {
	let mut passed_tests = 0u128;
	let mut failed_tests = 0u128;
	let mut test_errors = Vec::new();
	let mut current_module = String::from("MyModule");

	let pass_lines = ["✓ testA", "✓ testB", "✓ testC"];
	let fail_lines = ["✗ testD", "✗ testE"];

	for l in &pass_lines {
		let mut line = l.to_string();
		ValidationLine::validation_lines(
			&mut line,
			&mut passed_tests,
			&mut failed_tests,
			&mut test_errors,
			&mut current_module,
		);
	}
	
	for l in &fail_lines {
		let mut line = l.to_string();
		ValidationLine::validation_lines(
			&mut line,
			&mut passed_tests,
			&mut failed_tests,
			&mut test_errors,
			&mut current_module,
		);
	}

	assert_eq!(passed_tests, 3);
	assert_eq!(failed_tests, 2);
	assert_eq!(test_errors.len(), 2);
}

#[test]
fn test_failed_line_records_original_line() {
	let original = "✗ testBroken assertion failed at line 42";
	let mut line = original.to_string();
	let mut passed_tests = 0;
	let mut failed_tests = 0;
	let mut test_errors = Vec::new();
	let mut current_module = String::from("CoreModule");

	ValidationLine::validation_lines(
		&mut line,
		&mut passed_tests,
		&mut failed_tests,
		&mut test_errors,
		&mut current_module,
	);

	assert_eq!(test_errors[0].1, original);
}
