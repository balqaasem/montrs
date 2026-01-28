use std::path::Path;
use thiserror::Error;

pub mod comments;
pub mod config;
pub mod macro_fmt;

pub use config::FormatterSettings;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Parse error: {0}")]
    Parse(#[from] syn::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Macro format error: {0}")]
    Macro(String),
}

/// Formats a single Rust file.
pub fn format_file(path: impl AsRef<Path>, settings: &FormatterSettings) -> Result<String, FormatError> {
    let source = std::fs::read_to_string(path)?;
    format_source(&source, settings)
}

/// Formats a Rust source string.
pub fn format_source(source: &str, settings: &FormatterSettings) -> Result<String, FormatError> {
    // 1. Extract comments
    let (source_rope, comments) = comments::extract_comments(source);

    // 2. Parse the file into a syn::File
    let file = syn::parse_file(source)?;

    // 3. Collect and format view! macros
    let mut edits = Vec::new();
    macro_fmt::collect_and_format_macros(&file, &source_rope, settings, &mut edits)?;

    // 4. Format the file using prettyplease
    // Note: prettyplease will format the macros too, but we will overwrite them
    let formatted = prettyplease::unparse(&file);
    
    // 5. Re-apply macro edits to the formatted output
    // This is tricky because prettyplease changed the spans.
    // Instead, we should have formatted the macros and then used them.
    // For now, let's stick to the pipeline:
    // If we have macros, we need to find them in the formatted output.
    
    // Simplified: re-parse the formatted output and find macros again to apply edits
    let formatted_ast = syn::parse_file(&formatted)?;
    let mut formatted_rope = crop::Rope::from(formatted);

    let mut formatted_edits = Vec::new();
    macro_fmt::collect_and_format_macros(&formatted_ast, &formatted_rope, settings, &mut formatted_edits)?;

    macro_fmt::apply_edits(&mut formatted_rope, formatted_edits);

    // 6. Re-insert comments
    let final_source = comments::reinsert_comments(&formatted_rope.to_string(), comments);

    Ok(final_source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatterSettings;

    #[test]
    fn test_format_basic_rust() {
        let source = "fn main() { let x = 1; }";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("fn main() {"));
        assert!(formatted.contains("let x = 1;"));
    }

    #[test]
    fn test_format_with_comments() {
        let source = "fn main() {\n    // A line comment\n    let x = 1; /* a block comment */\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("// A line comment"));
        assert!(formatted.contains("/* a block comment */"));
    }

    #[test]
    fn test_format_view_macro() {
        let source = "fn main() {\n    view! { <div class=\"test\"><span>\"Hello\"</span></div> };\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("<div class=\"test\">"));
        assert!(formatted.contains("<span>"));
        assert!(formatted.contains("\"Hello\""));
    }
}
