//! Operations on Markdown that are not link- or reference-related

mod extract_code;
mod remove_includes;
mod replace_include;

#[doc(inline)]
pub use extract_code::*;
#[doc(inline)]
pub use remove_includes::*;
#[doc(inline)]
pub use replace_include::*;
