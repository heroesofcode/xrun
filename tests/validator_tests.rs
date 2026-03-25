use xrun::validator::*;

#[test]
fn test_no_errors_no_flags_returns_ok() {
	let result = Validator::handle_validation_args(false, false, &[]);
	assert!(result.is_ok());
}

#[test]
fn test_no_errors_with_fail_flag_returns_ok() {
	let result = Validator::handle_validation_args(true, false, &[]);
	assert!(result.is_ok());
}

#[test]
fn test_fail_flag_with_errors_returns_err() {
	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(true, false, &errors);
	assert!(result.is_err());
}

#[test]
fn test_no_flags_with_errors_returns_ok() {
	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(false, false, &errors);
	assert!(result.is_ok());
}

#[test]
fn test_fail_and_generate_file_with_errors_returns_err() {
	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(true, true, &errors);
	assert!(result.is_err());
}

#[test]
fn test_generate_file_only_with_errors_returns_ok() {
	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(false, true, &errors);
	assert!(result.is_ok());
}
