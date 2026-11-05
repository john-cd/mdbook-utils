//! Rules to create a badge URL from a link URL
//!
//! TODO Reviewed ??-??-2026.

use std::collections::HashMap;

use heck::ToKebabCase as _;
use regex::Regex;

/// Identifies a URL-matching rule used by badge and reference helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RuleKind {
    /// A crates.io category URL.
    Category,
    /// The crates.io home page.
    CratesIo,
    /// A crates.io crate page.
    Crate,
    /// The docs.rs home page.
    DocsRs,
    /// A docs.rs crate or item page.
    Documentation,
    /// The Rust standard-library home page.
    Std,
    /// A Rust standard-library or core item page.
    StdItemDocumentation,
    /// The lib.rs home page.
    LibRs,
    /// A crate page on lib.rs.
    CrateOnLibRs,
    /// A GitHub repository wiki page.
    GithubRepoWiki,
    /// A matching file under john-cd/rust_howto.
    GithubComJohnCd,
    /// A GitHub repository URL.
    GithubRepo,
    /// A GitHub Pages site.
    GithubPages,
    /// The Rust Book home page.
    RustBook,
    /// A Rust Book chapter page.
    RustBookItem,
    /// A Rust Reference page.
    RustReference,
    /// The Rust by Example home page.
    RustByExampleBook,
    /// A Rust by Example chapter page.
    RustByExampleChapter,
    /// A Cargo Book page.
    CargoBook,
    /// A generic website root URL.
    Website,
    /// A page on a generic website.
    WebsitePage,
}

/// A URL-matching rule: a regex plus the patterns derived from a match.
#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    /// Regex pattern that matches the URL.
    pub(crate) re: &'a str, // Regex pattern to match the url
    /// Optional replacement pattern that builds a badge image URL.
    pub(crate) badge_url_pattern: &'a str, /* optional pattern to build a
                                            * badge link */
}

/// Globally compiled regexes for each rule.
pub(crate) static COMPILED_RULES: std::sync::LazyLock<HashMap<RuleKind, Regex>> =
    std::sync::LazyLock::new(|| {
        GLOBAL_RULES
            .iter()
            .map(|(name, rule)| {
                (
                    *name,
                    Regex::new(rule.re)
                        .unwrap_or_else(|_| panic!("Invalid regex for rule: {name:?}")),
                )
            })
            .collect()
    });

