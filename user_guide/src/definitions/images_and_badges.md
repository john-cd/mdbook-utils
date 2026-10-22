<!-- TODO Reviewed ??-??-2026 -->
## Images and badges

_Images_ can be inserted using `![Image alternative text](link/to/image.png)` or, reference-style, `![Image][1]` followed by a _reference definition_ `[1]: <http://url/b.jpg>`.

More details may be found in the [CommonMark][commonmark] documentation.

A status _badge_ is a small image that provides at-a-glance information, for example the build status of a code repository. Badges are commonly displayed on GitHub READMEs and inserted in `mdbook` documentation as links to a crate's [docs.rs][docs-rs] documentation, GitHub repo, or [crates.io][crates-io] page. More information about badges may be found in the [awesome-badges] repo and in the [shields.io][shields-io] documentation.

There is no "badge" concept in the Markdown specification. Badges are simply clickable images e.g. `[ ![image-alt-text](link-to-image) ](link-to-webpage)`.

`mdbook-utils refdefs generate --badges` creates a flat-style GitHub badge by
default. Add `--badge-style flat-square`, `plastic`, `for-the-badge`, or
`social` to choose another [Shields.io style][shields-io]. Repository name
hyphens and underscores are escaped in the badge path so Shields.io renders
them as literal characters. The default flat style omits the `style` query
parameter, matching Shields.io's default and preserving existing output for
repository names without separators.

For example, a GitHub repository named `my_repo-with-dash` uses this badge
image URL with the flat-square style:

```text
https://img.shields.io/badge/my__repo--with--dash-steelblue?logo=github&style=flat-square
```

{{#include ../refs.md}}
