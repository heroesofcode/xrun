use crate::text_utils::sanitize_for_pdf;
use comfy_table::Table;
use printpdf::*;
use std::error::Error;

const LINE_HEIGHT_MM: f64 = 4.0;
const FONT_SIZE_PT: f64 = 8.0;
const PAGE_WIDTH_MM: f64 = 310.0;
const MAX_PAGE_HEIGHT_MM: f64 = 297.0;
const MARGIN_MM: f64 = 10.0;

pub struct PdfReport;

impl PdfReport {
	/// Generates a PDF file containing the provided test results table.
	pub fn generate(table: Table, file_path: &str) -> Result<(), Box<dyn Error>> {
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
