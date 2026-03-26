//! Rules to create a reference label and/or a badge URL
//! from a link URL
#![allow(clippy::vec_init_then_push)]

use regex::Regex;
use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    pub(crate) re: &'a str, // Regex pattern to match the url
    #[allow(dead_code)]
    pub(crate) label_pattern: &'a str, // regex replacement pattern
    pub(crate) badge_url_pattern: &'a str, /* optional pattern to build a
                             * badge link */
}

/// Globally compiled regexes for each rule.
pub(crate) static COMPILED_RULES: Lazy<HashMap<&str, Regex>> = Lazy::new(|| {
    GLOBAL_RULES
        .iter()
        .map(|(name, rule)| {
            (
                *name,
                Regex::new(rule.re).unwrap_or_else(|_| panic!("Invalid regex for rule: {}", name)),
            )
        })
        .collect()
});

// TODO the Regexes need testing
/// All rules that transform a URL to a label or badge URL.
pub(crate) static GLOBAL_RULES: Lazy<HashMap<&str, Rule<'_>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // CATEGORIES
    // [cat-websocket-badge]: https://badge-cache.kominick.com/badge/websocket--x.svg?style=social
    // [cat-websocket]: https://crates.io/categories/web-programming::websocket/
    // Optional query
    m.insert(
        "category",
        Rule {
            re: r"https://crates.io/categories/(?<catg>[^/?]+)/?(?:\?\S+)?",
            label_pattern: "cat-${catg}",
            badge_url_pattern: "https://badge-cache.kominick.com/badge/${catg}--x.svg?style=social",
        },
    );

    // CRATES
    // [crates-io]: https://crates.io/
    m.insert(
        "crates.io",
        Rule {
            re: r"https://crates.io(?:/crates)?/?",
            label_pattern: "crates-io",
            ..Rule::default()
        },
    );

    // [smol-badge]: https://badge-cache.kominick.com/crates/v/smol.svg?label=smol
    // [smol-crate]: https://crates.io/crates/smol/
    m.insert("crate", Rule {
        re: r"https://crates.io/crates/(?<crate>[^/?]+)/?",
        label_pattern: "crate-${crate}",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}",
    });

    // DOCS
    m.insert(
        "docs.rs",
        Rule {
            re: r"https://docs.rs/?",
            label_pattern: "docs-rs",
            ..Rule::default()
        },
    );

    // [sqlx-badge]: https://badge-cache.kominick.com/crates/v/sqlx.svg?label=sqlx
    // [sqlx]: https://docs.rs/sqlx/
    // [actix-web]: https://docs.rs/actix-web/latest/actix_web/
    // [join]: https://docs.rs/rayon/latest/rayon/fn.join.html
    // [spawn-blocking]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
    m.insert("documentation", Rule {
        re: r"https://docs.rs/(?<crate>[^/]+)(?:/(?:latest|[^/]+))?(?<item>(?:/[^/]+)+)?/?(?:index\.html)?",
        label_pattern: "${crate}${item}",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}",
    });

    // STD DOCS
    // [std]: https://doc.rust-lang.org/std/"
    // [std-badge]: https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg
    m.insert(
        "std",
        Rule {
            re: r"https://doc.rust-lang.org/std/?",
            label_pattern: "std",
            badge_url_pattern: "https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg",
        },
    );

    // [std::option::Option]: https://doc.rust-lang.org/std/option/"
    // [std::sync::atomic]: https://doc.rust-lang.org/std/sync/atomic/"
    // [core::cell::OnceCell]: https://doc.rust-lang.org/core/cell/struct.OnceCell.html
    m.insert(
        "std item documentation",
        Rule {
            re: r"https://doc.rust-lang.org/(?<lib>std|core)/(?<rest>\S*?)(?:/|.html)?",
            label_pattern: "${lib}-${rest}",
            ..Rule::default()
        },
    );

    // LIB.RS
    // https://lib.rs/
    m.insert(
        "lib.rs",
        Rule {
            re: r"https://lib.rs/?",
            label_pattern: "lib-rs",
            ..Rule::default()
        },
    );
    // [sqlx-librs]: https://lib.rs/crates/sqlx/
    m.insert(
        "crate on lib.rs",
        Rule {
            re: r"https://lib.rs/crates/(?<crate>\S+?)/?",
            label_pattern: "lib-rs-${crate}",
            ..Rule::default()
        },
    );

    // GITHUB REPO WIKI
    // https://github.com/cross-rs/cross/wiki/Getting-Started
    m.insert(
        "github repo wiki",
        Rule {
            re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)/wiki/(?:\S+?)",
            label_pattern: "${repo}-wiki",
            ..Rule::default()
        },
    );

    // [cross-example-toml]: https://github.com/cross-rs/wiki_assets/blob/main/Configuration/Cross.toml

    // GITHUB
    // https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md
    m.insert(
        "github.com/john-cd",
        Rule {
            re: r"https://github.com/john-cd/rust_howto/(?:\S+?/)*(?<last>\S*)(?:/|.md)?",
            label_pattern: "rust-howto-${last}",
            ..Rule::default()
        },
    );

    // [sqlx-github]: https://github.com/launchbadge/sqlx/
    // https://github.com/amar-laksh/workstation/blob/master/src/main.rs
    m.insert(
        "github repo",
        Rule {
            re: r"https://github.com/(?<owner>[^/]+?)/(?<repo>[^/]+)/?.*",
            label_pattern: "${repo}-github",
            badge_url_pattern: "https://img.shields.io/badge/${repo}-steelblue?logo=github",
        },
    );

    // GITHUB PAGES
    // [rustup-documentation]: https://rust-lang.github.io/rustup/
    // [rustup-command-examples]: https://rust-lang.github.io/rustup/examples.html
    m.insert(
        "github pages",
        Rule {
            re: r"https://(?<owner>[^.]+)\.github\.io/(?<repo>[^/]+)/?.*",
            label_pattern: "${repo}-github-pages",
            badge_url_pattern: "https://img.shields.io/badge/${repo}-red?logo=githubpages",
        },
    );

    //  BOOKS

    // RUST BOOK
    // [rust-book-badge]: https://img.shields.io/badge/Rust_Book-blue?logo=mdbook
    // [rust-book]: https://doc.rust-lang.org/book/
    m.insert(
        "rust book",
        Rule {
            re: r"https://doc.rust-lang.org/book/?",
            label_pattern: "rust-book",
            badge_url_pattern: "https://img.shields.io/badge/Rust_Book-blue?logo=mdbook",
        },
    );

    // [box-rust-book-badge]: https://img.shields.io/badge/Box-blue?logo=mdbook
    // [box-rust-book]: https://doc.rust-lang.org/book/ch15-01-box.html
    m.insert(
        "rust book item",
        Rule {
            re: r"https://doc.rust-lang.org/book/ch\d{2}-\d{2}-(?<item>).html",
            label_pattern: "rust-book-${item}",
            badge_url_pattern: "https://img.shields.io/badge/${item}-blue?logo=mdbook",
        },
    );

    // RUST REFERENCE
    // [object-safe-reference-badge]: https://img.shields.io/badge/Object_Safe_Traits-green?logo=mdbook
    // [object-safe-reference]: https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety
    // [attributes-reference]: https://doc.rust-lang.org/reference/attributes.html
    // [conditional-compilation]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
    m.insert("rust reference", Rule {
        re: r"https://doc.rust-lang.org(?:/nightly)?/reference/(?:(?<chapter>[^/]+)/)?(?<item>[^/]+)\.html(?:#\S+)?",
        label_pattern: "rust-reference-book-${chapter}-${item}",
        badge_url_pattern: "https://img.shields.io/badge/${item}-green?logo=mdbook",
    });

    // RUST BY EXAMPLE
    // [rust-by-example-book-badge]: https://img.shields.io/badge/RBE-violet?logo=mdbook
    // [rust-by-example-book]: https://doc.rust-lang.org/rust-by-example/
    m.insert(
        "rust by example book",
        Rule {
            re: r"https://doc.rust-lang.org/rust-by-example/?",
            label_pattern: "rust-by-example",
            badge_url_pattern: "https://img.shields.io/badge/Rust_by_example-violet?logo=mdbook",
        },
    );

    // [visibility-rules-rust-by-example-badge]: https://img.shields.io/badge/RBE-Visibility_Rules-violet?logo=mdbook
    // [visibility-rules-rust-by-example]: https://doc.rust-lang.org/rust-by-example/mod/visibility.html
    m.insert(
        "rust by example chapter",
        Rule {
            re: r"https://doc.rust-lang.org/rust-by-example/(?:\S+?/)*(?<last>\S*?)(?:.html)?",
            label_pattern: "rust-by-example-${last}",
            badge_url_pattern:
                "https://img.shields.io/badge/Rust_by_example-${last}-violet?logo=mdbook",
        },
    );

    // CARGO BOOK
    // [cargo-book]: https://doc.rust-lang.org/cargo/index.html
    m.insert(
        "cargo book",
        Rule {
            re: r"https://doc.rust-lang.org/cargo/(?<rest>\S+?)(?:.html)?",
            label_pattern: "cargo-book-${rest}",
            badge_url_pattern: "https://img.shields.io/badge/Cargo_Book-${last}-yellow?logo=mdbook",
        },
    );

    // GENERIC
    // TODO: Improve these generic regexes to be more robust and handle various URL
    // structures.
    m.insert(
        "website",
        Rule {
            re: r"https?://(?<domain>[^/]+)/?(?:[?#].*)?$",
            label_pattern: "${domain}-website",
            ..Rule::default()
        },
    );

    // [My terminal became more Rusty Community]: https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l
    // [tokio-glossary]: https://tokio.rs/tokio/glossary
    // [tokio-tutorial]: https://tokio.rs/tokio/tutorial
    m.insert(
        "website page",
        Rule {
            re: r"https?://(?<domain>[^/]+)/(?:.*/)?(?<last>[^/?#.]+)(?:\.html|/)?(?:[?#].*)?$",
            label_pattern: "${domain}-${last}",
            ..Rule::default()
        },
    );

    m
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_global_rules() {
        // By using COMPILED_RULES here, the Lazy block executes exactly once
        // and validates every single regex. This removes the regex compilation
        // loop from the test case body entirely.
        let compiled_rules = &*COMPILED_RULES;

        let category_re = compiled_rules.get("category").unwrap();
        let url = "https://crates.io/categories/web-programming::websocket/";
        assert!(category_re.is_match(url));
        let caps = category_re.captures(url).unwrap();
        assert_eq!(&caps["catg"], "web-programming::websocket");

        let crate_re = compiled_rules.get("crate").unwrap();
        let url = "https://crates.io/crates/smol/";
        assert!(crate_re.is_match(url));
        let caps = crate_re.captures(url).unwrap();
        assert_eq!(&caps["crate"], "smol");

        let doc_re = compiled_rules.get("documentation").unwrap();
        let url = "https://docs.rs/sqlx/latest/sqlx/struct.Pool.html";
        assert!(
            doc_re.is_match(url),
            "documentation rule failed to match {}",
            url
        );
        let caps = doc_re.captures(url).unwrap();
        assert_eq!(&caps["crate"], "sqlx");

        let github_repo_re = compiled_rules.get("github repo").unwrap();
        let url = "https://github.com/john-cd/mdbook-utils";
        assert!(github_repo_re.is_match(url));
        let caps = github_repo_re.captures(url).unwrap();
        assert_eq!(&caps["owner"], "john-cd");
        assert_eq!(&caps["repo"], "mdbook-utils");

        let github_pages_re = compiled_rules.get("github pages").unwrap();
        let url = "https://rust-lang.github.io/rustup/";
        assert!(github_pages_re.is_match(url));
        let caps = github_pages_re.captures(url).unwrap();
        assert_eq!(&caps["owner"], "rust-lang");
        assert_eq!(&caps["repo"], "rustup");

        let website_re = compiled_rules.get("website").unwrap();
        let url = "https://example.com/";
        assert!(website_re.is_match(url));
        let caps = website_re.captures(url).unwrap();
        assert_eq!(&caps["domain"], "example.com");

        let url_no_slash = "http://example.com";
        assert!(website_re.is_match(url_no_slash));
        let caps_no_slash = website_re.captures(url_no_slash).unwrap();
        assert_eq!(&caps_no_slash["domain"], "example.com");

        let url_query = "https://example.com/?id=1#foo";
        assert!(website_re.is_match(url_query));
        let caps_query = website_re.captures(url_query).unwrap();
        assert_eq!(&caps_query["domain"], "example.com");

        // Should not match a URL with a path
        let url_path = "https://example.com/path";
        assert!(!website_re.is_match(url_path));

        let website_page_re = compiled_rules.get("website page").unwrap();
        let url = "https://example.com/foo/bar/baz.html";
        assert!(website_page_re.is_match(url));
        let caps = website_page_re.captures(url).unwrap();
        assert_eq!(&caps["domain"], "example.com");
        assert_eq!(&caps["last"], "baz");

        let url_no_html = "https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l";
        assert!(website_page_re.is_match(url_no_html));
        let caps_no_html = website_page_re.captures(url_no_html).unwrap();
        assert_eq!(&caps_no_html["domain"], "dev.to");
        assert_eq!(&caps_no_html["last"], "my-terminal-became-more-rusty-4g8l");

        let url_trailing = "https://tokio.rs/tokio/tutorial/";
        assert!(website_page_re.is_match(url_trailing));
        let caps_trailing = website_page_re.captures(url_trailing).unwrap();
        assert_eq!(&caps_trailing["domain"], "tokio.rs");
        assert_eq!(&caps_trailing["last"], "tutorial");

        let url_short = "https://tokio.rs/tokio/tutorial";
        assert!(website_page_re.is_match(url_short));
        let caps_short = website_page_re.captures(url_short).unwrap();
        assert_eq!(&caps_short["domain"], "tokio.rs");
        assert_eq!(&caps_short["last"], "tutorial");

        let url_one_segment = "https://example.com/about";
        assert!(website_page_re.is_match(url_one_segment));
        let caps_one_segment = website_page_re.captures(url_one_segment).unwrap();
        assert_eq!(&caps_one_segment["domain"], "example.com");
        assert_eq!(&caps_one_segment["last"], "about");

        let url_query = "https://example.com/foo/bar.html?id=1#baz";
        assert!(website_page_re.is_match(url_query));
        let caps_query = website_page_re.captures(url_query).unwrap();
        assert_eq!(&caps_query["domain"], "example.com");
        assert_eq!(&caps_query["last"], "bar");
    }
}
