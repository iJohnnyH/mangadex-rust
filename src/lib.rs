//! Mangadex_rust is a wrapper for the MangadexV5 API
//! It (will) support the majority of operations described
//! at https://api.mangadex.org/
//!
//! Mangadex_rust uses the async HTTP client reqwest.
//!
//! Example:
//!     Query for a given manga:
//!     
//!

pub mod client;
pub mod session;
pub mod types;

pub const BASE_URI: &str = "https://api.mangadex.org";
