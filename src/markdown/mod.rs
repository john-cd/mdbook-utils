//! Operations on Markdown that are not link- or reference-related

pub mod extract_code;
pub mod remove_includes;
pub mod replace_include;

#[doc(inline)]
pub use extract_code::*;
#[doc(inline)]
pub use remove_includes::*;
#[doc(inline)]
pub use replace_include::*;
