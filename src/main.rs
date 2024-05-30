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
        let output = get_output(get_arg1, &args[2], &args[3], &args[4], &args[5]);

        let mut passed_tests = 0;
        let mut failed_tests = 0;

        let mut current_module = String::new();
        let mut test_errors: Vec<(String, String)> = Vec::new();  // Vetor para armazenar módulo e erro correspondente

        if let Some(stdout) = output.stdout {
            process_output(
                stdout, 
                &mut passed_tests, 
                &mut failed_tests, 
                &mut test_errors, 
                &mut current_module)
        }

        results(start_time, passed_tests, failed_tests);

        if args.get(6) == Some(&"fail".to_string()) && !test_errors.is_empty() {
            validation_show_errors(test_errors);
            exit(1);
        } else {
            validation_show_errors(test_errors);
        }
    } else {
        println!("{}", "Error in arguments".red());
        exit(1)
    }
}

fn validation_arg1(arg1: &String) -> &str {
    let get_arg1 = match arg1.as_str() {
            "project" => "-project",
            "workspace" => "-workspace",
            "test" => "test",
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
    passed_tests: &mut i32, 
    failed_tests: &mut i32, 
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