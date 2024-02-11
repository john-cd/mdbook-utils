# Definitions

[`mdbook`](https://rust-lang.github.io/mdBook/) is a command-line tool to create books with [Markdown][markdown]. It is commonly used for Rust user guides, such as the [Rust book](https://doc.rust-lang.org/book/) and the [Rust How-to](https://www.john-cd.com/rust_howto/) book.

## Markdown links and reference definitions

[_Markdown_][markdown] is a lightweight, readable markup language for writing structured documents.

A Markdown _link_ can be an _autolink_, e.g. `<https://example.com>`, an _inline link_ like `[Example](https://example.com)`, or a _reference-style link_: `[The user will see this][thisisthelabel]`.

A reference-style link requires a _reference definition_ with a matching _label_:

~~~markdown
thisisthelabel: https://example.com/
~~~

## Images and badges

_Images_ can be inserted using `![Image alternative text](link/to/image.png)` or, reference-style, `![Image][1]` followed by a _reference definition_ `[1]: <http://url/b.jpg>`.

More details may be found in the [CommonMark](https://commonmark.org/) documentation.

A status _badge_ is a small image that provides at-a-glance information, for example the build status of a code repository. Badges are commonly displayed on GitHub READMEs and inserted in `mdbook` documentation as links to a crate's [docs.rs](https://docs.rs/) documentation, GitHub repo, or [crates.io](https://crates.io/) page. More information about badges may be found in the [awesome-badges](https://github.com/badges/awesome-badges) repo and in the [shields.io](https://shields.io/) documentation.

There is no "badge" concept in the Markdown specification. Badges are simply clickable images e.g. `[ ![image-alt-text](link-to-image) ](link-to-webpage)`.

## Code blocks and includes

Markdown _fenced code blocks_ (we will call them _code examples_ as well) are inserted between two  _code fences_ (e.g. sets of triple backticks), with an optional _info string_ (a.k.a. _attributes_ ) after the first backtick group:

~~~markdown
```rust
fn main() {}
```
~~~

`mdbook` allows [including files](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) into your book via an _include statement_ written as `{{#include file.md}}`. `mdbook` interprets included files as Markdown. Since the include syntax is usually used for inserting code snippets and examples, it is often wrapped with ```

~~~markdown
```rust
{{#include file.rs}}
```
~~~

[markdown]: https://en.wikipedia.org/wiki/Markdown
