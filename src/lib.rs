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

mod mapping;
pub mod prefectures;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
