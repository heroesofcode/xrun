use std::process::exit;
use std::time::Instant;
use std::env;
use colored::Colorize;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

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
    
    let mut passed = 0u128;
    let mut failed = 0u128;
    let mut current_module = String::new();
    let mut errors = Vec::new();
    let mut failed_any = false;

    run_with_spinner("Running xcodebuild...", || {
        let output = Output::get_output(
            build_type,
            &args[2],
            &args[3],
            destination,
            device,
        );

        let output_result = output.wait_with_output().expect("Failed to wait for xcodebuild process");
        if !output_result.stdout.is_empty() {
            use std::io::Cursor;
            let cursor = Cursor::new(output_result.stdout.clone());
            Output::process_output(
                cursor,
                &mut passed,
                &mut failed,
                &mut errors,
                &mut current_module,
            );
        }
        if !output_result.status.success() {
            failed_any = true;
        }
    });

    Results::show_results(start_time, passed, failed);
    Validator::validation_arg_fail_and_file(args, errors);
    if failed_any {
        exit(1);
    }
}

fn run_with_spinner<F: FnOnce()>(_message: &str, job: F) {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template(&format!("{{spinner:.blue}} {} [{{elapsed_precise}}]\n", _message))
            .expect("Failed to set progress bar template")
    );
    progress_bar.enable_steady_tick(Duration::from_millis(100));
    job();
    progress_bar.finish_with_message("xcodebuild finished!");
}