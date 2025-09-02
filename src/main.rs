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

use crate::header::Header;
use crate::results::Results;
use crate::validator::Validator;
use crate::output::Output;

fn main() {
    Header::show_header();
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("{}", "Commands not found".red());
        exit(1);
    }
    let build_type = Validator::validation_arg1(&args[1]);
    let destination = &args[4];
    let device = if args.len() > 5 {
        Some(&args[5])
    } else if destination != "macOS" {
        eprintln!("{}", "Error in arguments: arg5 is required for non-macOS platforms".red());
        exit(1);
    } else {
        None
    };
    let output = Output::get_output(
        build_type,
        &args[2],
        &args[3],
        destination,
        device,
    );
    let mut passed = 0u128;
    let mut failed = 0u128;
    let mut current_module = String::new();
    let mut errors = Vec::new();
    if let Some(stdout) = output.stdout {
        Output::process_output(
            stdout,
            &mut passed,
            &mut failed,
            &mut errors,
            &mut current_module,
        );
    }
    Results::show_results(start_time, passed, failed);
    Validator::validation_arg_fail_and_file(args, errors);
}