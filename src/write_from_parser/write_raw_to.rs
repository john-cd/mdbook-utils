//! Output markdown parsing debugging information.
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;

/// Parse Markdown and write all raw events to e.g. a file
/// for debugging purposes.
///
/// parser: source Markdown parser.
///
/// w: writer / file to write to.
///
/// See <https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html>
/// and <https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Tag.html>.
pub(crate) fn write_raw_to<W>(parser: &mut Parser<'_>, w: &mut W) -> Result<()>
where
    W: Write,
{
    for event in parser.into_iter() {
        match event {
            // Start of a tagged element. Events that are yielded after this
            // event and before its corresponding End event are inside this
            // element. Start and end events are guaranteed to be balanced.
            Event::Start(tag) => {
                writeln!(w, "Start({:?})", tag)?;

                match tag {
                    Tag::Paragraph => {
                        writeln!(w, "Event::Start(Tag::Paragraph)")?;
                    }
                    // A heading. The first field indicates the level of the
                    // heading, the second the fragment identifier, and the
                    // third the classes.
                    Tag::Heading {
                        level,
                        id,
                        classes,
                        attrs,
                    } => {
                        writeln!(
                            w,
                            "Event::Start(Tag::Heading{{ level: {} fragment identifier: {:?} classes: {:?} attributes: {:?} }})",
                            level, id, classes, attrs
                        )?;
                    }
                    // A blockQuote with optional kind (Note, Tip, Important, Warning, Caution).
                    Tag::BlockQuote(k) => {
                        writeln!(w, "Event::Start(Tag::BlockQuote({:?}))", k)?;
                    }
                    // A code block.
                    Tag::CodeBlock(code_block_kind) => {
                        writeln!(
                            w,
                            "Event::Start(Tag::CodeBlock(code_block_kind: {:?} ))",
                            code_block_kind
                        )?;
                    }
                    Tag::DefinitionList => {
                        writeln!(w, "Event::Start(Tag::DefinitionList)")?;
                    }
                    Tag::DefinitionListTitle => {
                        writeln!(w, "Event::Start(Tag::DefinitionListTitle)")?;
                    }
                    Tag::DefinitionListDefinition => {
                        writeln!(w, "Event::Start(DefinitionListDefinition)")?;
                    }
                    // A HTML block.
                    Tag::HtmlBlock => {
                        writeln!(w, "Event::Start(Tag::HtmlBlock)")?;
                    }
                    // A list. If the list is ordered the field indicates the
                    // number of the first item. Contains only list items.
                    Tag::List(ordered_list_first_item_number) => {
                        writeln!(
                            w,
                            "Event::Start(Tag::List( ordered_list_first_item_number: {:?} ))",
                            ordered_list_first_item_number
                        )?;
                    }
                    // A list item.
                    Tag::Item => {
                        writeln!(w, "Event::Start(Tag::Item) (this is a list item)")?;
                    }
                    // A footnote definition. The value contained is the
                    // footnote’s label by which it can be referred to.
                    Tag::FootnoteDefinition(label) => {
                        writeln!(
                            w,
                            "Event::Start(Tag::FootnoteDefinition( label: {} ))",
                            label
                        )?;
                    }
                    // A table. Contains a vector describing the text-alignment
                    // for each of its columns.
                    Tag::Table(column_text_alignment_list) => {
                        writeln!(
                            w,
                            "Event::Start(Tag::Table( column_text_alignment_list: {:?} ))",
                            column_text_alignment_list
                        )?;
                    }
                    // A table header. Contains only TableCells. Note that the
                    // table body starts immediately after the closure of the
                    // TableHead tag. There is no TableBody tag.
                    Tag::TableHead => {
                        writeln!(w, "Event::Start(Tag::TableHead) (contains TableRow tags)")?;
                    }
                    // A table row. Is used both for header rows as body rows.
                    // Contains only TableCells.
                    Tag::TableRow => {
                        writeln!(w, "Event::Start(Tag::TableRow) (contains TableCell tags)")?;
                    }
                    Tag::TableCell => {
                        writeln!(w, "Event::Start(Tag::TableCell) (contains inline tags)")?;
                    }
                    Tag::Emphasis => {
                        writeln!(w, "Event::Start(Tag::Emphasis) (this is a span tag)")?;
                    }
                    Tag::Strong => {
                        writeln!(w, "Event::Start(Tag::Strong) (this is a span tag)")?;
                    }
                    Tag::Strikethrough => {
                        writeln!(w, "Event::Start(Tag::Strikethrough) (this is a span tag)")?;
                    }
                    // A link. The first field is the link type, the second the
                    // destination URL and the third is a title.
                    Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => {
                        writeln!(
                            w,
                            "Event::Start(Tag::Link{{ link_type: {:?} url: {} title: {} id: {} }})",
                            link_type, dest_url, title, id
                        )?;
                    }
                    // An image. The first field is the link type, the second
                    // the destination URL and the third is a title.
                    Tag::Image {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => {
                        writeln!(
                            w,
                            "Event::Start(Tag::Image( link_type: {:?} url: {} title: {} id: {} ))",
                            link_type, dest_url, title, id
                        )?;
                    }
                    // A metadata block.
                    Tag::MetadataBlock(kind) => {
                        writeln!(w, "Event::Start(Tag::MetadataBlock({:?}))", kind)?;
                    }
                    Tag::Superscript => {
                        writeln!(w, "Event::Start(Tag::Superscript)")?;
                    }
                    Tag::Subscript => {
                        writeln!(w, "Event::Start(Tag::Subscript)")?;
                    }
                }
            }
            // End of a tagged element.
            Event::End(tag) => {
                writeln!(w, "Event::End({:?})", tag)?;
            }
            // A text node.
            Event::Text(s) => {
                writeln!(w, "Event::Text({:?})", s)?;
            }
            // An inline code node.
            Event::Code(s) => {
                writeln!(w, "Event::Code({:?})", s)?;
            }
            // An HTML node.
            Event::Html(s) => {
                writeln!(w, "Event::Html({:?})", s)?;
            }
            // An inline HTML node.
            Event::InlineHtml(s) => {
                writeln!(w, "Event::InlineHtml({:?})", s)?;
            }
            // A reference to a footnote with given label, which may or may not
            // be defined by an event with a Tag::FootnoteDefinition tag.
            // Definitions and references to them may occur in any order.
            Event::FootnoteReference(s) => {
                writeln!(w, "Event::FootnoteReference({:?})", s)?;
            }
            // A soft line break.
            Event::SoftBreak => {
                writeln!(w, "Event::SoftBreak")?;
            }
            // A hard line break.
            Event::HardBreak => {
                writeln!(w, "Event::HardBreak")?;
            }
            // A horizontal ruler.
            Event::Rule => {
                writeln!(w, "Event::Rule")?;
            }
            // A task list marker, rendered as a checkbox in HTML. Contains a
            // true when it is checked.
            Event::TaskListMarker(b) => {
                writeln!(w, "Event::TaskListMarker({:?})", b)?;
            }
            // An inline math environment node.
            Event::InlineMath(s) => {
                writeln!(w, "Event::InlineMath({:?})", s)?;
            }
            // A display math environment node.
            Event::DisplayMath(s) => {
                writeln!(w, "Event::DisplayMath({:?})", s)?;
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
