use colored::Colorize;

pub fn validation_lines(
    line: &mut String, 
    passed_tests: &mut i32, 
    failed_tests: &mut i32, 
    get_errors: &mut Vec<String>,
    file_names: &mut Vec<String>) {

    if line.contains("Test Suite") && line.contains("started") {
        if let Some(start) = line.find("Test Suite") {
            if let Some(end) = line.find("started") {
                let mut test_name = line[start+11..end].trim().to_string();

                if test_name.ends_with(".xctest") {
                    test_name.truncate(test_name.len() - 7);
                }

                println!("\n{}", test_name.purple());
                file_names.push(test_name);
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
        get_errors.push(line.clone());
    }
}