//! Rules to create a reference label and/or a badge URL
//! from a link URL
#![allow(clippy::vec_init_then_push)]

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
            re: r"https://doc.rust-lang.org/(?<lib>std|core)/(?<rest>[^/]+(?:/[^/]+)*?)(?:/|\.html)?",
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
            re: r"https://lib.rs/crates/(?<crate>[^/]+)/?",
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
            re: r"https://doc.rust-lang.org/book/ch\d{2}-\d{2}-(?<item>[^.]+)\.html",
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
            re: r"https://doc.rust-lang.org/rust-by-example/(?:[^/]+/)*(?<last>[^/.]+)(?:\.html)?",
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
            re: r"https://doc.rust-lang.org/cargo/(?<rest>[^/.]+)(?:\.html)?",
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
            re: r"http[s]?://(?<domain>[^/]+)/?",
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
            re: r"http[s]?://(?<domain>[^/]+)/(?:\S+?)/(?<last>[^/]+)(?:/|.html)?",
            label_pattern: "${domain}-${last}",
            ..Rule::default()
        },
    );

    m
});

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    fn get_re(name: &str) -> Regex {
        let rule = GLOBAL_RULES.get(name).unwrap();
        Regex::new(rule.re).unwrap()
    }

    #[test]
    fn test_category_rule() {
        let re = get_re("category");
        let url = "https://crates.io/categories/web-programming::websocket/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["catg"], "web-programming::websocket");
    }

    #[test]
    fn test_crates_io_rule() {
        let re = get_re("crates.io");
        assert!(re.is_match("https://crates.io/"));
        assert!(re.is_match("https://crates.io/crates"));
    }

    #[test]
    fn test_crate_rule() {
        let re = get_re("crate");
        let url = "https://crates.io/crates/smol/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["crate"], "smol");
    }

    #[test]
    fn test_docs_rs_rule() {
        let re = get_re("docs.rs");
        assert!(re.is_match("https://docs.rs/"));
    }

    #[test]
    fn test_documentation_rule() {
        let re = get_re("documentation");
        let url = "https://docs.rs/sqlx/latest/sqlx/struct.Pool.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["crate"], "sqlx");
    }

    #[test]
    fn test_std_rule() {
        let re = get_re("std");
        assert!(re.is_match("https://doc.rust-lang.org/std/"));
    }

    #[test]
    fn test_std_item_documentation_rule() {
        let re = get_re("std item documentation");
        let url = "https://doc.rust-lang.org/std/option/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["lib"], "std");
        assert_eq!(&caps["rest"], "option");
    }

    #[test]
    fn test_lib_rs_rule() {
        let re = get_re("lib.rs");
        assert!(re.is_match("https://lib.rs/"));
    }

    #[test]
    fn test_crate_on_lib_rs_rule() {
        let re = get_re("crate on lib.rs");
        let url = "https://lib.rs/crates/sqlx/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["crate"], "sqlx");
    }

    #[test]
    fn test_github_repo_wiki_rule() {
        let re = get_re("github repo wiki");
        let url = "https://github.com/cross-rs/cross/wiki/Getting-Started";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["owner"], "cross-rs");
        assert_eq!(&caps["repo"], "cross");
    }

    #[test]
    fn test_github_john_cd_rule() {
        let re = get_re("github.com/john-cd");
        let url = "https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["last"], "CONTRIBUTING.md");
    }

    #[test]
    fn test_github_repo_rule() {
        let re = get_re("github repo");
        let url = "https://github.com/john-cd/mdbook-utils";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["owner"], "john-cd");
        assert_eq!(&caps["repo"], "mdbook-utils");
    }

    #[test]
    fn test_github_pages_rule() {
        let re = get_re("github pages");
        let url = "https://rust-lang.github.io/rustup/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["owner"], "rust-lang");
        assert_eq!(&caps["repo"], "rustup");
    }

    #[test]
    fn test_rust_book_rule() {
        let re = get_re("rust book");
        assert!(re.is_match("https://doc.rust-lang.org/book/"));
    }

    #[test]
    fn test_rust_book_item_rule() {
        let re = get_re("rust book item");
        let url = "https://doc.rust-lang.org/book/ch15-01-box.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["item"], "box");
    }

    #[test]
    fn test_rust_reference_rule() {
        let re = get_re("rust reference");
        let url = "https://doc.rust-lang.org/reference/attributes.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["item"], "attributes");
    }

    #[test]
    fn test_rust_by_example_book_rule() {
        let re = get_re("rust by example book");
        assert!(re.is_match("https://doc.rust-lang.org/rust-by-example/"));
    }

    #[test]
    fn test_rust_by_example_chapter_rule() {
        let re = get_re("rust by example chapter");
        let url = "https://doc.rust-lang.org/rust-by-example/mod/visibility.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["last"], "visibility");
    }

    #[test]
    fn test_cargo_book_rule() {
        let re = get_re("cargo book");
        let url = "https://doc.rust-lang.org/cargo/index.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["rest"], "index");
    }

    #[test]
    fn test_website_rule() {
        let re = get_re("website");
        let url = "https://dev.to/";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["domain"], "dev.to");
    }

    #[test]
    fn test_website_page_rule() {
        let re = get_re("website page");
        let url = "https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l";
        assert!(re.is_match(url));
        let caps = re.captures(url).unwrap();
        assert_eq!(&caps["domain"], "dev.to");
        assert_eq!(&caps["last"], "my-terminal-became-more-rusty-4g8l");
    }
}
