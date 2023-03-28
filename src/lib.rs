//! # Overview
//! jp-prefecture is an utility library for using Japanese prefectures
//!
//! # Getting Started
//! Crate has to be added as dependency to `Cargo.toml`
//! ```toml
//! [dependencies]
//! jp-prefecture = "1.0.4"
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
//! println!("{:?}", tokyo); // => Some(Prefecture::Tokyo)
//! println!("{:?}", tokyo.unwrap().kanji()); // => "東京都"
//! println!("{:?}", tokyo.unwrap().kanji_short()); // => "東京"
//! println!("{:?}", tokyo.unwrap().english()); // => "tokyo"
//!
//! let tokyo = prefectures::find_by_kanji("東京県"); // uhmmmm...
//! println!("{:?}", tokyo); // => None
//! ```

mod mapping;
pub mod prefectures;

/// Enum representing errors related to Japanese prefectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error<'a> {
    /// The prefecture code cannot be parsed or is invalid
    #[error("Invalid prefecture code: {0}")]
    InvalidPrefectureCode(u32),
    /// The prefecture name cannot be parsed or is invalid
    #[error("Invalid prefecture name: {0}")]
    InvalidPrefectureName(&'a str),
}
