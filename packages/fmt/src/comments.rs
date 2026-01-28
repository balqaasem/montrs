use crop::Rope;
use proc_macro2::LineColumn;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
    pub start: LineColumn,
    pub end: LineColumn,
    pub is_doc: bool,
}

pub type CommentMap = BTreeMap<usize, Vec<Comment>>;

/// Extracts all comments from the source string.
pub fn extract_comments(source: &str) -> (Rope, Vec<Comment>) {
    let mut comments = Vec::new();
    let rope = Rope::from(source);
    
    let mut chars = source.char_indices().peekable();
    let mut line = 1;
    let mut col = 0;

    while let Some((_idx, c)) = chars.next() {
        if c == '\n' {
            line += 1;
            col = 0;
            continue;
        }
        col += 1;

        if c == '/' {
            if let Some(&(_, next_c)) = chars.peek() {
                if next_c == '/' {
                    // Line comment
                    let start = LineColumn { line, column: col - 1 };
                    let mut text = String::from("//");
                    chars.next(); // consume second /
                    col += 1;

                    while let Some((_, c)) = chars.next() {
                        text.push(c);
                        if c == '\n' {
                            line += 1;
                            col = 0;
                            break;
                        }
                        col += 1;
                    }
                    let end = LineColumn { line, column: col };
                    comments.push(Comment {
                        is_doc: text.starts_with("///") || text.starts_with("//!"),
                        text,
                        start,
                        end,
                    });
                } else if next_c == '*' {
                    // Block comment
                    let start = LineColumn { line, column: col - 1 };
                    let mut text = String::from("/*");
                    chars.next(); // consume *
                    col += 1;

                    while let Some((_, c)) = chars.next() {
                        text.push(c);
                        if c == '\n' {
                            line += 1;
                            col = 0;
                        } else {
                            col += 1;
                        }
                        if c == '*' {
                            if let Some(&(_, next_c)) = chars.peek() {
                                if next_c == '/' {
                                    text.push('/');
                                    chars.next();
                                    col += 1;
                                    break;
                                }
                            }
                        }
                    }
                    let end = LineColumn { line, column: col };
                    comments.push(Comment {
                        is_doc: text.starts_with("/**") || text.starts_with("/*!"),
                        text,
                        start,
                        end,
                    });
                }
            }
        }
    }

    (rope, comments)
}

/// Re-inserts comments into the formatted source string.
pub fn reinsert_comments(formatted: &str, comments: Vec<Comment>) -> String {
    if comments.is_empty() {
        return formatted.to_string();
    }

    let mut result = String::new();
    let formatted_lines: Vec<&str> = formatted.lines().collect();
    let mut current_comment_idx = 0;

    // This is a simplified re-insertion. 
    // In the full Span-Gap algorithm, we would match comments to the tokens they were next to.
    // For now, we'll try to keep them on their relative lines if possible, 
    // or at least ensure they aren't lost.
    
    for (i, line) in formatted_lines.iter().enumerate() {
        let line_num = i + 1;
        
        // Check for comments that should be BEFORE or ON this line
        while current_comment_idx < comments.len() && comments[current_comment_idx].start.line <= line_num {
            let comment = &comments[current_comment_idx];
            if !formatted.contains(&comment.text) {
                result.push_str(&comment.text);
                result.push('\n');
            }
            current_comment_idx += 1;
        }
        
        result.push_str(line);
        result.push('\n');
    }

    // Add any remaining comments
    while current_comment_idx < comments.len() {
        let comment = &comments[current_comment_idx];
        if !formatted.contains(&comment.text) {
            result.push_str(&comment.text);
            result.push('\n');
        }
        current_comment_idx += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_line_comments() {
        let source = "let x = 1; // line comment";
        let (_, comments) = extract_comments(source);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text.trim(), "// line comment");
    }

    #[test]
    fn test_extract_block_comments() {
        let source = "let x = 1; /* block \n comment */";
        let (_, comments) = extract_comments(source);
        assert_eq!(comments.len(), 1);
        assert!(comments[0].text.contains("block"));
        assert!(comments[0].text.contains("comment"));
    }

    #[test]
    fn test_reinsert_basic() {
        let formatted = "let x = 1;\n";
        let comments = vec![Comment {
            text: "// comment".to_string(),
            start: LineColumn { line: 1, column: 0 },
            end: LineColumn { line: 1, column: 10 },
            is_doc: false,
        }];
        let result = reinsert_comments(formatted, comments);
        assert!(result.contains("// comment"));
        assert!(result.contains("let x = 1;"));
    }
}

/// Helper to get text between two spans
pub fn get_text_between_spans(source: &Rope, start: LineColumn, end: LineColumn) -> String {
    if start.line > end.line || (start.line == end.line && start.column > end.column) {
        return String::new();
    }

    let mut result = String::new();
    for line_idx in (start.line - 1)..end.line {
        if line_idx < source.line_len() {
            let line = source.line(line_idx);
            let line_str = line.to_string();
            let start_col = if line_idx == start.line - 1 { start.column } else { 0 };
            let end_col = if line_idx == end.line - 1 { end.column } else { line_str.len() };
            
            if start_col < line_str.len() {
                result.push_str(&line_str[start_col..std::cmp::min(end_col, line_str.len())]);
            }
            if line_idx < end.line - 1 {
                result.push('\n');
            }
        }
    }
    result
}
