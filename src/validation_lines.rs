use colored::Colorize;

pub struct ValidationLine {}

impl ValidationLine {

    pub fn validation_lines(
        line: &mut String,
        passed_tests: &mut u128,
        failed_tests: &mut u128,
        test_errors: &mut Vec<(String, String)>,
        current_module: &mut String,
    ) {
        let is_pass = line.contains("✓") && (line.contains("test") || line.contains("Test"));
        let is_fail = line.contains("✗");
        if is_pass {
            *passed_tests += 1;
            let cleaned = line.replace("✓", "").trim().to_string();
            println!("    {} {}", "✅", cleaned.green());
        } else if is_fail {
            *failed_tests += 1;
            let cleaned = line.replace("✗", "").trim().to_string();
            println!("    {} {}", "❌", cleaned.red());
            test_errors.push((current_module.clone(), line.clone()));
        }
    }
}