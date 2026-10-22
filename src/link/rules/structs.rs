// TODO Reviewed ??-??-2026
//! Data type for URL-matching rules.
/// A URL-matching rule: a regex plus the patterns derived from a match.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    /// Regex pattern that matches the URL.
    pub(crate) re: &'a str, // Regex pattern to match the url
    /// Replacement pattern that builds the reference label.
    #[allow(dead_code)]
    pub(crate) label_pattern: &'a str, // regex replacement pattern
    /// Optional replacement pattern that builds a badge image URL.
    pub(crate) badge_url_pattern: &'a str, /* optional pattern to build a
                                            * badge link */
}
