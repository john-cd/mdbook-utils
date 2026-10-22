//! Locate fenced Rust code blocks in Markdown with the `CommonMark` parser.
//!
//! Extraction and replacement both use this scanner, so they always agree on
//! how many blocks a chapter has and on their order.
//!
//! TODO Reviewed ??-??-2026

use std::ops::Range;

use pulldown_cmark::CodeBlockKind;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::TagEnd;

/// A fenced code block whose info string starts with the `rust` language.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct RustBlock {
    /// Code inside the fence, as rendered by the parser (container indentation
    /// removed), including the trailing newline of the last line.
    pub(super) code: String,
    /// Byte range of the opening fence line, without its line ending.
    pub(super) open_fence: Range<usize>,
    /// Byte range of the closing fence line without indentation or line
    /// ending, or `None` if the block is not closed before the end of input.
    pub(super) close_fence: Option<Range<usize>>,
    /// Leading whitespace of the opening fence line, repeated before each
    /// line that replaces the block; `None` if the fence sits inside another
    /// container (such as a block quote) and cannot be rewritten safely.
    pub(super) indent: Option<String>,
    /// Byte range of the whole block, from the opening fence to the end of
    /// the closing fence line.
    pub(super) range: Range<usize>,
}

/// Return whether a fenced block's info string selects Rust.
fn is_rust_info(info: &str) -> bool {
    info.split(|c: char| c == ',' || c.is_whitespace())
        .next()
        .is_some_and(|language| language == "rust")
}

/// Find every fenced Rust code block in `markdown`, in document order.
pub(super) fn find_rust_blocks(markdown: &str) -> Vec<RustBlock> {
    let mut blocks = Vec::new();
    let mut current: Option<(Range<usize>, String)> = None;

    for (event, range) in Parser::new_ext(markdown, Options::empty()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) if is_rust_info(&info) => {
                current = Some((range, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, code)) = current.as_mut() {
                    code.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((range, code)) = current.take() {
                    blocks.push(build_block(markdown, range, code));
                }
            }
            _ => {}
        }
    }
    blocks
}

/// Compute the fence line ranges and indentation of one block.
fn build_block(markdown: &str, range: Range<usize>, code: String) -> RustBlock {
    let end = range.start + markdown[range.clone()].trim_end_matches(['\n', '\r']).len();
    let text = &markdown[range.start..end];
    let open_end = range.start + text.find('\n').unwrap_or(text.len());
    let open_fence = range.start..open_end.min(end);
    let open_line = markdown[open_fence.clone()].trim_end();
    let fence_char = open_line.chars().next().unwrap_or('`');
    let fence_len = open_line.chars().take_while(|&c| c == fence_char).count();

    let close_fence = (open_end < end)
        .then(|| {
            let line_start = markdown[..end].rfind('\n').map_or(0, |i| i + 1);
            let line = markdown[line_start..end].trim_start();
            let trimmed = line.trim_end();
            let is_fence = trimmed.len() >= fence_len && trimmed.chars().all(|c| c == fence_char);
            (line_start > open_end && is_fence)
                .then(|| end - trimmed.len()..end - (line.len() - trimmed.len()))
        })
        .flatten();

    let line_start = markdown[..range.start].rfind('\n').map_or(0, |i| i + 1);
    let prefix = &markdown[line_start..range.start];
    let indent = prefix
        .chars()
        .all(char::is_whitespace)
        .then(|| prefix.to_owned());

    RustBlock {
        code,
        open_fence,
        close_fence,
        indent,
        range: range.start..end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Empty blocks, text between blocks and later blocks are all located.
    #[test]
    fn finds_empty_and_following_blocks() {
        let md = "```rust\n```\n\ntext\n\n```rust\nfn main() {}\n```\n";
        let blocks = find_rust_blocks(md);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].code, "");
        assert_eq!(blocks[1].code, "fn main() {}\n");
        assert_eq!(&md[blocks[0].range.clone()], "```rust\n```");
        assert_eq!(&md[blocks[1].range.clone()], "```rust\nfn main() {}\n```");
    }

    /// Tilde fences, attributes and longer fences are recognized; other
    /// languages and ```` ```rustc ```` are not.
    #[test]
    fn recognizes_fence_variants_and_languages() {
        let md = "~~~rust\na\n~~~\n\n```rust,ignore\nb\n```\n\n````rust\n```\nc\n```\n````\n\n```rustc\nd\n```\n\n```python\ne\n```\n";
        let codes: Vec<_> = find_rust_blocks(md).into_iter().map(|b| b.code).collect();
        assert_eq!(codes, ["a\n", "b\n", "```\nc\n```\n"]);
    }

    /// A fence nested in a list keeps its indentation.
    #[test]
    fn records_indentation_of_list_items() {
        let md = "- item\n\n  ```rust\n  fn f() {}\n  ```\n";
        let block = &find_rust_blocks(md)[0];
        assert_eq!(block.code, "fn f() {}\n");
        assert_eq!(block.indent.as_deref(), Some("  "));
        assert_eq!(&md[block.close_fence.clone().unwrap_or(0..0)], "```");
    }

    /// An unterminated block has no closing fence.
    #[test]
    fn unterminated_block_has_no_closing_fence() {
        let block = &find_rust_blocks("```rust\nfn f() {}\n")[0];
        assert_eq!(block.close_fence, None);
    }
}
