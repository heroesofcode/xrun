use std::process::{exit, Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::env;
use comfy_table::Table;
use colored::Colorize;

mod header;
mod validation;

use crate::header::*;
use crate::validation::*;

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

fn results(start_time: Instant, passed_tests: i32, failed_tests: i32) {

    if passed_tests == 0 && failed_tests == 0 {
        println!("{}", "We had a problem with the command, I recommend you check if you are using the correct arguments".red());
        exit(1);
    } else {
        println!("\n\n🗳️  The results have been completed below\n");

        let duration = start_time.elapsed();

        let mut table = Table::new();

        let titles = vec!["Runtime", "Total Tests", "✅ Passed Tests", " ❌ Failed Tests"];
        table.set_header(titles);

        let formatted_duration = format!("{:.2?}", duration);
        let total_tests = passed_tests + failed_tests;
        let row = vec![
            formatted_duration.to_string(), 
            total_tests.to_string(), 
            passed_tests.to_string(),
            failed_tests.to_string()];

        table.add_row(row);
        println!("{table}");
    }
}

fn validation_show_errors(get_errors: Vec<String>) {
    let mut table = Table::new();

    let titles = vec!["Errors found"];
    table.set_header(titles);

    if get_errors.is_empty() {
        println!("{}", "\n👏 Congratulations, no errors were found!!!".green())
    } else {
        println!("{}", "\n ⚠️  Below contains the errors \n".yellow());
        
        for errors in &get_errors {
            let row = vec![errors.to_string()];
            table.add_row(row);
        }

        println!("{table}");
    }
}