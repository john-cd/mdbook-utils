<!-- TODO Reviewed ??-??-2026 -->
# Public API

The `mdbook-utils` package provides one library and one command-line binary. The library API is useful when another Rust program needs to run the same processing as a CLI command or compose it with its own workflow.

Add the crate to your `Cargo.toml`:

```bash
cargo add mdbook-utils
```

The grouped library operations are under `mdbook_utils::api`. The CLI uses these same functions after its private `src/cli` layer parses arguments, resolves command options, and reports results. The library owns reusable Markdown processing, link and reference generation, file scans, and sitemap generation.

For example, generate the categories index from a Markdown source directory:

```rust,ignore
mdbook_utils::api::generate_categories("src", "categories.md")?;
```

Generate GitHub badges with a selected Shields.io style using
`generate_badges_with_style`. The existing `generate_badges` function keeps
the default flat style:

```rust,ignore
mdbook_utils::api::generate_badges_with_style(
    "src",
    "badge_refs.md",
    mdbook_utils::api::BadgeStyle::FlatSquare,
)?;
```

Library operations return `anyhow::Result`, so callers can propagate path, input, parsing, Cargo, and file output errors. Each operation's rustdoc describes the errors it can return.

The flat `mdbook_utils::function_name` re-exports remain available for compatibility with existing callers. New code should use the grouped `mdbook_utils::api` path. The public `mdbook_utils::markdown` module also exposes the lower-level include and code extraction operations used by the CLI. Internal parsing, filesystem, generation, and link implementation modules remain private. The parser test fixture and hidden development command are not part of the library API.

The current single package already keeps the CLI composition root (`src/main.rs`) separate from reusable library code (`src/lib.rs`). A second binary directory or Cargo workspace would add structure without adding a current CLI or library boundary, so the layout stays as it is.

See the crate's [API reference][mdbook-utils-docs-rs] for function signatures and detailed documentation.

{{#include ./refs.md}}
