use crate::pdf_report::PdfReport;
use crate::text_utils::decode_html_entities;
use colored::Colorize;
use comfy_table::Table;
use std::process::exit;
use std::time::Instant;

pub struct Results {}

impl Results {
	/// Displays a summary table of test results including runtime and pass/fail counts.
	pub fn show_results(start_time: Instant, passed_tests: u128, failed_tests: u128) {
		if passed_tests == 0 && failed_tests == 0 {
			println!(
                "{}",
                "We had a problem with the command, I recommend you check if you are using the correct arguments".red()
            );

			exit(1);
		}

		println!("\n\n🗳️  The results have been completed below\n");

		let duration = start_time.elapsed();

		let mut table = Table::new();
		table.set_header(vec![
			"Runtime",
			"Total Tests",
			"✅ Passed Tests",
			"❌ Failed Tests",
		]);

		let total_tests = passed_tests + failed_tests;
		table.add_row(vec![
			format!("{:.2?}", duration),
			total_tests.to_string(),
			passed_tests.to_string(),
			failed_tests.to_string(),
		]);

		println!("{table}");
	}

	/// Displays test errors in a table and optionally generates a PDF report.
	pub fn validation_show_errors(test_errors: Vec<(String, String)>, generate_file: bool) {
		let mut table = Table::new();
		table.set_header(vec!["Module", "Errors found"]);

		if test_errors.is_empty() {
			println!(
				"{}",
				"\n👏 Congratulations, no errors were found!!!".green()
			);
			return;
		}

		println!("{}", "\n⚠️ Below contains the errors\n".yellow());

		for (module, error) in test_errors {
			let module = decode_html_entities(&module);
			let error = decode_html_entities(&error);
			table.add_row(vec![module, error]);
		}
		println!("{table}");

		if generate_file {
			if let Err(e) = PdfReport::generate(table, "results-xrun.pdf") {
				eprintln!("Failed to generate file 'results-xrun.pdf': {}", e);
			}
		}
	}
}
