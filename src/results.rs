use std::process::exit;
use std::time::Instant;
use comfy_table::Table;
use colored::Colorize;
use std::fs::File;
use std::io::Write;

pub fn results(start_time: Instant, passed_tests: u128, failed_tests: u128) {

    if passed_tests == 0 && failed_tests == 0 {
        println!(
            "{}",
            "We had a problem with the command, I recommend you \
            check if you are using the correct arguments".red()
        );

        exit(1);
    } else {
        println!("\n\n🗳️  The results have been completed below\n");

        let duration = start_time.elapsed();
        let mut table = Table::new();

        let titles = vec![
            "Runtime",
            "Total Tests",
            "✅ Passed Tests",
            "❌ Failed Tests"];

        table.set_header(titles);

        let formatted_duration = format!("{:.2?}", duration);
        let total_tests = passed_tests + failed_tests;
        let row = vec![
            formatted_duration.to_string(), 
            total_tests.to_string(), 
            passed_tests.to_string(),
            failed_tests.to_string()];

        table.add_row(row);
        println!("{table}");
    }
}

pub fn validation_show_errors(test_errors: Vec<(String, String)>, file: bool) {
    let mut table = Table::new();
    table.set_header(vec!["Module", "Errors found"]);

    if test_errors.is_empty() {
        println!("{}", "\n👏 Congratulations, no errors were found!!!".green());
    } else {
        println!("{}", "\n⚠️ Below contains the errors\n".yellow());

        for (module, error) in test_errors {
            table.add_row(vec![module, error]);
        }

        println!("{table}");
        generate_file(table, file)
    }
}

fn generate_file(table: Table, generate: bool) {
    if generate == true {
        let mut file = File::create("results-xrun.txt")
            .expect("Unable to generate file");

        let table_string = table.to_string();
        let table_as_bytes = table_string.as_bytes();

        file.write_all(table_as_bytes).expect("Unable to write file");
    }
}