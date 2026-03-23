use xrun::validator::*;

#[test]
fn test_validator_args() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"platform".to_string(),
		"device".to_string(),
		"fail".to_string(),
		"generate-file".to_string(),
	];

	let result = Validator::handle_validation_args(&args, &[]);
	assert!(result.is_ok());
}

#[test]
fn test_validation_arg1() {
	assert_eq!(Validator::validation_arg1("project"), "-project");
	assert_eq!(Validator::validation_arg1("workspace"), "-workspace");
}

#[test]
fn test_no_errors_no_flags_returns_ok() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"macOS".to_string(),
	];

	let result = Validator::handle_validation_args(&args, &[]);
	assert!(result.is_ok());
}

#[test]
fn test_fail_flag_with_errors_returns_err() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"macOS".to_string(),
		"".to_string(),
		"fail".to_string(),
	];

	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(&args, &errors);
	assert!(result.is_err());
}

#[test]
fn test_no_flags_with_errors_returns_ok() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"macOS".to_string(),
	];

	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(&args, &errors);
	assert!(result.is_ok());
}

#[test]
fn test_generate_file_before_fail_returns_err() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"macOS".to_string(),
		"".to_string(),
		"generate-file".to_string(),
		"fail".to_string(),
	];

	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(&args, &errors);
	assert!(result.is_err());
}

#[test]
fn test_invalid_flag_returns_err() {
	let args = vec![
		"xrun".to_string(),
		"project".to_string(),
		"path".to_string(),
		"scheme".to_string(),
		"macOS".to_string(),
		"".to_string(),
		"fail".to_string(),
		"unknown-flag".to_string(),
	];
	
	let errors = vec![("Module".to_string(), "testFoo failed".to_string())];
	let result = Validator::handle_validation_args(&args, &errors);
	assert!(result.is_err());
}
