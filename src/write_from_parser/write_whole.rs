//! Functions that read from a Markdown parser and write the whole
//! content to various outputs / formats
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Event;
use pulldown_cmark::html;

// HTML

/// Read from a Markdown parser and write HTML to standard output.
///
/// parser: Markdown parser.
#[allow(dead_code)]
pub(crate) fn write_html_to_stdout<'a, I>(parser: I) -> std::io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to stdout. This could also be anything implementing the
    // `Write` trait e.g., a file or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n")?;
    html::write_html_io(&mut handle, parser)?;
    Ok(())
}

/// Read from a Markdown parser and write HTML to bytes.
///
/// parser: Markdown parser.
#[allow(dead_code)]
pub(crate) fn write_html_to_bytes<'a, I>(parser: I) -> Result<Vec<u8>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut bytes = Vec::new();
    // A Cursor wraps an in-memory buffer
    html::write_html_io(std::io::Cursor::new(&mut bytes), parser)?;
    Ok(bytes)
}

/// Read from a Markdown parser and write HTML to string.
///
/// parser: Markdown parser.
#[allow(dead_code)]
pub(crate) fn write_html_to_string<'a, I>(parser: I) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to a new String buffer
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

