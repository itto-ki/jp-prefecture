//! # Overview
//! jp-prefecture is an utility library for handling Japanese prefectures.
//!
//! # Getting Started
//! Crate has to be added as dependency to `Cargo.toml`
//! ```toml
//! [dependencies]
//! jp-prefecture = "3.0.0"
//! ```
//! and imported to the scope of a block where it's begin called
//! ```rust
//! use jp_prefecture::prefectures;
//! ```
//!
//! # Examples
//! ```
//! use jp_prefecture::prefectures;
//!
//! let tokyo = prefectures::find_by_kanji("東京都");
//! println!("{:?}", tokyo); // => Ok(Prefecture::Tokyo)
//! println!("{:?}", tokyo.as_ref().unwrap().kanji()); // => "東京都"
//! println!("{:?}", tokyo.as_ref().unwrap().kanji_short()); // => "東京"
//! println!("{:?}", tokyo.as_ref().unwrap().english()); // => "Tokyo"
//!
//! let tokyo = prefectures::find_by_kanji("東京県"); // uhmmmm...
//! println!("{:?}", tokyo); // => Err(Error::InvalidPrefectureName("東京県"))
//! ```

mod mapping;
pub mod prefectures;

/// Enum representing errors related to Japanese prefectures
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The prefecture code cannot be parsed or is invalid
    #[error("Invalid prefecture code: {0}")]
    InvalidPrefectureCode(u32),
    /// The prefecture name cannot be parsed or is invalid
    #[error("Invalid prefecture name: {0}")]
    InvalidPrefectureName(String),
}
