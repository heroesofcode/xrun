use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::env;
use comfy_table::Table;
use colored::Colorize;

mod header;

use crate::header::header;

fn main() {
    header();

    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() > 4 {
        let arg1 = &args[1];
        let arg2 = &args[2];
        let arg3 = &args[3];
        let arg4 = &args[4];

        let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "set -o pipefail && xcodebuild -project {} \
             -scheme {} \
             -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
             clean test | xcpretty", arg1, arg2, arg3, arg4))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

        let mut passed_tests = 0;
        let mut failed_tests = 0;

        let mut get_errors: Vec<String> = Vec::new();

        if let Some(stdout) = output.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    validation_lines(
                        line, 
                        &mut passed_tests, 
                        &mut failed_tests, 
                        &mut get_errors);
                }
            }
        }

        results(start_time, passed_tests, failed_tests);
        validation_show_errors(get_errors);
    } else {
        println!("Error in arguments")
    }
}

fn validation_lines(
    line: String, 
    passed_tests: &mut i32, 
    failed_tests: &mut i32, 
    get_errors: &mut Vec<String>) {

    if line.contains("✓") {
        *passed_tests += 1;
        println!("{}", line.green());
    } else if line.contains("✗") {
        *failed_tests += 1;
        println!("{}", line.red());
        get_errors.push(line);
    }
}

fn results(start_time: Instant, passed_tests: i32, failed_tests: i32) {
    println!("\n🗳️  The results have been completed below\n");

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

fn validation_show_errors(get_errors: Vec<String>) {
    let mut table = Table::new();

    let titles = vec!["Errors found"];
    table.set_header(titles);

    if get_errors.is_empty() {
        let text_congratulations = "\n👏 Congratulations, no errors were found!!!".green();
        println!("{}", text_congratulations)
    } else {
        let text_constains_error = "\n ⚠️  Below contains the errors \n".red();
        println!("{}", text_constains_error);
        
        for errors in &get_errors {
            let row = vec![errors.to_string()];
            table.add_row(row);
        }

        println!("{table}");
    }
}