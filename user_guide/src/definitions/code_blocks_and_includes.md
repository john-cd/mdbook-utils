<!-- TODO Reviewed ??-??-2026 -->
## Code blocks and includes

Markdown _fenced code blocks_ (we will call them _code examples_ as well) are inserted between two  _code fences_ (e.g. sets of triple backticks), with an optional _info string_ (a.k.a. _attributes_ ) after the first backtick group:

~~~markdown
```rust
fn main() {}
```
~~~

`mdbook` allows including files into your book via [_include statements_][mdbook-include]. `mdbook` interprets included files as Markdown. Since the include syntax is usually used for inserting code snippets and examples, it is often wrapped between two sets of backticks.

~~~markdown
```rust
{{# include file.rs}}
```
~~~

An include path is relative to the Markdown file containing the directive.
mdBook also supports selecting a line or line range, or the content between
matching `ANCHOR` and `ANCHOR_END` markers. A `:2:5` selector includes lines
2 through 5, while `:setup` selects the content inside the `setup` anchor.
See the [mdBook include reference][mdbook-include] for the complete syntax.

The `mdbook-utils markdown extract-code-examples` command writes each fenced
Rust block to a deterministic file beneath the requested output directory.
The chapter's source-relative path is retained in the output path, and blocks
are numbered in source order (`block-000.rs`, `block-001.rs`, and so on). This
prevents same-named chapters from overwriting each other's examples. The
companion `replace-code-examples-by-includes` command uses the same mapping to
refer back to those files.

{{#include ../refs.md}}