/// All rules that transform a URL to a label or badge URL.
pub(crate) static GLOBAL_RULES: std::sync::LazyLock<HashMap<RuleKind, Rule<'_>>> =
    std::sync::LazyLock::new(|| {
        let mut m = HashMap::new();

        // CATEGORIES
        // [cat-websocket-badge]: https://badge-cache.kominick.com/badge/websocket--x.svg?style=social
        // [cat-websocket]: https://crates.io/categories/web-programming::websocket/
        // Optional query
        m.insert(
        RuleKind::Category,
        Rule {
            re: r"https://crates.io/categories/(?<catg>[^/?]+)/?(?:\?\S+)?",
            badge_url_pattern: "https://badge-cache.kominick.com/badge/${catg}--x.svg?style=social",
        },
    );

        // CRATES
        // [crates-io]: https://crates.io/
        m.insert(
            RuleKind::CratesIo,
            Rule {
                re: "https://crates.io(?:/crates)?/?",
                ..Rule::default()
            },
        );

        // [smol-badge]: https://badge-cache.kominick.com/crates/v/smol.svg?label=smol
        // [smol-crate]: https://crates.io/crates/smol/
        m.insert(RuleKind::Crate, Rule {
        re: "https://crates.io/crates/(?<crate>[^/?]+)/?",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}",
    });

        // DOCS
        m.insert(
            RuleKind::DocsRs,
            Rule {
                re: "https://docs.rs/?",
                ..Rule::default()
            },
        );

        // [sqlx-badge]: https://badge-cache.kominick.com/crates/v/sqlx.svg?label=sqlx
        // [sqlx]: https://docs.rs/sqlx/
        // [actix-web]: https://docs.rs/actix-web/latest/actix_web/
        // [join]: https://docs.rs/rayon/latest/rayon/fn.join.html
        // [spawn-blocking]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
        m.insert(RuleKind::Documentation, Rule {
        re: r"https://docs.rs/(?<crate>[^/]+)(?:/(?:latest|[^/]+))?(?<item>(?:/[^/]+)+)?/?(?:index\.html)?",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}",
    });

        // STD DOCS
        // [std]: https://doc.rust-lang.org/std/"
        // [std-badge]: https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg
        m.insert(
            RuleKind::Std,
            Rule {
                re: "https://doc.rust-lang.org/std/?",
                badge_url_pattern: "https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg",
            },
        );

        // [std::option::Option]: https://doc.rust-lang.org/std/option/"
        // [std::sync::atomic]: https://doc.rust-lang.org/std/sync/atomic/"
        // [core::cell::OnceCell]: https://doc.rust-lang.org/core/cell/struct.OnceCell.html
        m.insert(
        RuleKind::StdItemDocumentation,
        Rule {
            re: r"https://doc.rust-lang.org/(?<lib>std|core)/(?<rest>[^/?#]+?(?:/[^/?#]+?)*?)(?:/|\.html|[?#]|$)(?:[?#].*)?$",
            ..Rule::default()
        },
    );

        // LIB.RS
        // https://lib.rs/
        m.insert(
            RuleKind::LibRs,
            Rule {
                re: "https://lib.rs/?",
                ..Rule::default()
            },
        );
        // [sqlx-librs]: https://lib.rs/crates/sqlx/
        m.insert(
            RuleKind::CrateOnLibRs,
            Rule {
                re: "https://lib.rs/crates/(?<crate>[^/]+)/?",
                ..Rule::default()
            },
        );

        // GITHUB REPO WIKI
        // https://github.com/cross-rs/cross/wiki/Getting-Started
        m.insert(
            RuleKind::GithubRepoWiki,
            Rule {
                re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)/wiki/(?:\S+?)",
                ..Rule::default()
            },
        );

        // [cross-example-toml]: https://github.com/cross-rs/wiki_assets/blob/main/Configuration/Cross.toml

        // GITHUB
        // https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md
        m.insert(
            RuleKind::GithubComJohnCd,
            Rule {
                re: r"https://github.com/john-cd/rust_howto/(?:\S+?/)*(?<last>\S*)(?:/|.md)?",
                ..Rule::default()
            },
        );

        // [sqlx-github]: https://github.com/launchbadge/sqlx/
        // https://github.com/amar-laksh/workstation/blob/master/src/main.rs
        m.insert(
            RuleKind::GithubRepo,
            Rule {
                re: "https://github.com/(?<owner>[^/]+?)/(?<repo>[^/]+)/?.*",
                badge_url_pattern: "https://img.shields.io/badge/${repo}-steelblue?logo=github",
            },
        );

        // GITHUB PAGES
        // [rustup-documentation]: https://rust-lang.github.io/rustup/
        // [rustup-command-examples]: https://rust-lang.github.io/rustup/examples.html
        m.insert(
            RuleKind::GithubPages,
            Rule {
                re: r"https://(?<owner>[^.]+)\.github\.io/(?<repo>[^/]+)/?.*",
                badge_url_pattern: "https://img.shields.io/badge/${repo}-red?logo=githubpages",
            },
        );

        //  BOOKS

        // RUST BOOK
        // [rust-book-badge]: https://img.shields.io/badge/Rust_Book-blue?logo=mdbook
        // [rust-book]: https://doc.rust-lang.org/book/
        m.insert(
            RuleKind::RustBook,
            Rule {
                re: "https://doc.rust-lang.org/book/?",
                badge_url_pattern: "https://img.shields.io/badge/Rust_Book-blue?logo=mdbook",
            },
        );

        // [box-rust-book-badge]: https://img.shields.io/badge/Box-blue?logo=mdbook
        // [box-rust-book]: https://doc.rust-lang.org/book/ch15-01-box.html
        m.insert(
            RuleKind::RustBookItem,
            Rule {
                re: r"https://doc.rust-lang.org/book/ch\d{2}-\d{2}-(?<item>[^.]+)\.html",
                badge_url_pattern: "https://img.shields.io/badge/${item}-blue?logo=mdbook",
            },
        );

        // RUST REFERENCE
        // [object-safe-reference-badge]: https://img.shields.io/badge/Object_Safe_Traits-green?logo=mdbook
        // [object-safe-reference]: https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety
        // [attributes-reference]: https://doc.rust-lang.org/reference/attributes.html
        // [conditional-compilation]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
        m.insert(RuleKind::RustReference, Rule {
        re: r"https://doc.rust-lang.org(?:/nightly)?/reference/(?:(?<chapter>[^/]+)/)?(?<item>[^/]+)\.html(?:#\S+)?",
        badge_url_pattern: "https://img.shields.io/badge/${item}-green?logo=mdbook",
    });

        // RUST BY EXAMPLE
        // [rust-by-example-book-badge]: https://img.shields.io/badge/RBE-violet?logo=mdbook
        // [rust-by-example-book]: https://doc.rust-lang.org/rust-by-example/
        m.insert(
        RuleKind::RustByExampleBook,
        Rule {
            re: "https://doc.rust-lang.org/rust-by-example/?",
            badge_url_pattern: "https://img.shields.io/badge/Rust_by_example-violet?logo=mdbook",
        },
    );

        // [visibility-rules-rust-by-example-badge]: https://img.shields.io/badge/RBE-Visibility_Rules-violet?logo=mdbook
        // [visibility-rules-rust-by-example]: https://doc.rust-lang.org/rust-by-example/mod/visibility.html
        m.insert(
        RuleKind::RustByExampleChapter,
        Rule {
            re: r"https://doc.rust-lang.org/rust-by-example/(?:[^/]+/)*(?<last>[^/.]+)(?:\.html)?",
            badge_url_pattern:
                "https://img.shields.io/badge/Rust_by_example-${last}-violet?logo=mdbook",
        },
    );

        // CARGO BOOK
        // [cargo-book]: https://doc.rust-lang.org/cargo/index.html
        m.insert(
        RuleKind::CargoBook,
        Rule {
            re: r"https://doc.rust-lang.org/cargo/(?<rest>[^/.]+)(?:\.html)?",
            badge_url_pattern: "https://img.shields.io/badge/Cargo_Book-${rest}-yellow?logo=mdbook",
        },
    );

        // GENERIC
        m.insert(
            RuleKind::Website,
            Rule {
                re: r"^https?://(?<domain>[^/?#\s]+)/?(?:[?#][^\s]*)?$",
                ..Rule::default()
            },
        );

        // [My terminal became more Rusty Community]: https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l
        // [tokio-glossary]: https://tokio.rs/tokio/glossary
        // [tokio-tutorial]: https://tokio.rs/tokio/tutorial
        m.insert(
        RuleKind::WebsitePage,
        Rule {
            re: r"^https?://(?<domain>[^/?#\s]+)/(?:[^\s?#]*/)*(?<last>[^/?#\s]+?)(?:\.html|/)?(?:[?#][^\s]*)?$",
            ..Rule::default()
        },
    );

        m
    });

