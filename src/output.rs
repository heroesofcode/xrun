use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Stdio};
use crate::validation_lines::*;

pub struct Output;

impl Output {

    pub fn get_output(
        arg1: &str,
        arg2: &String,
        arg3: &String,
        arg4: &String,
        arg5: Option<&String>) -> Child {
        if arg4 == "macOS" {
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform={} \
                 clean test | xcpretty", arg1, arg2, arg3, arg4))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start command");

            output
        } else {
            let arg5_str = arg5.expect("arg5 is required for non-macOS platforms");

            let output = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "set -o pipefail && xcodebuild {} {} \
                 -scheme {} \
                 -destination platform=iOS\\ Simulator,OS={},name=iPhone\\ {} \
                 clean test | xcpretty", arg1, arg2, arg3, arg4, arg5_str))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start command");

            output
        }
    }

    pub fn process_output(
        stdout: ChildStdout,
        passed_tests: &mut u128,
        failed_tests: &mut u128,
        test_errors: &mut Vec<(String, String)>,
        current_module: &mut String) {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut mutable_line = line.clone();
                ValidationLine::validation_lines(
                    &mut mutable_line,
                    passed_tests,
                    failed_tests,
                    test_errors,
                    current_module);
            }
        }
    }

}