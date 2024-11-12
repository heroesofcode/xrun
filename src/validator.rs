use std::process::exit;
use colored::Colorize;
use crate::results::*;
use crate::utils::*;

pub struct Validator;

impl Validator {

    pub fn validation_arg_fail_and_file(args: Vec<String>, test_errors: Vec<(String, String)>) {
        let arg6 = args.get(6).map(|s| s.as_str());
        let arg7 = args.get(7).map(|s| s.as_str());

        if !test_errors.is_empty() {
            match (arg6, arg7) {
                (Some("fail"), None) => {
                    Results::validation_show_errors(test_errors, false);
                    exit(1);
                }
                (Some("fail"), Some("generate-file")) => {
                    Utils::show_message_success_with_file(test_errors);
                    exit(1);
                }
                (Some("generate-file"), Some("fail")) => {
                    println!("{}", "Error in arguments with fail or generate-file".red());
                    exit(1);
                }
                (Some("fail"), Some(other)) if other != "generate-file" => {
                    println!("{}", "Error in arguments with fail or generate-file".red());
                    exit(1);
                }
                (Some(other), None) if other != "generate-file" => {
                    println!("{}", "Error in arguments with fail or generate-file".red());
                    exit(1);
                }
                (Some("generate-file"), None) => {
                    Utils::show_message_success_with_file(test_errors);
                }
                (_, _) => {
                    Results::validation_show_errors(test_errors, false);
                }
            }
        } else {
            Results::validation_show_errors(test_errors, false);
        }
    }

    pub fn validation_arg1(arg1: &str) -> &str {
        match arg1 {
            "project" => "-project",
            "workspace" => "-workspace",
            _ => {
                let text_error_arguments = "Error in arguments".red();
                println!("{}", text_error_arguments);
                exit(1)
            }
        }
    }

}