/// Suggests a stable reference label from a recognized URL pattern.
///
/// Known documentation and repository URLs use their crate, repository, or
/// page identity. Other HTTP URLs use the last path segment, then the host.
pub(crate) fn suggest_label_from_url(url: &str) -> Option<String> {
    let candidates = [
        (RuleKind::Category, "catg"),
        (RuleKind::Crate, "crate"),
        (RuleKind::Documentation, "crate"),
        (RuleKind::StdItemDocumentation, "rest"),
        (RuleKind::CrateOnLibRs, "crate"),
        (RuleKind::GithubRepoWiki, "repo"),
        (RuleKind::GithubComJohnCd, "last"),
        (RuleKind::GithubRepo, "repo"),
        (RuleKind::GithubPages, "repo"),
        (RuleKind::RustBookItem, "item"),
        (RuleKind::RustReference, "item"),
        (RuleKind::RustByExampleChapter, "last"),
        (RuleKind::CargoBook, "rest"),
        (RuleKind::WebsitePage, "last"),
    ];

    for (kind, capture) in candidates {
        let Some(rule) = COMPILED_RULES.get(&kind) else {
            continue;
        };
        let Some(captures) = rule.captures(url) else {
            continue;
        };
        let Some(value) = captures.name(capture).map(|value| value.as_str()) else {
            continue;
        };
        let label = value.rsplit('/').next().unwrap_or_default().to_kebab_case();
        if !label.is_empty() {
            return Some(label);
        }
    }

    let parsed = url::Url::parse(url).ok()?;
    let host = parsed.host_str()?;
    let value = parsed
        .path_segments()?
        .rfind(|segment| !segment.is_empty())
        .unwrap_or(host)
        .trim_end_matches(".html");
    let label = value.to_kebab_case();
    (!label.is_empty()).then_some(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns the compiled matching rule named by `name`.
    fn get_re(name: RuleKind) -> &'static Regex {
        COMPILED_RULES.get(&name).expect("rule kind should exist")
    }

    /// Every rule compiles, and the rule looked up by name at run time exists,
    /// so the string-keyed lookups cannot panic.
    #[test]
    fn every_rule_compiles_and_the_github_repo_rule_exists() {
        assert_eq!(COMPILED_RULES.len(), GLOBAL_RULES.len());
        assert!(GLOBAL_RULES.contains_key(&RuleKind::GithubRepo));
        assert!(COMPILED_RULES.contains_key(&RuleKind::GithubRepo));
    }

    /// Recognized crates.io, docs.rs, and GitHub URLs use their primary names.
    #[test]
    fn suggests_labels_from_known_url_patterns() {
        assert_eq!(
            suggest_label_from_url("https://crates.io/crates/serde"),
            Some("serde".to_owned())
        );
        assert_eq!(
            suggest_label_from_url("https://docs.rs/tokio/latest/tokio/task/index.html"),
            Some("tokio".to_owned())
        );
        assert_eq!(
            suggest_label_from_url("https://github.com/launchbadge/sqlx"),
            Some("sqlx".to_owned())
        );
    }

    /// Generic URL labels use the final page segment or the host at the root.
    #[test]
    fn suggests_labels_from_generic_urls() {
        assert_eq!(
            suggest_label_from_url("https://example.com/guide.html?lang=en#start"),
            Some("guide".to_owned())
        );
        assert_eq!(
            suggest_label_from_url("https://example.com/"),
            Some("example-com".to_owned())
        );
        assert_eq!(suggest_label_from_url("relative/path"), None);
    }

    /// Matches a crates.io category URL and captures its name.
    #[test]
    fn test_category_rule() {
        let re = get_re(RuleKind::Category);
        let url = "https://crates.io/categories/web-programming::websocket/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["catg"], "web-programming::websocket");
    }

    /// Matches the crates.io home and crates pages.
    #[test]
    fn test_crates_io_rule() {
        let re = get_re(RuleKind::CratesIo);
        assert!(re.is_match("https://crates.io/"));
        assert!(re.is_match("https://crates.io/crates"));
    }

    /// Matches a crates.io crate URL and captures the crate name.
    #[test]
    fn test_crate_rule() {
        let re = get_re(RuleKind::Crate);
        let url = "https://crates.io/crates/smol/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["crate"], "smol");
    }

    /// Matches the docs.rs home page.
    #[test]
    fn test_docs_rs_rule() {
        let re = get_re(RuleKind::DocsRs);
        assert!(re.is_match("https://docs.rs/"));
    }

    /// Matches a docs.rs item page.
    #[test]
    fn test_documentation_rule() {
        let re = get_re(RuleKind::Documentation);
        let url = "https://docs.rs/sqlx/latest/sqlx/struct.Pool.html";
        assert!(re.is_match(url), "documentation rule failed to match {url}");
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["crate"], "sqlx");

        let url_version = "https://docs.rs/sqlx/0.7.2/sqlx/struct.Pool.html";
        assert!(re.is_match(url_version));
        let caps = re.captures(url_version).expect("input should parse");
        assert_eq!(&caps["crate"], "sqlx");
    }

    /// Matches the standard library documentation home.
    #[test]
    fn test_std_rule() {
        let re = get_re(RuleKind::Std);
        assert!(re.is_match("https://doc.rust-lang.org/std/"));
    }

    /// Matches a standard library item page and captures its path.
    #[test]
    fn test_std_item_documentation_rule() {
        let re = get_re(RuleKind::StdItemDocumentation);
        let url = "https://doc.rust-lang.org/std/option/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["lib"], "std");
        assert_eq!(&caps["rest"], "option");

        let url_core = "https://doc.rust-lang.org/core/cell/struct.OnceCell.html";
        assert!(re.is_match(url_core));
        let caps = re.captures(url_core).expect("input should parse");
        assert_eq!(&caps["lib"], "core");
        assert_eq!(&caps["rest"], "cell/struct.OnceCell");

        let url_core_item_method =
            "https://doc.rust-lang.org/core/cell/struct.OnceCell.html#method.get";
        let caps = re
            .captures(url_core_item_method)
            .expect("input should parse");
        assert_eq!(&caps["rest"], "cell/struct.OnceCell");
    }

    /// Matches the lib.rs home page.
    #[test]
    fn test_lib_rs_rule() {
        let re = get_re(RuleKind::LibRs);
        assert!(re.is_match("https://lib.rs/"));
    }

    /// Matches a crate page on lib.rs.
    #[test]
    fn test_crate_on_lib_rs_rule() {
        let re = get_re(RuleKind::CrateOnLibRs);
        let url = "https://lib.rs/crates/sqlx/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["crate"], "sqlx");
    }

    /// Matches a GitHub wiki page and captures owner and repository.
    #[test]
    fn test_github_repo_wiki_rule() {
        let re = get_re(RuleKind::GithubRepoWiki);
        let url = "https://github.com/cross-rs/cross/wiki/Getting-Started";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["owner"], "cross-rs");
        assert_eq!(&caps["repo"], "cross");
    }

    /// Matches a file in a john-cd repository.
    #[test]
    fn test_github_john_cd_rule() {
        let re = get_re(RuleKind::GithubComJohnCd);
        let url = "https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["last"], "CONTRIBUTING.md");
    }

    /// Matches a GitHub repository URL.
    #[test]
    fn test_github_repo_rule() {
        let re = get_re(RuleKind::GithubRepo);
        let url = "https://github.com/john-cd/mdbook-utils";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["owner"], "john-cd");
        assert_eq!(&caps["repo"], "mdbook-utils");

        let url_deep = "https://github.com/amar-laksh/workstation/blob/master/src/main.rs";
        assert!(re.is_match(url_deep));
        let caps = re.captures(url_deep).expect("input should parse");
        assert_eq!(&caps["owner"], "amar-laksh");
        assert_eq!(&caps["repo"], "workstation");
    }

    /// Matches a GitHub Pages site.
    #[test]
    fn test_github_pages_rule() {
        let re = get_re(RuleKind::GithubPages);
        let url = "https://rust-lang.github.io/rustup/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["owner"], "rust-lang");
        assert_eq!(&caps["repo"], "rustup");
    }

    /// Matches the Rust Book home page.
    #[test]
    fn test_rust_book_rule() {
        let re = get_re(RuleKind::RustBook);
        assert!(re.is_match("https://doc.rust-lang.org/book/"));
    }

    /// Matches a Rust Book chapter and captures its name.
    #[test]
    fn test_rust_book_item_rule() {
        let re = get_re(RuleKind::RustBookItem);
        let url = "https://doc.rust-lang.org/book/ch15-01-box.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["item"], "box");
    }

    /// Matches a Rust Reference page.
    #[test]
    fn test_rust_reference_rule() {
        let re = get_re(RuleKind::RustReference);
        let url = "https://doc.rust-lang.org/reference/attributes.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["item"], "attributes");

        let url_nightly =
            "https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety";
        assert!(re.is_match(url_nightly));
        let caps = re.captures(url_nightly).expect("input should parse");
        assert_eq!(caps.name("chapter").map(|m| m.as_str()), Some("items"));
        assert_eq!(&caps["item"], "traits");
    }

    /// Matches the Rust by Example home page.
    #[test]
    fn test_rust_by_example_book_rule() {
        let re = get_re(RuleKind::RustByExampleBook);
        assert!(re.is_match("https://doc.rust-lang.org/rust-by-example/"));
    }

    /// Matches a Rust by Example chapter.
    #[test]
    fn test_rust_by_example_chapter_rule() {
        let re = get_re(RuleKind::RustByExampleChapter);
        let url = "https://doc.rust-lang.org/rust-by-example/mod/visibility.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["last"], "visibility");
    }

    /// Matches a Cargo Book page.
    #[test]
    fn test_cargo_book_rule() {
        let re = get_re(RuleKind::CargoBook);
        let url = "https://doc.rust-lang.org/cargo/index.html";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["rest"], "index");
    }

    /// Matches a bare website domain.
    #[test]
    fn test_website_rule() {
        let re = get_re(RuleKind::Website);
        let url = "https://dev.to/";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["domain"], "dev.to");

        let url_fragment = "https://dev.to/#main";
        assert!(re.is_match(url_fragment));
        let caps = re.captures(url_fragment).expect("input should parse");
        assert_eq!(&caps["domain"], "dev.to");

        let url_example = "https://example.com/";
        assert!(re.is_match(url_example));
        let caps_example = re.captures(url_example).expect("input should parse");
        assert_eq!(&caps_example["domain"], "example.com");

        let url_no_slash = "http://example.com";
        assert!(re.is_match(url_no_slash));
        let caps_no_slash = re.captures(url_no_slash).expect("input should parse");
        assert_eq!(&caps_no_slash["domain"], "example.com");

        let url_query = "https://example.com/?id=1#foo";
        assert!(re.is_match(url_query));
        let caps_query = re.captures(url_query).expect("input should parse");
        assert_eq!(&caps_query["domain"], "example.com");

        let url_without_slash = "https://example.com?id=1#foo";
        let caps_without_slash = re.captures(url_without_slash).expect("input should parse");
        assert_eq!(&caps_without_slash["domain"], "example.com");

        // Should not match a URL with a path
        let url_path = "https://example.com/path";
        assert!(!re.is_match(url_path));
    }

    /// Matches generic root URLs with Unicode hosts and separated
    /// query/fragment data.
    #[test]
    fn test_website_rule_handles_unicode_query_and_fragment() {
        let re = get_re(RuleKind::Website);
        let url = "https://例え.テスト?topic=rust%2Fmarkdown#intro";
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["domain"], "例え.テスト");
    }

    /// Rejects generic root destinations with a missing host or raw whitespace.
    #[test]
    fn test_website_rule_rejects_malformed_destinations() {
        let re = get_re(RuleKind::Website);
        for url in [
            "https://?query=1",
            "https:///path",
            "https://example.com?query=two words",
        ] {
            assert!(!re.is_match(url), "unexpectedly matched {url:?}");
        }
    }

    /// Matches a page on an arbitrary website.
    #[test]
    fn test_website_page_rule() {
        let re = get_re(RuleKind::WebsitePage);
        let url = "https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l";
        assert!(re.is_match(url));
        let caps = re.captures(url).expect("input should parse");
        assert_eq!(&caps["domain"], "dev.to");
        assert_eq!(&caps["last"], "my-terminal-became-more-rusty-4g8l");

        let url_example = "https://example.com/foo/bar/baz.html";
        assert!(re.is_match(url_example));
        let caps_example = re.captures(url_example).expect("input should parse");
        assert_eq!(&caps_example["domain"], "example.com");
        assert_eq!(&caps_example["last"], "baz");

        let url_trailing = "https://tokio.rs/tokio/tutorial/";
        assert!(re.is_match(url_trailing));
        let caps_trailing = re.captures(url_trailing).expect("input should parse");
        assert_eq!(&caps_trailing["domain"], "tokio.rs");
        assert_eq!(&caps_trailing["last"], "tutorial");

        let url_short = "https://tokio.rs/tokio/tutorial";
        assert!(re.is_match(url_short));
        let caps_short = re.captures(url_short).expect("input should parse");
        assert_eq!(&caps_short["domain"], "tokio.rs");
        assert_eq!(&caps_short["last"], "tutorial");

        let url_one_segment = "https://example.com/about";
        assert!(re.is_match(url_one_segment));
        let caps_one_segment = re.captures(url_one_segment).expect("input should parse");
        assert_eq!(&caps_one_segment["domain"], "example.com");
        assert_eq!(&caps_one_segment["last"], "about");

        let url_query_fragment = "https://example.com/foo/bar.html?id=1#baz";
        assert!(re.is_match(url_query_fragment));
        let caps_query_fragment = re.captures(url_query_fragment).expect("input should parse");
        assert_eq!(&caps_query_fragment["domain"], "example.com");
        assert_eq!(&caps_query_fragment["last"], "bar");

        let punctuation = "https://example.com/guide/v1.2/page_(draft)!?lang=ja#section";
        let caps_punctuation = re.captures(punctuation).expect("input should parse");
        assert_eq!(&caps_punctuation["domain"], "example.com");
        assert_eq!(&caps_punctuation["last"], "page_(draft)!");

        let unicode = "https://例え.テスト/雪/%E9%9B%AA";
        let caps_unicode = re.captures(unicode).expect("input should parse");
        assert_eq!(&caps_unicode["domain"], "例え.テスト");
        assert_eq!(&caps_unicode["last"], "%E9%9B%AA");

        let versioned_page = "https://example.com/docs/v1.2";
        let caps_versioned_page = re.captures(versioned_page).expect("input should parse");
        assert_eq!(&caps_versioned_page["last"], "v1.2");
    }

    /// Rejects malformed generic page destinations without a host or with
    /// whitespace.
    #[test]
    fn test_website_page_rule_rejects_malformed_destinations() {
        let re = get_re(RuleKind::WebsitePage);
        for url in [
            "https://?/page",
            "https:///page",
            "https://example.com/path with space",
        ] {
            assert!(!re.is_match(url), "unexpectedly matched {url:?}");
        }
    }
}
