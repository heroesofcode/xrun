use std::process::exit;
use std::time::Instant;
use std::env;
use colored::Colorize;

mod header;
mod validation_lines;
mod results;
mod validator;
mod utils;
mod output;

use crate::header::*;
use crate::results::*;
use crate::validator::*;
use crate::output::*;

fn main() {
    Header::show_header();

    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{}", "Commands not found".red());
        exit(1);
    }

    let get_arg1 = Validator::validation_arg1(&args[1]);

    let get_arg5 = if args.len() > 5 {
        Some(&args[5])
    } else if &args[4] != "macOS" {
        eprintln!("{}", "Error in arguments: arg5 is required for non-macOS platforms".red());
        exit(1);
    } else {
        None
    };

    let output = Output::get_output(
        get_arg1,
        &args[2],
        &args[3],
        &args[4],
        get_arg5);

    let mut passed_tests: u128 = 0;
    let mut failed_tests: u128 = 0;

    let mut current_module = String::new();
    let mut test_errors: Vec<(String, String)> = Vec::new();

    if let Some(stdout) = output.stdout {
        Output::process_output(
            stdout,
            &mut passed_tests,
            &mut failed_tests,
            &mut test_errors,
            &mut current_module)
    }

    Results::show_results(start_time, passed_tests, failed_tests);
    Validator::validation_arg_fail_and_file(args, test_errors);
}