use colored::Colorize;
use comfy_table::Table;
use printpdf::*;
use std::error::Error;
use std::process::exit;
use std::time::Instant;

pub struct Results {}

/// Decodes common HTML entities to their character equivalents.
/// Handles entities like &quot;, &amp;, etc. that may appear in test output.
fn decode_html_entities(s: &str) -> String {
	s.replace("&quot;", "\"")
		.replace("&amp;", "&")
		.replace("&lt;", "<")
		.replace("&gt;", ">")
		.replace("&apos;", "'")
}

/// Sanitizes text for PDF builtin fonts (WinAnsi encoding).
/// Replaces Unicode characters that would render incorrectly with ASCII equivalents.
/// For example: × → x, ã → a, — → -
fn sanitize_for_pdf(s: &str) -> String {
	s.chars()
		.map(|c| match c {
			'×' => 'x',
			'÷' => '/',
			'´' | '`' | 'ʻ' => '\'',
			'–' | '—' => '-',
			'"' | '\u{201c}' | '\u{201d}' => '"',
			'\'' | '\u{2018}' | '\u{2019}' => '\'',
			'ã' | 'á' | 'à' | 'â' | 'ä' | 'å' | 'ā' => 'a',
			'Ã' | 'Á' | 'À' | 'Â' | 'Ä' | 'Å' | 'Ā' => 'A',
			'é' | 'è' | 'ê' | 'ë' | 'ē' => 'e',
			'É' | 'È' | 'Ê' | 'Ë' | 'Ē' => 'E',
			'í' | 'ì' | 'î' | 'ï' | 'ī' => 'i',
			'Í' | 'Ì' | 'Î' | 'Ï' | 'Ī' => 'I',
			'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ō' => 'o',
			'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' | 'Ō' => 'O',
			'ú' | 'ù' | 'û' | 'ü' | 'ū' => 'u',
			'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ū' => 'U',
			'ç' | 'Ç' => 'c',
			'ñ' | 'Ñ' => 'n',
			'ß' => 's',
			'€' => 'E',
			_ if c.is_ascii() => c,
			_ => '?',
		})
		.collect()
}

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

		if let Err(e) = Self::generate_file(table, generate_file, "results-xrun.pdf") {
			eprintln!("Failed to generate file 'results-xrun.pdf': {}", e);
		}
	}

	/// Generates a PDF file containing the test results table.
	/// Only creates the file if generate is true.
	fn generate_file(table: Table, generate: bool, file_path: &str) -> Result<(), Box<dyn Error>> {
		if !generate {
			return Ok(());
		}

		const LINE_HEIGHT_MM: f64 = 4.0;
		const FONT_SIZE_PT: f64 = 8.0;
		const PAGE_WIDTH_MM: f64 = 310.0;
		const MAX_PAGE_HEIGHT_MM: f64 = 297.0; // A4 height
		const MARGIN_MM: f64 = 10.0;

		let table_string = table.to_string();
		let lines: Vec<&str> = table_string.split('\n').collect();

		let mut required_height = lines.len() as f64 * LINE_HEIGHT_MM + 20.0;
		if required_height > MAX_PAGE_HEIGHT_MM {
			required_height = MAX_PAGE_HEIGHT_MM;
		}

		let mut doc = PdfDocument::new("xrun results");
		let mut ops = vec![
			Op::StartTextSection,
			Op::SetFont {
				font: PdfFontHandle::Builtin(BuiltinFont::Courier),
				size: Pt(FONT_SIZE_PT as f32),
			},
		];

		let mut current_y = (required_height - MARGIN_MM) as f32;
		
		for line in &lines {
			ops.push(Op::SetTextMatrix {
				matrix: TextMatrix::Translate(Pt::from(Mm(MARGIN_MM as f32)), Pt::from(Mm(current_y))),
			});
			ops.push(Op::ShowText {
				items: vec![TextItem::Text(sanitize_for_pdf(line))],
			});
			current_y -= LINE_HEIGHT_MM as f32;
		}

		ops.push(Op::EndTextSection);

		let page = PdfPage::new(Mm(PAGE_WIDTH_MM as f32), Mm(required_height as f32), ops);

		doc.pages.push(page);

		let mut warnings = Vec::new();
		let bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
		std::fs::write(file_path, bytes)?;

		Ok(())
	}
}
