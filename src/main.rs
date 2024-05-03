use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "set -o pipefail && xcodebuild -project DeliveryApp-iOS/DeliveryApp.xcodeproj \
             -scheme DeliveryApp \
             -destination platform=iOS\\ Simulator,OS=17.4,name=iPhone\\ 15 \
             clean test | xcpretty"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut passed_tests = 0;
    let mut failed_tests = 0;

    if let Some(stdout) = output.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("✓") {
                    passed_tests += 1;
                    println!("{}", line);
                } else if line.contains("❌") {
                    failed_tests += 1;
                    println!("{}", line);
                }
            }
        }
    }

    results(start_time, passed_tests, failed_tests);
}

fn results(start_time: Instant, passed_tests: i32, failed_tests: i32) {
    let duration = start_time.elapsed();

    println!("Runtime: {:.2?}", duration);

    println!("Test Summary:");
    println!("Total Tests: {}", passed_tests + failed_tests);
    println!("✅: {}", passed_tests);
    println!("❌: {}", failed_tests);
}