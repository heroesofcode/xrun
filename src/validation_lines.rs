use colored::Colorize;

pub struct ValidationLine {}

impl ValidationLine {

    pub fn validation_lines(
        line: &mut String,
        passed_tests: &mut u128,
        failed_tests: &mut u128,
        test_errors: &mut Vec<(String, String)>,
        current_module: &mut String) {

        if line.contains("✓") && (line.contains("test") || line.contains("Test")) {
            *passed_tests += 1;
            println!("    {} {}", "✅", line.trim().replace("✓", "").trim().green());
        } else if line.contains("✗") {
            *failed_tests += 1;
            println!("    {} {}", "❌", line.trim().replace("✗", "").trim().red());
            test_errors.push((current_module.clone(), line.clone()));
        }
    }
}