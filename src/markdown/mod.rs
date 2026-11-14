//! Operations on Markdown that are not link- or reference-related
//!
//! TODO Reviewed ??-??-2026.

/// Implements extract code behavior for the markdown component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
pub mod extract_code;
/// Implements remove includes behavior for the markdown component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
pub mod remove_includes;
/// Implements replace include behavior for the markdown component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
pub mod replace_include;
/// Implements rust blocks behavior for the markdown component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod rust_blocks;

#[doc(inline)]
pub use extract_code::*;
#[doc(inline)]
pub use remove_includes::*;
#[doc(inline)]
pub use replace_include::*;
