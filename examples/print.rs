use jp_prefecture::prefectures::find_by_kanji;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokyo = find_by_kanji("東京都")?;
    println!("{:?}", tokyo); // => Ok(Prefecture::Tokyo)
    println!("{:?}", tokyo.kanji());
    println!("{:?}", tokyo.kanji_short());
    println!("{:?}", tokyo.hiragana());
    println!("{:?}", tokyo.hiragana_short());
    println!("{:?}", tokyo.katakana());
    println!("{:?}", tokyo.katakana_short());
    println!("{:?}", tokyo.english());

    Ok(())
}
