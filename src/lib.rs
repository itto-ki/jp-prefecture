//! # Overview
//! jp-prefecture is utility library for using Japanese prefectures
//!
//! # Getting Started
//! Crate has to be added as dependency to `Cargo.toml`
//! ```toml
//! [dependencies]
//! jp-prefecture = "0.1.0"
//! ```
//! and imported to the scope of a block where it's begin called
//! ```rust
//! use jp_prefecture::prefectures;
//! ```
//!
//! # Examples
//! ```
//! use jp_prefecture::prefectures::Prefecture;
//!
//! let tokyo = Prefecture::find_by_kanji("東京都");
//! println!("{}", tokyo.unwrap().english()); // => "tokyo"
//!
//! let tokyo = Prefecture::find_by_hiragana("とうきょうけん"); // uhmmmm...
//! println!("{:?}", tokyo); // => None
//! ```

use thiserror;

mod mapping;
pub mod prefectures;

/// Represents errors that can occur using prefectures
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// a prefecture name cannot be parsed
    #[error("Failed conversion to prefecture: {0}")]
    ParseError(String),
}
