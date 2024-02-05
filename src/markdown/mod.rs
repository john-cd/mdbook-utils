//! Operations on Markdown that are not link- or reference-related

mod extract_code;
mod remove_includes;
mod replace_include;

pub use extract_code::*;
pub use remove_includes::*;
pub use replace_include::*;
