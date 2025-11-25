use colored::Colorize;
use crate::results::Results;
use crate::utils::Utils;

enum ValidationArg {
    Fail,
    GenerateFile,
    Other,
    None
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

    pub fn handle_validation_args(
        args: &[String], 
        test_errors: &[(String, String)]
    ) -> Result<(), String> {
        let arg6 = ValidationArg::from_option(args.get(6).map(String::as_str));
        let arg7 = ValidationArg::from_option(args.get(7).map(String::as_str));

        if test_errors.is_empty() {
            Results::validation_show_errors(test_errors.to_vec(), false);
            return Ok(());
        }

        match (arg6, arg7) {
            (ValidationArg::Fail, ValidationArg::None) => {
                Results::validation_show_errors(test_errors.to_vec(), false);
                Err("Tests failed".into())
            },
            (ValidationArg::Fail, ValidationArg::GenerateFile) => {
                Utils::show_message_success_with_file(test_errors.to_vec());
                Err("Tests failed (file generated)".into())
            },
            (ValidationArg::GenerateFile, ValidationArg::Fail) => {
                Err("Argument error: conflict between fail and generate-file".into())
            },
            (ValidationArg::Fail, ValidationArg::Other) | 
            (ValidationArg::Other, ValidationArg::None) => {
                Err("Argument error: invalid value".into())
            },
            (ValidationArg::GenerateFile, ValidationArg::None) => {
                Utils::show_message_success_with_file(test_errors.to_vec());
                Ok(())
            },
            _ => {
                Results::validation_show_errors(test_errors.to_vec(), false);
                Err("Invalid arguments".into())
            }
        }
    }

    pub fn validation_arg1(arg1: &str) -> &str {
        match arg1 {
            "project" => "-project",
            "workspace" => "-workspace",
            _ => {
                eprintln!("{}", "Error in arguments".red());
                std::process::exit(1)
            }
        }
    }
}