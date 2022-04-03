//! # Overview
//! jp-prefecture is utility library for using Japanese prefectures
//!
//! # Getting Started
//! Crate has to be added as dependency to `Cargo.toml`
//! ```
//! [dependencies]
//! jp-prefecture = "0.1.0"
//! ```
//! and imported to the scope of a block where it's begin called
//! ```
//! use jp_prefecture::prefecture;
//! ```

pub mod prefectures;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
