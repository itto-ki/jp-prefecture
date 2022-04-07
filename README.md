[![Crates.io](https://img.shields.io/crates/v/jp-prefecture.svg)](https://crates.io/crates/jp-prefecture)
[![Docs.rs](https://docs.rs/jp-prefecture/badge.svg)](https://docs.rs/jp-prefecture)
![Crates.io](https://img.shields.io/crates/l/jp-prefecture)

# jp-prefecture

jp-prefecture is an useful crate for using Japanese prefectures

# Getting Started

Crate has to be added as dependency to `Cargo.toml`

```toml
[dependencies]
jp-prefecture = "0.1.0"
```

and imported to the scope of a block where it's begin called

```rust
use jp_prefecture::prefectures;
```

# Examples

```rust
use jp_prefecture::prefectures;

let tokyo = prefectures::find_by_kanji("東京都");
println!("{:?}", tokyo); // => Some(Prefecture::Tokyo)
println!("{:?}", tokyo.unwrap().kanji()); // => "東京都"
println!("{:?}", tokyo.unwrap().kanji_short()); // => "東京"
println!("{:?}", tokyo.unwrap().english()); // => "tokyo"

let tokyo = prefectures::find_by_hiragana("東京県"); // uhmmmm...
println!("{:?}", tokyo); // => None
```
