use std::process::{exit, Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::env;
use colored::Colorize;

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
        let arg1 = &args[1];
        let arg2 = &args[2];
        let arg3 = &args[3];
        let arg4 = &args[4];
        let arg5 = &args[5];

        let get_arg1 = match arg1.as_str() {
            "project" => "project",
            "workspace" => "workspace",
            _ => {
                let text_error_arguments = "Error in arguments".red();
                println!("{}", text_error_arguments);
                exit(1)
            }
        };

        let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "set -o pipefail && xcodebuild -{} {} \
             -scheme {} \
             -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
             clean test | xcpretty", get_arg1, arg2, arg3, arg4, arg5))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

        let mut passed_tests = 0;
        let mut failed_tests = 0;

        let mut get_errors: Vec<String> = Vec::new();
        let mut file_names: Vec<String> = Vec::new();

        if let Some(stdout) = output.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {

                    let mut mutable_line = line.clone();
                    validation_lines(
                        &mut mutable_line, 
                        &mut passed_tests, 
                        &mut failed_tests, 
                        &mut get_errors,
                        &mut file_names);
                }
            }
        }

        results(start_time, passed_tests, failed_tests);
        validation_show_errors(get_errors);
    } else {
        println!("{}", "Error in arguments".red());
        exit(1)
    }
}