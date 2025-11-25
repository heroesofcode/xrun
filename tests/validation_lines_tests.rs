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
        &mut current_module
    );

    assert_eq!(passed_tests, 1);
    assert_eq!(failed_tests, 0);
    assert_eq!(test_errors.len(), 0);
}