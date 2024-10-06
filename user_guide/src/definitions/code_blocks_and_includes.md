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

{{#include ../refs.md}}
