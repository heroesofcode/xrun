use std::error::Error;
use std::process::exit;
use std::time::Instant;
use comfy_table::Table;
use colored::Colorize;
use std::fs::File;
use printpdf::*;
use std::io::BufWriter;

pub struct Results {}

impl Results {

    pub fn show_results(start_time: Instant, passed_tests: u128, failed_tests: u128) {

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

            if let Err(e) = Self::generate_file(
                table, file,
                "results-xrun.pdf") {
                eprintln!("Failed to generate file 'results-xrun.pdf': {}", e);
            }
        }
    }

    fn generate_file(table: Table, generate: bool, file_path: &str) -> Result<(), Box<dyn Error>> {
        if generate {
            let table_string = table.to_string();
            let lines: Vec<&str> = table_string.split('\n').collect();
            let line_height = 4.0;
            let font_size = 8.0;
            let page_width = 310.0;
            let mut required_height = lines.len() as f64 * line_height + 20.0;

            if required_height > 297.0 {
                required_height = 297.0;
            }

            let (doc, page1, layer1) = PdfDocument::new(
                file_path,
                Mm(page_width as f32),
                Mm(required_height as f32),
                "Layer 1"
            );

            let current_layer = doc.get_page(page1).get_layer(layer1);

            let font = doc.add_builtin_font(BuiltinFont::Courier)?;

            let mut current_y = Mm((required_height - 10.0) as f32);

            for line in lines {
                current_layer.use_text(line, font_size, Mm(10.0), current_y, &font);
                current_y -= Mm(line_height as f32);
            }

            let file = File::create(file_path)?;
            let mut buffer = BufWriter::new(file);

            doc.save(&mut buffer)?;
        }

        Ok(())
    }
}