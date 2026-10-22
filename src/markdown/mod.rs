//! Operations on Markdown that are not link- or reference-related
//!
//! TODO Reviewed ??-??-2026

pub mod extract_code;
pub mod remove_includes;
pub mod replace_include;
mod rust_blocks;

#[doc(inline)]
pub use extract_code::*;
#[doc(inline)]
pub use remove_includes::*;
#[doc(inline)]
pub use replace_include::*;
