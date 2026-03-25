use xrun::text_utils::{decode_html_entities, sanitize_for_pdf};

#[test]
fn test_decode_html_entities_quot() {
	assert_eq!(
		decode_html_entities("say &quot;hello&quot;"),
		"say \"hello\""
	);
}

#[test]
fn test_decode_html_entities_amp() {
	assert_eq!(decode_html_entities("foo &amp; bar"), "foo & bar");
}

#[test]
fn test_decode_html_entities_lt_gt() {
	assert_eq!(decode_html_entities("&lt;div&gt;"), "<div>");
}

#[test]
fn test_decode_html_entities_apos() {
	assert_eq!(decode_html_entities("it&apos;s"), "it's");
}

#[test]
fn test_decode_html_entities_no_entities() {
	assert_eq!(decode_html_entities("plain text"), "plain text");
}

#[test]
fn test_decode_html_entities_multiple() {
	assert_eq!(
		decode_html_entities("&lt;a href=&quot;url&quot;&gt;link&lt;/a&gt;"),
		"<a href=\"url\">link</a>"
	);
}

#[test]
fn test_sanitize_for_pdf_ascii_unchanged() {
	assert_eq!(sanitize_for_pdf("hello world 123"), "hello world 123");
}

#[test]
fn test_sanitize_for_pdf_accented_vowels() {
	assert_eq!(sanitize_for_pdf("ãáàâäå"), "aaaaaa");
	assert_eq!(sanitize_for_pdf("éèêë"), "eeee");
	assert_eq!(sanitize_for_pdf("íìîï"), "iiii");
	assert_eq!(sanitize_for_pdf("óòôöõ"), "ooooo");
	assert_eq!(sanitize_for_pdf("úùûü"), "uuuu");
}

#[test]
fn test_sanitize_for_pdf_special_chars() {
	assert_eq!(sanitize_for_pdf("×"), "x");
	assert_eq!(sanitize_for_pdf("÷"), "/");
	assert_eq!(sanitize_for_pdf("–"), "-");
	assert_eq!(sanitize_for_pdf("—"), "-");
}

#[test]
fn test_sanitize_for_pdf_cedilla_and_tilde_n() {
	assert_eq!(sanitize_for_pdf("ç"), "c");
	assert_eq!(sanitize_for_pdf("Ç"), "C");
	assert_eq!(sanitize_for_pdf("ñ"), "n");
	assert_eq!(sanitize_for_pdf("Ñ"), "N");
}

#[test]
fn test_sanitize_for_pdf_unknown_unicode_becomes_question_mark() {
	assert_eq!(sanitize_for_pdf("日"), "?");
}
