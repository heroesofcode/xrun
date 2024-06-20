use std::process::{exit, ChildStdout, Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::env;
use colored::Colorize;
use std::process::Child;

mod header;
mod validation_lines;
mod results;

use crate::header::*;
use crate::validation_lines::*;
use crate::results::*;

fn main() {
    header();

    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() > 5 {
        let get_arg1 = validation_arg1(&args[1]);
        let output = get_output(
            get_arg1,
            &args[2],
            &args[3],
            &args[4],
            &args[5]);

        let mut passed_tests: u128 = 0;
        let mut failed_tests: u128 = 0;

        let mut current_module = String::new();
        let mut test_errors: Vec<(String, String)> = Vec::new();

        if let Some(stdout) = output.stdout {
            process_output(
                stdout, 
                &mut passed_tests, 
                &mut failed_tests, 
                &mut test_errors, 
                &mut current_module)
        }

        results(start_time, passed_tests, failed_tests);
        validation_arg_fail_and_file(args, test_errors);
    } else {
        println!("{}", "Error in arguments".red());
        exit(1)
    }
}

fn validation_arg_fail_and_file(args: Vec<String>, test_errors: Vec<(String, String)>) {
    let arg6 = args.get(6).map(|s| s.as_str());
    let arg7 = args.get(7).map(|s| s.as_str());

    if !test_errors.is_empty() {
        match (arg6, arg7) {
            (Some("fail"), None) => {
                validation_show_errors(test_errors, false);
                exit(1);
            },
            (Some("fail"), Some("generate-file")) => {
                show_message_success_with_file(test_errors);
                exit(1);
            },
            (Some("generate-file"), Some("fail")) => {
                println!("{}", "Error in arguments with fail or generate-file".red());
                exit(1);
            },
            (Some("fail"), Some(other)) if other != "generate-file" => {
                println!("{}", "Error in arguments with fail or generate-file".red());
                exit(1);
            }
            (Some(other), None) if other != "generate-file" => {
                println!("{}", "Error in arguments with fail or generate-file".red());
                exit(1);
            },
            (Some("generate-file"), None) => {
                show_message_success_with_file(test_errors);
            },
            (_, _) => {
                validation_show_errors(test_errors, false);
            }
        }
    } else {
        validation_show_errors(test_errors, false);
    }
}

fn show_message_success_with_file(test_errors: Vec<(String, String)>) {
    validation_show_errors(test_errors, true);
    println!("{}", "results-xrun.pdf file generated successfully".green());
}

fn validation_arg1(arg1: &String) -> &str {
    let get_arg1 = match arg1.as_str() {
        "project" => "-project",
        "workspace" => "-workspace",
        _ => {
            let text_error_arguments = "Error in arguments".red();
            println!("{}", text_error_arguments);
            exit(1)
        }
    };

    return get_arg1;
}

fn get_output(
    arg1: &str, 
    arg2: &String, 
    arg3: &String, 
    arg4: &String, 
    arg5: &String) -> Child {

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "set -o pipefail && xcodebuild {} {} \
             -scheme {} \
             -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
             clean test | xcpretty", arg1, arg2, arg3, arg4, arg5))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

        
    return output;
}

fn process_output(
    stdout: ChildStdout, 
    passed_tests: &mut u128, 
    failed_tests: &mut u128, 
    test_errors: &mut Vec<(String, String)>,
    current_module: &mut String) {

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut mutable_line = line.clone();
            validation_lines(
                &mut mutable_line, 
                passed_tests, 
                failed_tests, 
                test_errors,
                current_module);
        }
    }
}
