<!-- TODO Reviewed ??-??-2026 -->
## Markdown

`mdbook-utils markdown` deals with fenced code blocks and includes:

```txt
Manage code blocks (embedded examples) and includes

Usage: mdbook-utils markdown [OPTIONS] <COMMAND>

Commands:
  extract-code-examples              Copy Rust code examples from the Markdown into .rs files
  replace-code-examples-by-includes  Replace Rust code examples from the Markdown by #include statements
  replace-includes-by-contents       Replace #include statements by the file contents
  remove-includes                    Remove #include statements (and replace them by a hard-coded string)
  generate-categories                Generate a listing of crates.io categories and write to a Markdown file
  generate-crates                    Generate a crate index and write to a Markdown file
  identify-files-not-in-summary      Identify .md files not in SUMMARY.md
  identify-unused-rs-examples        Identify .rs examples not used in Markdown files
  help                               Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

To keep Rust examples in separate source files, run `extract-code-examples`
with the Markdown source directory and an output directory. It writes each
example under a directory derived from the chapter's path, followed by
`block-NNN.rs`. For example, `chapters/intro.md` maps to
`code/chapters/intro.md.d/block-000.rs`. This keeps chapters with the same
filename separate. Running extraction again is safe when the generated files
still match; the command reports a conflict instead of overwriting an
unrelated file.

After extraction, `replace-code-examples-by-includes` replaces the fenced Rust
blocks with mdBook include directives that point to those files. Run
`replace-includes-by-contents` to expand Markdown includes before a tool that
does not process mdBook directives. Include paths resolve from the Markdown
file containing each directive. The expansion command also handles mdBook line
and named-anchor selectors, and leaves `*-refs.md` includes intact.

`identify-files-not-in-summary` reads chapter links from `SUMMARY.md`. Nested
paths are resolved from the Markdown source directory, URL-encoded path
characters are decoded, and query strings or heading fragments do not change
which source file is checked, following [mdBook's SUMMARY.md chapter links][mdbook-summary].
Links that resolve outside the source directory are ignored. A missing
`SUMMARY.md` is an error.

`identify-unused-rs-examples` recognizes the mdBook Rust file helpers
`{{# include path/to/example.rs}}`,
`{{# rustdoc_include path/to/example.rs:section}}`, and
`{{# playground path/to/example.rs editable}}`. Their paths are relative to the
Markdown file containing the helper and must resolve inside the requested code
directory. Other `.rs` text is not treated as a reference, including Rust
string literals and ordinary prose or code that mentions a file path.
Standard-library Rust macros such as `include_str!` are not interpreted by
this Markdown scan.

These commands print the unmatched Markdown or Rust paths. A chapter omitted
from `SUMMARY.md` appears in the first command's output, and a Rust file with
no supported mdBook helper appears in the second command's output.

{{#include ../refs.md}}
