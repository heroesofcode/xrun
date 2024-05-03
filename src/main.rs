use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::env;

fn main() {
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
                    if line.contains("✓") {
                        passed_tests += 1;
                        println!("{}", line);
                    } else if line.contains("✗") {
                        failed_tests += 1;
                        println!("{}", line);
                        get_errors.push(line);
                    }
                }
            }
        }

        results(start_time, passed_tests, failed_tests);

        for errors in &get_errors {
            println!("{}", errors)
        }
    } else {
        println!("Error in arguments")
    }
}

fn results(start_time: Instant, passed_tests: i32, failed_tests: i32) {
    let duration = start_time.elapsed();

    println!("Runtime: {:.2?}", duration);

    println!("Test Summary:");
    println!("Total Tests: {}", passed_tests + failed_tests);
    println!("✅: {}", passed_tests);
    println!("❌: {}", failed_tests);
}