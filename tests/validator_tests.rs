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
        "generate-file".to_string()
    ];

    let result = Validator::handle_validation_args(&args, &[]);
    assert!(result.is_ok());
}

#[test]
fn test_validation_arg1() {
    assert_eq!(Validator::validation_arg1("project"), "-project");
    assert_eq!(Validator::validation_arg1("workspace"), "-workspace");
}