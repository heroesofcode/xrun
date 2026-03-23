/// Decodes common HTML entities to their character equivalents.
/// Handles entities like &quot;, &amp;, etc. that may appear in test output.
pub fn decode_html_entities(s: &str) -> String {
	s.replace("&quot;", "\"")
		.replace("&amp;", "&")
		.replace("&lt;", "<")
		.replace("&gt;", ">")
		.replace("&apos;", "'")
}

/// Sanitizes text for PDF builtin fonts (WinAnsi encoding).
/// Replaces Unicode characters that would render incorrectly with ASCII equivalents.
/// For example: × → x, ã → a, — → -
pub fn sanitize_for_pdf(s: &str) -> String {
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
			'ç' => 'c',
			'Ç' => 'C',
			'ñ' => 'n',
			'Ñ' => 'N',
			'ß' => 's',
			'€' => 'E',
			_ if c.is_ascii() => c,
			_ => '?',
		})
		.collect()
}
