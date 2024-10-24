use colored::Colorize;

pub struct ValidationLine {}

impl ValidationLine {

    pub fn validation_lines(
        line: &mut String,
        passed_tests: &mut u128,
        failed_tests: &mut u128,
        test_errors: &mut Vec<(String, String)>,
        current_module: &mut String) {

        if line.contains("Test Suite") && line.contains("started") {
            if let Some(start) = line.find("Test Suite") {
                if let Some(end) = line.find("started") {
                    *current_module = line[start+11..end].trim().to_string();

                    if current_module.ends_with(".xctest") {
                        current_module.truncate(current_module.len() - 7);
                    }

                    println!("\n{}", current_module.purple());
                }
            }
        }

        if line.contains("✓") {
            *line = line.replace("✓", "✅");
            *passed_tests += 1;

            println!("{}", line.green());
        } else if line.contains("✗") {
            *line = line.replace("✗", "❌");
            *failed_tests += 1;

            println!("{}", line.red());

            test_errors.push((current_module.clone(), line.clone()));
        }
    }
}