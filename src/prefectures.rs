//! japanese prefectures
//!
//! # Examples
//!
//! ```
//! use jp_prefecture::prefectures::{self, Prefecture};
//!
//! let tokyo = prefectures::find_by_kanji("東京都");
//!
//! assert_eq!(tokyo, Ok(Prefecture::Tokyo));
//! assert_eq!(tokyo.as_ref().unwrap().kanji(), "東京都");
//! assert_eq!(tokyo.as_ref().unwrap().kanji_short(), "東京");
//! assert_eq!(tokyo.as_ref().unwrap().kanji_short(), "東京");
//! assert_eq!(tokyo.as_ref().unwrap().hiragana(), "とうきょうと");
//! assert_eq!(tokyo.as_ref().unwrap().hiragana_short(), "とうきょう");
//! assert_eq!(tokyo.as_ref().unwrap().katakana(), "トウキョウト");
//! assert_eq!(tokyo.as_ref().unwrap().katakana_short(), "トウキョウ");
//! assert_eq!(tokyo.as_ref().unwrap().english(), "Tokyo");
//! ```

use std::collections::HashMap;
use std::str::FromStr;

use crate::mapping::PREFECTURE_MAP;
use crate::Error;

/// A value of japanese prefecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prefecture {
    Hokkaido = 1,
    Aomori = 2,
    Iwate = 3,
    Miyagi = 4,
    Akita = 5,
    Yamagata = 6,
    Fukushima = 7,
    Ibaraki = 8,
    Tochigi = 9,
    Gunma = 10,
    Saitama = 11,
    Chiba = 12,
    Tokyo = 13,
    Kanagawa = 14,
    Niigata = 15,
    Toyama = 16,
    Ishikawa = 17,
    Fukui = 18,
    Yamanashi = 19,
    Nagano = 20,
    Gifu = 21,
    Shizuoka = 22,
    Aichi = 23,
    Mie = 24,
    Shiga = 25,
    Kyoto = 26,
    Osaka = 27,
    Hyogo = 28,
    Nara = 29,
    Wakayama = 30,
    Tottori = 31,
    Shimane = 32,
    Okayama = 33,
    Hiroshima = 34,
    Yamaguchi = 35,
    Tokushima = 36,
    Kagawa = 37,
    Ehime = 38,
    Kochi = 39,
    Fukuoka = 40,
    Saga = 41,
    Nagasaki = 42,
    Kumamoto = 43,
    Oita = 44,
    Miyazaki = 45,
    Kagoshima = 46,
    Okinawa = 47,
}

impl Prefecture {
    /// Returns a prefecture code defined by a JIS X 0401
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.jis_x_0401_code(), 13);
    /// ```
    pub fn jis_x_0401_code(&self) -> u32 {
        *self as u32
    }

    /// Returns a prefecture name in kanji
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.kanji(), "東京都".to_string());
    /// ```
    pub fn kanji(&self) -> String {
        PREFECTURE_MAP
            .get(self)
            .expect("Unexpected error")
            .kanji
            .to_string()
    }

    /// Return a short prefecture name in kanji
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.kanji_short(), "東京".to_string());
    /// ```
    pub fn kanji_short(&self) -> String {
        let kanji = self.kanji();
        let kanji_short = match self {
            Prefecture::Hokkaido => kanji.as_str(),
            Prefecture::Tokyo => kanji.trim_end_matches('都'),
            Prefecture::Kyoto | Prefecture::Osaka => kanji.trim_end_matches('府'),
            _ => kanji.trim_end_matches('県'),
        };
        String::from(kanji_short)
    }

    /// Return a prefecture name in hiragana
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.hiragana(), "とうきょうと".to_string());
    /// ```
    pub fn hiragana(&self) -> String {
        PREFECTURE_MAP
            .get(self)
            .expect("Unexpected error")
            .hiragana
            .to_string()
    }

    /// Return a short prefecture name in hiragana
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.hiragana_short(), "とうきょう".to_string());
    /// ```
    pub fn hiragana_short(&self) -> String {
        let hiragana = self.hiragana();
        let hiragana_short = match self {
            Prefecture::Hokkaido => hiragana.as_str(),
            Prefecture::Tokyo => hiragana.trim_end_matches('と'),
            Prefecture::Kyoto | Prefecture::Osaka => hiragana.trim_end_matches('ふ'),
            _ => hiragana.trim_end_matches("けん"),
        };
        String::from(hiragana_short)
    }

    /// Return a prefecture name in katakana
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.katakana(), "トウキョウト".to_string());
    /// ```
    pub fn katakana(&self) -> String {
        PREFECTURE_MAP
            .get(self)
            .expect("Unexpected error")
            .katakana
            .to_string()
    }

    /// Return a prefecture name in katakana
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.katakana_short(), "トウキョウ".to_string());
    /// ```
    pub fn katakana_short(&self) -> String {
        let katakana = self.katakana();
        let katakana_short = match self {
            Prefecture::Hokkaido => katakana.as_str(),
            Prefecture::Tokyo => katakana.trim_end_matches('ト'),
            Prefecture::Kyoto | Prefecture::Osaka => katakana.trim_end_matches('フ'),
            _ => katakana.trim_end_matches("ケン"),
        };
        String::from(katakana_short)
    }

    /// Return a prefecture name in english
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.english(), "Tokyo");
    /// ```
    pub fn english(&self) -> String {
        let english = PREFECTURE_MAP.get(self).expect("Unexpected error").english;
        let mut chars = english.chars();
        if let Some(fist_char) = chars.next() {
            let capitalized_char = fist_char.to_uppercase().collect::<String>();
            let rest_of_enlish = chars.as_str();
            capitalized_char + rest_of_enlish
        } else {
            // Unreachable
            panic!("Unexpected error");
        }
    }
}

/// Find a prefecture by JIS X 0401 code
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find_by_code(13), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_code(100), Err(Error::InvalidPrefectureCode(100)));
/// ```
pub fn find_by_code(code: u32) -> Result<Prefecture, Error> {
    let mut map: HashMap<u32, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.jis_x_0401_code(), *pref);
    });
    map.get(&code)
        .ok_or_else(|| Error::InvalidPrefectureCode(code))
        .copied()
}

/// Find a prefecture by name in kanji
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find_by_kanji("東京都"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_kanji("東京"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_kanji("東京県"), Err(Error::InvalidPrefectureName("東京県".to_string())));
/// ```
pub fn find_by_kanji<T: AsRef<str> + ToString>(kanji: T) -> Result<Prefecture, Error> {
    let mut map: HashMap<String, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.kanji(), *pref);
        map.insert(pref.kanji_short(), *pref);
    });
    map.get(kanji.as_ref())
        .ok_or_else(|| Error::InvalidPrefectureName(kanji.to_string()))
        .copied()
}

/// Find a prefecture by name in hiragana
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find_by_hiragana("とうきょうと"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_hiragana("とうきょう"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_hiragana("とうきょうけん"), Err(Error::InvalidPrefectureName("とうきょうけん".to_string())));
/// ```
pub fn find_by_hiragana<T: AsRef<str> + ToString>(hiragana: T) -> Result<Prefecture, Error> {
    let mut map: HashMap<String, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.hiragana(), *pref);
        map.insert(pref.hiragana_short(), *pref);
    });
    map.get(hiragana.as_ref())
        .ok_or_else(|| Error::InvalidPrefectureName(hiragana.to_string()))
        .copied()
}

/// Find a prefecture by name in katakana
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find_by_katakana("トウキョウト"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_katakana("トウキョウ"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_katakana("トウキョウケン"), Err(Error::InvalidPrefectureName("トウキョウケン".to_string())));
/// ```
pub fn find_by_katakana<T: AsRef<str> + ToString>(katakana: T) -> Result<Prefecture, Error> {
    let mut map: HashMap<String, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.katakana(), *pref);
        map.insert(pref.katakana_short(), *pref);
    });
    map.get(katakana.as_ref())
        .ok_or_else(|| Error::InvalidPrefectureName(katakana.to_string()))
        .copied()
}

/// Find a prefecture by name in english
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find_by_english("tokyo"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_english("Tokyo"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_english("tOkYo"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find_by_english("tokyo~~~"), Err(Error::InvalidPrefectureName("tokyo~~~".to_string())));
/// ```
pub fn find_by_english<T: AsRef<str> + ToString>(english: T) -> Result<Prefecture, Error> {
    PREFECTURE_MAP
        .iter()
        .find(|(_, data)| data.english == english.as_ref().to_lowercase())
        .map(|(pref, _)| *pref)
        .ok_or_else(|| Error::InvalidPrefectureName(english.to_string()))
}

/// Find a prefecture by name
///
/// # Examples
///
/// ```
/// use jp_prefecture::{prefectures::{self, Prefecture}, Error};
///
/// assert_eq!(prefectures::find("東京都"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("東京"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("とうきょうと"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("とうきょう"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("トウキョウト"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("トウキョウ"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("tokyo"), Ok(Prefecture::Tokyo));
/// assert_eq!(prefectures::find("none"), Err(Error::InvalidPrefectureName("none".to_string())));
/// ```
pub fn find<T: AsRef<str>>(s: T) -> Result<Prefecture, Error> {
    Prefecture::from_str(s.as_ref())
}

impl FromStr for Prefecture {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, Prefecture> = HashMap::new();
        PREFECTURE_MAP.iter().for_each(|(pref, _)| {
            map.insert(pref.kanji(), *pref);
            map.insert(pref.kanji_short(), *pref);
            map.insert(pref.hiragana(), *pref);
            map.insert(pref.hiragana_short(), *pref);
            map.insert(pref.katakana(), *pref);
            map.insert(pref.katakana_short(), *pref);
            map.insert(pref.english().to_lowercase(), *pref);
        });
        map.get(s.to_ascii_lowercase().as_str())
            .copied()
            .ok_or_else(|| Self::Err::InvalidPrefectureName(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Prefecture::Hokkaido => 1)]
    #[test_case(Prefecture::Aomori => 2)]
    #[test_case(Prefecture::Iwate => 3)]
    #[test_case(Prefecture::Miyagi => 4)]
    #[test_case(Prefecture::Akita => 5)]
    #[test_case(Prefecture::Yamagata => 6)]
    #[test_case(Prefecture::Fukushima => 7)]
    #[test_case(Prefecture::Ibaraki => 8)]
    #[test_case(Prefecture::Tochigi => 9)]
    #[test_case(Prefecture::Gunma => 10)]
    #[test_case(Prefecture::Saitama => 11)]
    #[test_case(Prefecture::Chiba => 12)]
    #[test_case(Prefecture::Tokyo => 13)]
    #[test_case(Prefecture::Kanagawa => 14)]
    #[test_case(Prefecture::Niigata => 15)]
    #[test_case(Prefecture::Toyama => 16)]
    #[test_case(Prefecture::Ishikawa => 17)]
    #[test_case(Prefecture::Fukui => 18)]
    #[test_case(Prefecture::Yamanashi => 19)]
    #[test_case(Prefecture::Nagano => 20)]
    #[test_case(Prefecture::Gifu => 21)]
    #[test_case(Prefecture::Shizuoka => 22)]
    #[test_case(Prefecture::Aichi => 23)]
    #[test_case(Prefecture::Mie => 24)]
    #[test_case(Prefecture::Shiga => 25)]
    #[test_case(Prefecture::Kyoto => 26)]
    #[test_case(Prefecture::Osaka => 27)]
    #[test_case(Prefecture::Hyogo => 28)]
    #[test_case(Prefecture::Nara => 29)]
    #[test_case(Prefecture::Wakayama => 30)]
    #[test_case(Prefecture::Tottori => 31)]
    #[test_case(Prefecture::Shimane => 32)]
    #[test_case(Prefecture::Okayama => 33)]
    #[test_case(Prefecture::Hiroshima => 34)]
    #[test_case(Prefecture::Yamaguchi => 35)]
    #[test_case(Prefecture::Tokushima => 36)]
    #[test_case(Prefecture::Kagawa => 37)]
    #[test_case(Prefecture::Ehime => 38)]
    #[test_case(Prefecture::Kochi => 39)]
    #[test_case(Prefecture::Fukuoka => 40)]
    #[test_case(Prefecture::Saga => 41)]
    #[test_case(Prefecture::Nagasaki => 42)]
    #[test_case(Prefecture::Kumamoto => 43)]
    #[test_case(Prefecture::Oita => 44)]
    #[test_case(Prefecture::Miyazaki => 45)]
    #[test_case(Prefecture::Kagoshima => 46)]
    #[test_case(Prefecture::Okinawa => 47)]
    fn jis_x_0401_code_tests(prefecture: Prefecture) -> u32 {
        prefecture.jis_x_0401_code()
    }

    #[test_case(Prefecture::Hokkaido => String::from("北海道"))]
    #[test_case(Prefecture::Aomori => String::from("青森県"))]
    #[test_case(Prefecture::Iwate => String::from("岩手県"))]
    #[test_case(Prefecture::Miyagi => String::from("宮城県"))]
    #[test_case(Prefecture::Akita => String::from("秋田県"))]
    #[test_case(Prefecture::Yamagata => String::from("山形県"))]
    #[test_case(Prefecture::Fukushima => String::from("福島県"))]
    #[test_case(Prefecture::Ibaraki => String::from("茨城県"))]
    #[test_case(Prefecture::Tochigi => String::from("栃木県"))]
    #[test_case(Prefecture::Gunma => String::from("群馬県"))]
    #[test_case(Prefecture::Saitama => String::from("埼玉県"))]
    #[test_case(Prefecture::Chiba => String::from("千葉県"))]
    #[test_case(Prefecture::Tokyo => String::from("東京都"))]
    #[test_case(Prefecture::Kanagawa => String::from("神奈川県"))]
    #[test_case(Prefecture::Niigata => String::from("新潟県"))]
    #[test_case(Prefecture::Toyama => String::from("富山県"))]
    #[test_case(Prefecture::Ishikawa => String::from("石川県"))]
    #[test_case(Prefecture::Fukui => String::from("福井県"))]
    #[test_case(Prefecture::Yamanashi => String::from("山梨県"))]
    #[test_case(Prefecture::Nagano => String::from("長野県"))]
    #[test_case(Prefecture::Gifu => String::from("岐阜県"))]
    #[test_case(Prefecture::Shizuoka => String::from("静岡県"))]
    #[test_case(Prefecture::Aichi => String::from("愛知県"))]
    #[test_case(Prefecture::Mie => String::from("三重県"))]
    #[test_case(Prefecture::Shiga => String::from("滋賀県"))]
    #[test_case(Prefecture::Kyoto => String::from("京都府"))]
    #[test_case(Prefecture::Osaka => String::from("大阪府"))]
    #[test_case(Prefecture::Hyogo => String::from("兵庫県"))]
    #[test_case(Prefecture::Nara => String::from("奈良県"))]
    #[test_case(Prefecture::Wakayama => String::from("和歌山県"))]
    #[test_case(Prefecture::Tottori => String::from("鳥取県"))]
    #[test_case(Prefecture::Shimane => String::from("島根県"))]
    #[test_case(Prefecture::Okayama => String::from("岡山県"))]
    #[test_case(Prefecture::Hiroshima => String::from("広島県"))]
    #[test_case(Prefecture::Yamaguchi => String::from("山口県"))]
    #[test_case(Prefecture::Tokushima => String::from("徳島県"))]
    #[test_case(Prefecture::Kagawa => String::from("香川県"))]
    #[test_case(Prefecture::Ehime => String::from("愛媛県"))]
    #[test_case(Prefecture::Kochi => String::from("高知県"))]
    #[test_case(Prefecture::Fukuoka => String::from("福岡県"))]
    #[test_case(Prefecture::Saga => String::from("佐賀県"))]
    #[test_case(Prefecture::Nagasaki => String::from("長崎県"))]
    #[test_case(Prefecture::Kumamoto => String::from("熊本県"))]
    #[test_case(Prefecture::Oita => String::from("大分県"))]
    #[test_case(Prefecture::Miyazaki => String::from("宮崎県"))]
    #[test_case(Prefecture::Kagoshima => String::from("鹿児島県"))]
    #[test_case(Prefecture::Okinawa => String::from("沖縄県"))]
    fn kanji_tests(prefecture: Prefecture) -> String {
        prefecture.kanji()
    }

    #[test_case(Prefecture::Hokkaido => String::from("北海道"))]
    #[test_case(Prefecture::Aomori => String::from("青森"))]
    #[test_case(Prefecture::Iwate => String::from("岩手"))]
    #[test_case(Prefecture::Miyagi => String::from("宮城"))]
    #[test_case(Prefecture::Akita => String::from("秋田"))]
    #[test_case(Prefecture::Yamagata => String::from("山形"))]
    #[test_case(Prefecture::Fukushima => String::from("福島"))]
    #[test_case(Prefecture::Ibaraki => String::from("茨城"))]
    #[test_case(Prefecture::Tochigi => String::from("栃木"))]
    #[test_case(Prefecture::Gunma => String::from("群馬"))]
    #[test_case(Prefecture::Saitama => String::from("埼玉"))]
    #[test_case(Prefecture::Chiba => String::from("千葉"))]
    #[test_case(Prefecture::Tokyo => String::from("東京"))]
    #[test_case(Prefecture::Kanagawa => String::from("神奈川"))]
    #[test_case(Prefecture::Niigata => String::from("新潟"))]
    #[test_case(Prefecture::Toyama => String::from("富山"))]
    #[test_case(Prefecture::Ishikawa => String::from("石川"))]
    #[test_case(Prefecture::Fukui => String::from("福井"))]
    #[test_case(Prefecture::Yamanashi => String::from("山梨"))]
    #[test_case(Prefecture::Nagano => String::from("長野"))]
    #[test_case(Prefecture::Gifu => String::from("岐阜"))]
    #[test_case(Prefecture::Shizuoka => String::from("静岡"))]
    #[test_case(Prefecture::Aichi => String::from("愛知"))]
    #[test_case(Prefecture::Mie => String::from("三重"))]
    #[test_case(Prefecture::Shiga => String::from("滋賀"))]
    #[test_case(Prefecture::Kyoto => String::from("京都"))]
    #[test_case(Prefecture::Osaka => String::from("大阪"))]
    #[test_case(Prefecture::Hyogo => String::from("兵庫"))]
    #[test_case(Prefecture::Nara => String::from("奈良"))]
    #[test_case(Prefecture::Wakayama => String::from("和歌山"))]
    #[test_case(Prefecture::Tottori => String::from("鳥取"))]
    #[test_case(Prefecture::Shimane => String::from("島根"))]
    #[test_case(Prefecture::Okayama => String::from("岡山"))]
    #[test_case(Prefecture::Hiroshima => String::from("広島"))]
    #[test_case(Prefecture::Yamaguchi => String::from("山口"))]
    #[test_case(Prefecture::Tokushima => String::from("徳島"))]
    #[test_case(Prefecture::Kagawa => String::from("香川"))]
    #[test_case(Prefecture::Ehime => String::from("愛媛"))]
    #[test_case(Prefecture::Kochi => String::from("高知"))]
    #[test_case(Prefecture::Fukuoka => String::from("福岡"))]
    #[test_case(Prefecture::Saga => String::from("佐賀"))]
    #[test_case(Prefecture::Nagasaki => String::from("長崎"))]
    #[test_case(Prefecture::Kumamoto => String::from("熊本"))]
    #[test_case(Prefecture::Oita => String::from("大分"))]
    #[test_case(Prefecture::Miyazaki => String::from("宮崎"))]
    #[test_case(Prefecture::Kagoshima => String::from("鹿児島"))]
    #[test_case(Prefecture::Okinawa => String::from("沖縄"))]
    fn kanji_short_tests(prefecture: Prefecture) -> String {
        prefecture.kanji_short()
    }

    #[test_case(Prefecture::Hokkaido => String::from("ほっかいどう"))]
    #[test_case(Prefecture::Aomori => String::from("あおもりけん"))]
    #[test_case(Prefecture::Iwate => String::from("いわてけん"))]
    #[test_case(Prefecture::Miyagi => String::from("みやぎけん"))]
    #[test_case(Prefecture::Akita => String::from("あきたけん"))]
    #[test_case(Prefecture::Yamagata => String::from("やまがたけん"))]
    #[test_case(Prefecture::Fukushima => String::from("ふくしまけん"))]
    #[test_case(Prefecture::Ibaraki => String::from("いばらきけん"))]
    #[test_case(Prefecture::Tochigi => String::from("とちぎけん"))]
    #[test_case(Prefecture::Gunma => String::from("ぐんまけん"))]
    #[test_case(Prefecture::Saitama => String::from("さいたまけん"))]
    #[test_case(Prefecture::Chiba => String::from("ちばけん"))]
    #[test_case(Prefecture::Tokyo => String::from("とうきょうと"))]
    #[test_case(Prefecture::Kanagawa => String::from("かながわけん"))]
    #[test_case(Prefecture::Niigata => String::from("にいがたけん"))]
    #[test_case(Prefecture::Toyama => String::from("とやまけん"))]
    #[test_case(Prefecture::Ishikawa => String::from("いしかわけん"))]
    #[test_case(Prefecture::Fukui => String::from("ふくいけん"))]
    #[test_case(Prefecture::Yamanashi => String::from("やまなしけん"))]
    #[test_case(Prefecture::Nagano => String::from("ながのけん"))]
    #[test_case(Prefecture::Gifu => String::from("ぎふけん"))]
    #[test_case(Prefecture::Shizuoka => String::from("しずおかけん"))]
    #[test_case(Prefecture::Aichi => String::from("あいちけん"))]
    #[test_case(Prefecture::Mie => String::from("みえけん"))]
    #[test_case(Prefecture::Shiga => String::from("しがけん"))]
    #[test_case(Prefecture::Kyoto => String::from("きょうとふ"))]
    #[test_case(Prefecture::Osaka => String::from("おおさかふ"))]
    #[test_case(Prefecture::Hyogo => String::from("ひょうごけん"))]
    #[test_case(Prefecture::Nara => String::from("ならけん"))]
    #[test_case(Prefecture::Wakayama => String::from("わかやまけん"))]
    #[test_case(Prefecture::Tottori => String::from("とっとりけん"))]
    #[test_case(Prefecture::Shimane => String::from("しまねけん"))]
    #[test_case(Prefecture::Okayama => String::from("おかやまけん"))]
    #[test_case(Prefecture::Hiroshima => String::from("ひろしまけん"))]
    #[test_case(Prefecture::Yamaguchi => String::from("やまぐちけん"))]
    #[test_case(Prefecture::Tokushima => String::from("とくしまけん"))]
    #[test_case(Prefecture::Kagawa => String::from("かがわけん"))]
    #[test_case(Prefecture::Ehime => String::from("えひめけん"))]
    #[test_case(Prefecture::Kochi => String::from("こうちけん"))]
    #[test_case(Prefecture::Fukuoka => String::from("ふくおかけん"))]
    #[test_case(Prefecture::Saga => String::from("さがけん"))]
    #[test_case(Prefecture::Nagasaki => String::from("ながさきけん"))]
    #[test_case(Prefecture::Kumamoto => String::from("くまもとけん"))]
    #[test_case(Prefecture::Oita => String::from("おおいたけん"))]
    #[test_case(Prefecture::Miyazaki => String::from("みやざきけん"))]
    #[test_case(Prefecture::Kagoshima => String::from("かごしまけん"))]
    #[test_case(Prefecture::Okinawa => String::from("おきなわけん"))]
    fn hiragana_tests(prefecture: Prefecture) -> String {
        prefecture.hiragana()
    }

    #[test_case(Prefecture::Hokkaido => String::from("ほっかいどう"))]
    #[test_case(Prefecture::Aomori => String::from("あおもり"))]
    #[test_case(Prefecture::Iwate => String::from("いわて"))]
    #[test_case(Prefecture::Miyagi => String::from("みやぎ"))]
    #[test_case(Prefecture::Akita => String::from("あきた"))]
    #[test_case(Prefecture::Yamagata => String::from("やまがた"))]
    #[test_case(Prefecture::Fukushima => String::from("ふくしま"))]
    #[test_case(Prefecture::Ibaraki => String::from("いばらき"))]
    #[test_case(Prefecture::Tochigi => String::from("とちぎ"))]
    #[test_case(Prefecture::Gunma => String::from("ぐんま"))]
    #[test_case(Prefecture::Saitama => String::from("さいたま"))]
    #[test_case(Prefecture::Chiba => String::from("ちば"))]
    #[test_case(Prefecture::Tokyo => String::from("とうきょう"))]
    #[test_case(Prefecture::Kanagawa => String::from("かながわ"))]
    #[test_case(Prefecture::Niigata => String::from("にいがた"))]
    #[test_case(Prefecture::Toyama => String::from("とやま"))]
    #[test_case(Prefecture::Ishikawa => String::from("いしかわ"))]
    #[test_case(Prefecture::Fukui => String::from("ふくい"))]
    #[test_case(Prefecture::Yamanashi => String::from("やまなし"))]
    #[test_case(Prefecture::Nagano => String::from("ながの"))]
    #[test_case(Prefecture::Gifu => String::from("ぎふ"))]
    #[test_case(Prefecture::Shizuoka => String::from("しずおか"))]
    #[test_case(Prefecture::Aichi => String::from("あいち"))]
    #[test_case(Prefecture::Mie => String::from("みえ"))]
    #[test_case(Prefecture::Shiga => String::from("しが"))]
    #[test_case(Prefecture::Kyoto => String::from("きょうと"))]
    #[test_case(Prefecture::Osaka => String::from("おおさか"))]
    #[test_case(Prefecture::Hyogo => String::from("ひょうご"))]
    #[test_case(Prefecture::Nara => String::from("なら"))]
    #[test_case(Prefecture::Wakayama => String::from("わかやま"))]
    #[test_case(Prefecture::Tottori => String::from("とっとり"))]
    #[test_case(Prefecture::Shimane => String::from("しまね"))]
    #[test_case(Prefecture::Okayama => String::from("おかやま"))]
    #[test_case(Prefecture::Hiroshima => String::from("ひろしま"))]
    #[test_case(Prefecture::Yamaguchi => String::from("やまぐち"))]
    #[test_case(Prefecture::Tokushima => String::from("とくしま"))]
    #[test_case(Prefecture::Kagawa => String::from("かがわ"))]
    #[test_case(Prefecture::Ehime => String::from("えひめ"))]
    #[test_case(Prefecture::Kochi => String::from("こうち"))]
    #[test_case(Prefecture::Fukuoka => String::from("ふくおか"))]
    #[test_case(Prefecture::Saga => String::from("さが"))]
    #[test_case(Prefecture::Nagasaki => String::from("ながさき"))]
    #[test_case(Prefecture::Kumamoto => String::from("くまもと"))]
    #[test_case(Prefecture::Oita => String::from("おおいた"))]
    #[test_case(Prefecture::Miyazaki => String::from("みやざき"))]
    #[test_case(Prefecture::Kagoshima => String::from("かごしま"))]
    #[test_case(Prefecture::Okinawa => String::from("おきなわ"))]
    fn hiragana_short_tests(prefecture: Prefecture) -> String {
        prefecture.hiragana_short()
    }

    #[test_case(Prefecture::Hokkaido => String::from("ホッカイドウ"))]
    #[test_case(Prefecture::Aomori => String::from("アオモリケン"))]
    #[test_case(Prefecture::Iwate => String::from("イワテケン"))]
    #[test_case(Prefecture::Miyagi => String::from("ミヤギケン"))]
    #[test_case(Prefecture::Akita => String::from("アキタケン"))]
    #[test_case(Prefecture::Yamagata => String::from("ヤマガタケン"))]
    #[test_case(Prefecture::Fukushima => String::from("フクシマケン"))]
    #[test_case(Prefecture::Ibaraki => String::from("イバラキケン"))]
    #[test_case(Prefecture::Tochigi => String::from("トチギケン"))]
    #[test_case(Prefecture::Gunma => String::from("グンマケン"))]
    #[test_case(Prefecture::Saitama => String::from("サイタマケン"))]
    #[test_case(Prefecture::Chiba => String::from("チバケン"))]
    #[test_case(Prefecture::Tokyo => String::from("トウキョウト"))]
    #[test_case(Prefecture::Kanagawa => String::from("カナガワケン"))]
    #[test_case(Prefecture::Niigata => String::from("ニイガタケン"))]
    #[test_case(Prefecture::Toyama => String::from("トヤマケン"))]
    #[test_case(Prefecture::Ishikawa => String::from("イシカワケン"))]
    #[test_case(Prefecture::Fukui => String::from("フクイケン"))]
    #[test_case(Prefecture::Yamanashi => String::from("ヤマナシケン"))]
    #[test_case(Prefecture::Nagano => String::from("ナガノケン"))]
    #[test_case(Prefecture::Gifu => String::from("ギフケン"))]
    #[test_case(Prefecture::Shizuoka => String::from("シズオカケン"))]
    #[test_case(Prefecture::Aichi => String::from("アイチケン"))]
    #[test_case(Prefecture::Mie => String::from("ミエケン"))]
    #[test_case(Prefecture::Shiga => String::from("シガケン"))]
    #[test_case(Prefecture::Kyoto => String::from("キョウトフ"))]
    #[test_case(Prefecture::Osaka => String::from("オオサカフ"))]
    #[test_case(Prefecture::Hyogo => String::from("ヒョウゴケン"))]
    #[test_case(Prefecture::Nara => String::from("ナラケン"))]
    #[test_case(Prefecture::Wakayama => String::from("ワカヤマケン"))]
    #[test_case(Prefecture::Tottori => String::from("トットリケン"))]
    #[test_case(Prefecture::Shimane => String::from("シマネケン"))]
    #[test_case(Prefecture::Okayama => String::from("オカヤマケン"))]
    #[test_case(Prefecture::Hiroshima => String::from("ヒロシマケン"))]
    #[test_case(Prefecture::Yamaguchi => String::from("ヤマグチケン"))]
    #[test_case(Prefecture::Tokushima => String::from("トクシマケン"))]
    #[test_case(Prefecture::Kagawa => String::from("カガワケン"))]
    #[test_case(Prefecture::Ehime => String::from("エヒメケン"))]
    #[test_case(Prefecture::Kochi => String::from("コウチケン"))]
    #[test_case(Prefecture::Fukuoka => String::from("フクオカケン"))]
    #[test_case(Prefecture::Saga => String::from("サガケン"))]
    #[test_case(Prefecture::Nagasaki => String::from("ナガサキケン"))]
    #[test_case(Prefecture::Kumamoto => String::from("クマモトケン"))]
    #[test_case(Prefecture::Oita => String::from("オオイタケン"))]
    #[test_case(Prefecture::Miyazaki => String::from("ミヤザキケン"))]
    #[test_case(Prefecture::Kagoshima => String::from("カゴシマケン"))]
    #[test_case(Prefecture::Okinawa => String::from("オキナワケン"))]
    fn katakana_tests(prefecture: Prefecture) -> String {
        prefecture.katakana()
    }

    #[test_case(Prefecture::Hokkaido => String::from("ホッカイドウ"))]
    #[test_case(Prefecture::Aomori => String::from("アオモリ"))]
    #[test_case(Prefecture::Iwate => String::from("イワテ"))]
    #[test_case(Prefecture::Miyagi => String::from("ミヤギ"))]
    #[test_case(Prefecture::Akita => String::from("アキタ"))]
    #[test_case(Prefecture::Yamagata => String::from("ヤマガタ"))]
    #[test_case(Prefecture::Fukushima => String::from("フクシマ"))]
    #[test_case(Prefecture::Ibaraki => String::from("イバラキ"))]
    #[test_case(Prefecture::Tochigi => String::from("トチギ"))]
    #[test_case(Prefecture::Gunma => String::from("グンマ"))]
    #[test_case(Prefecture::Saitama => String::from("サイタマ"))]
    #[test_case(Prefecture::Chiba => String::from("チバ"))]
    #[test_case(Prefecture::Tokyo => String::from("トウキョウ"))]
    #[test_case(Prefecture::Kanagawa => String::from("カナガワ"))]
    #[test_case(Prefecture::Niigata => String::from("ニイガタ"))]
    #[test_case(Prefecture::Toyama => String::from("トヤマ"))]
    #[test_case(Prefecture::Ishikawa => String::from("イシカワ"))]
    #[test_case(Prefecture::Fukui => String::from("フクイ"))]
    #[test_case(Prefecture::Yamanashi => String::from("ヤマナシ"))]
    #[test_case(Prefecture::Nagano => String::from("ナガノ"))]
    #[test_case(Prefecture::Gifu => String::from("ギフ"))]
    #[test_case(Prefecture::Shizuoka => String::from("シズオカ"))]
    #[test_case(Prefecture::Aichi => String::from("アイチ"))]
    #[test_case(Prefecture::Mie => String::from("ミエ"))]
    #[test_case(Prefecture::Shiga => String::from("シガ"))]
    #[test_case(Prefecture::Kyoto => String::from("キョウト"))]
    #[test_case(Prefecture::Osaka => String::from("オオサカ"))]
    #[test_case(Prefecture::Hyogo => String::from("ヒョウゴ"))]
    #[test_case(Prefecture::Nara => String::from("ナラ"))]
    #[test_case(Prefecture::Wakayama => String::from("ワカヤマ"))]
    #[test_case(Prefecture::Tottori => String::from("トットリ"))]
    #[test_case(Prefecture::Shimane => String::from("シマネ"))]
    #[test_case(Prefecture::Okayama => String::from("オカヤマ"))]
    #[test_case(Prefecture::Hiroshima => String::from("ヒロシマ"))]
    #[test_case(Prefecture::Yamaguchi => String::from("ヤマグチ"))]
    #[test_case(Prefecture::Tokushima => String::from("トクシマ"))]
    #[test_case(Prefecture::Kagawa => String::from("カガワ"))]
    #[test_case(Prefecture::Ehime => String::from("エヒメ"))]
    #[test_case(Prefecture::Kochi => String::from("コウチ"))]
    #[test_case(Prefecture::Fukuoka => String::from("フクオカ"))]
    #[test_case(Prefecture::Saga => String::from("サガ"))]
    #[test_case(Prefecture::Nagasaki => String::from("ナガサキ"))]
    #[test_case(Prefecture::Kumamoto => String::from("クマモト"))]
    #[test_case(Prefecture::Oita => String::from("オオイタ"))]
    #[test_case(Prefecture::Miyazaki => String::from("ミヤザキ"))]
    #[test_case(Prefecture::Kagoshima => String::from("カゴシマ"))]
    #[test_case(Prefecture::Okinawa => String::from("オキナワ"))]
    fn katakana_short_tests(prefecture: Prefecture) -> String {
        prefecture.katakana_short()
    }

    #[test_case(Prefecture::Hokkaido => String::from("Hokkaido"))]
    #[test_case(Prefecture::Aomori => String::from("Aomori"))]
    #[test_case(Prefecture::Iwate => String::from("Iwate"))]
    #[test_case(Prefecture::Miyagi => String::from("Miyagi"))]
    #[test_case(Prefecture::Akita => String::from("Akita"))]
    #[test_case(Prefecture::Yamagata => String::from("Yamagata"))]
    #[test_case(Prefecture::Fukushima => String::from("Fukushima"))]
    #[test_case(Prefecture::Ibaraki => String::from("Ibaraki"))]
    #[test_case(Prefecture::Tochigi => String::from("Tochigi"))]
    #[test_case(Prefecture::Gunma => String::from("Gunma"))]
    #[test_case(Prefecture::Saitama => String::from("Saitama"))]
    #[test_case(Prefecture::Chiba => String::from("Chiba"))]
    #[test_case(Prefecture::Tokyo => String::from("Tokyo"))]
    #[test_case(Prefecture::Kanagawa => String::from("Kanagawa"))]
    #[test_case(Prefecture::Niigata => String::from("Niigata"))]
    #[test_case(Prefecture::Toyama => String::from("Toyama"))]
    #[test_case(Prefecture::Ishikawa => String::from("Ishikawa"))]
    #[test_case(Prefecture::Fukui => String::from("Fukui"))]
    #[test_case(Prefecture::Yamanashi => String::from("Yamanashi"))]
    #[test_case(Prefecture::Nagano => String::from("Nagano"))]
    #[test_case(Prefecture::Gifu => String::from("Gifu"))]
    #[test_case(Prefecture::Shizuoka => String::from("Shizuoka"))]
    #[test_case(Prefecture::Aichi => String::from("Aichi"))]
    #[test_case(Prefecture::Mie => String::from("Mie"))]
    #[test_case(Prefecture::Shiga => String::from("Shiga"))]
    #[test_case(Prefecture::Kyoto => String::from("Kyoto"))]
    #[test_case(Prefecture::Osaka => String::from("Osaka"))]
    #[test_case(Prefecture::Hyogo => String::from("Hyogo"))]
    #[test_case(Prefecture::Nara => String::from("Nara"))]
    #[test_case(Prefecture::Wakayama => String::from("Wakayama"))]
    #[test_case(Prefecture::Tottori => String::from("Tottori"))]
    #[test_case(Prefecture::Shimane => String::from("Shimane"))]
    #[test_case(Prefecture::Okayama => String::from("Okayama"))]
    #[test_case(Prefecture::Hiroshima => String::from("Hiroshima"))]
    #[test_case(Prefecture::Yamaguchi => String::from("Yamaguchi"))]
    #[test_case(Prefecture::Tokushima => String::from("Tokushima"))]
    #[test_case(Prefecture::Kagawa => String::from("Kagawa"))]
    #[test_case(Prefecture::Ehime => String::from("Ehime"))]
    #[test_case(Prefecture::Kochi => String::from("Kochi"))]
    #[test_case(Prefecture::Fukuoka => String::from("Fukuoka"))]
    #[test_case(Prefecture::Saga => String::from("Saga"))]
    #[test_case(Prefecture::Nagasaki => String::from("Nagasaki"))]
    #[test_case(Prefecture::Kumamoto => String::from("Kumamoto"))]
    #[test_case(Prefecture::Oita => String::from("Oita"))]
    #[test_case(Prefecture::Miyazaki => String::from("Miyazaki"))]
    #[test_case(Prefecture::Kagoshima => String::from("Kagoshima"))]
    #[test_case(Prefecture::Okinawa => String::from("Okinawa"))]
    fn english_tests(prefecture: Prefecture) -> String {
        prefecture.english()
    }

    #[test_case(1 => Ok(Prefecture::Hokkaido))]
    #[test_case(2 => Ok(Prefecture::Aomori))]
    #[test_case(3 => Ok(Prefecture::Iwate))]
    #[test_case(4 => Ok(Prefecture::Miyagi))]
    #[test_case(5 => Ok(Prefecture::Akita))]
    #[test_case(6 => Ok(Prefecture::Yamagata))]
    #[test_case(7 => Ok(Prefecture::Fukushima))]
    #[test_case(8 => Ok(Prefecture::Ibaraki))]
    #[test_case(9 => Ok(Prefecture::Tochigi))]
    #[test_case(10 => Ok(Prefecture::Gunma))]
    #[test_case(11 => Ok(Prefecture::Saitama))]
    #[test_case(12 => Ok(Prefecture::Chiba))]
    #[test_case(13 => Ok(Prefecture::Tokyo))]
    #[test_case(14 => Ok(Prefecture::Kanagawa))]
    #[test_case(15 => Ok(Prefecture::Niigata))]
    #[test_case(16 => Ok(Prefecture::Toyama))]
    #[test_case(17 => Ok(Prefecture::Ishikawa))]
    #[test_case(18 => Ok(Prefecture::Fukui))]
    #[test_case(19 => Ok(Prefecture::Yamanashi))]
    #[test_case(20 => Ok(Prefecture::Nagano))]
    #[test_case(21 => Ok(Prefecture::Gifu))]
    #[test_case(22 => Ok(Prefecture::Shizuoka))]
    #[test_case(23 => Ok(Prefecture::Aichi))]
    #[test_case(24 => Ok(Prefecture::Mie))]
    #[test_case(25 => Ok(Prefecture::Shiga))]
    #[test_case(26 => Ok(Prefecture::Kyoto))]
    #[test_case(27 => Ok(Prefecture::Osaka))]
    #[test_case(28 => Ok(Prefecture::Hyogo))]
    #[test_case(29 => Ok(Prefecture::Nara))]
    #[test_case(30 => Ok(Prefecture::Wakayama))]
    #[test_case(31 => Ok(Prefecture::Tottori))]
    #[test_case(32 => Ok(Prefecture::Shimane))]
    #[test_case(33 => Ok(Prefecture::Okayama))]
    #[test_case(34 => Ok(Prefecture::Hiroshima))]
    #[test_case(35 => Ok(Prefecture::Yamaguchi))]
    #[test_case(36 => Ok(Prefecture::Tokushima))]
    #[test_case(37 => Ok(Prefecture::Kagawa))]
    #[test_case(38 => Ok(Prefecture::Ehime))]
    #[test_case(39 => Ok(Prefecture::Kochi))]
    #[test_case(40 => Ok(Prefecture::Fukuoka))]
    #[test_case(41 => Ok(Prefecture::Saga))]
    #[test_case(42 => Ok(Prefecture::Nagasaki))]
    #[test_case(43 => Ok(Prefecture::Kumamoto))]
    #[test_case(44 => Ok(Prefecture::Oita))]
    #[test_case(45 => Ok(Prefecture::Miyazaki))]
    #[test_case(46 => Ok(Prefecture::Kagoshima))]
    #[test_case(47 => Ok(Prefecture::Okinawa))]
    #[test_case(48 => Err(Error::InvalidPrefectureCode(48)))]
    fn find_by_code_tests(code: u32) -> Result<Prefecture, Error> {
        find_by_code(code)
    }

    #[test_case("北海道" => Ok(Prefecture::Hokkaido))]
    #[test_case("青森県" => Ok(Prefecture::Aomori))]
    #[test_case("青森" => Ok(Prefecture::Aomori))]
    #[test_case("岩手県" => Ok(Prefecture::Iwate))]
    #[test_case("岩手" => Ok(Prefecture::Iwate))]
    #[test_case("宮城県" => Ok(Prefecture::Miyagi))]
    #[test_case("宮城" => Ok(Prefecture::Miyagi))]
    #[test_case("秋田県" => Ok(Prefecture::Akita))]
    #[test_case("秋田" => Ok(Prefecture::Akita))]
    #[test_case("山形県" => Ok(Prefecture::Yamagata))]
    #[test_case("山形" => Ok(Prefecture::Yamagata))]
    #[test_case("福島県" => Ok(Prefecture::Fukushima))]
    #[test_case("福島" => Ok(Prefecture::Fukushima))]
    #[test_case("茨城県" => Ok(Prefecture::Ibaraki))]
    #[test_case("茨城" => Ok(Prefecture::Ibaraki))]
    #[test_case("栃木県" => Ok(Prefecture::Tochigi))]
    #[test_case("栃木" => Ok(Prefecture::Tochigi))]
    #[test_case("群馬県" => Ok(Prefecture::Gunma))]
    #[test_case("群馬" => Ok(Prefecture::Gunma))]
    #[test_case("埼玉県" => Ok(Prefecture::Saitama))]
    #[test_case("埼玉" => Ok(Prefecture::Saitama))]
    #[test_case("千葉県" => Ok(Prefecture::Chiba))]
    #[test_case("千葉" => Ok(Prefecture::Chiba))]
    #[test_case("東京都" => Ok(Prefecture::Tokyo))]
    #[test_case("東京" => Ok(Prefecture::Tokyo))]
    #[test_case("神奈川県" => Ok(Prefecture::Kanagawa))]
    #[test_case("神奈川" => Ok(Prefecture::Kanagawa))]
    #[test_case("新潟県" => Ok(Prefecture::Niigata))]
    #[test_case("新潟" => Ok(Prefecture::Niigata))]
    #[test_case("富山県" => Ok(Prefecture::Toyama))]
    #[test_case("富山" => Ok(Prefecture::Toyama))]
    #[test_case("石川県" => Ok(Prefecture::Ishikawa))]
    #[test_case("石川" => Ok(Prefecture::Ishikawa))]
    #[test_case("福井県" => Ok(Prefecture::Fukui))]
    #[test_case("福井" => Ok(Prefecture::Fukui))]
    #[test_case("山梨県" => Ok(Prefecture::Yamanashi))]
    #[test_case("山梨" => Ok(Prefecture::Yamanashi))]
    #[test_case("長野県" => Ok(Prefecture::Nagano))]
    #[test_case("長野" => Ok(Prefecture::Nagano))]
    #[test_case("岐阜県" => Ok(Prefecture::Gifu))]
    #[test_case("岐阜" => Ok(Prefecture::Gifu))]
    #[test_case("静岡県" => Ok(Prefecture::Shizuoka))]
    #[test_case("静岡" => Ok(Prefecture::Shizuoka))]
    #[test_case("愛知県" => Ok(Prefecture::Aichi))]
    #[test_case("愛知" => Ok(Prefecture::Aichi))]
    #[test_case("三重県" => Ok(Prefecture::Mie))]
    #[test_case("三重" => Ok(Prefecture::Mie))]
    #[test_case("滋賀県" => Ok(Prefecture::Shiga))]
    #[test_case("滋賀" => Ok(Prefecture::Shiga))]
    #[test_case("京都府" => Ok(Prefecture::Kyoto))]
    #[test_case("京都" => Ok(Prefecture::Kyoto))]
    #[test_case("大阪府" => Ok(Prefecture::Osaka))]
    #[test_case("大阪" => Ok(Prefecture::Osaka))]
    #[test_case("兵庫県" => Ok(Prefecture::Hyogo))]
    #[test_case("兵庫" => Ok(Prefecture::Hyogo))]
    #[test_case("奈良県" => Ok(Prefecture::Nara))]
    #[test_case("奈良" => Ok(Prefecture::Nara))]
    #[test_case("和歌山県" => Ok(Prefecture::Wakayama))]
    #[test_case("和歌山" => Ok(Prefecture::Wakayama))]
    #[test_case("鳥取県" => Ok(Prefecture::Tottori))]
    #[test_case("鳥取" => Ok(Prefecture::Tottori))]
    #[test_case("島根県" => Ok(Prefecture::Shimane))]
    #[test_case("島根" => Ok(Prefecture::Shimane))]
    #[test_case("岡山県" => Ok(Prefecture::Okayama))]
    #[test_case("岡山" => Ok(Prefecture::Okayama))]
    #[test_case("広島県" => Ok(Prefecture::Hiroshima))]
    #[test_case("広島" => Ok(Prefecture::Hiroshima))]
    #[test_case("山口県" => Ok(Prefecture::Yamaguchi))]
    #[test_case("山口" => Ok(Prefecture::Yamaguchi))]
    #[test_case("徳島県" => Ok(Prefecture::Tokushima))]
    #[test_case("徳島" => Ok(Prefecture::Tokushima))]
    #[test_case("香川県" => Ok(Prefecture::Kagawa))]
    #[test_case("香川" => Ok(Prefecture::Kagawa))]
    #[test_case("愛媛県" => Ok(Prefecture::Ehime))]
    #[test_case("愛媛" => Ok(Prefecture::Ehime))]
    #[test_case("高知県" => Ok(Prefecture::Kochi))]
    #[test_case("高知" => Ok(Prefecture::Kochi))]
    #[test_case("福岡県" => Ok(Prefecture::Fukuoka))]
    #[test_case("福岡" => Ok(Prefecture::Fukuoka))]
    #[test_case("佐賀県" => Ok(Prefecture::Saga))]
    #[test_case("佐賀" => Ok(Prefecture::Saga))]
    #[test_case("長崎県" => Ok(Prefecture::Nagasaki))]
    #[test_case("長崎" => Ok(Prefecture::Nagasaki))]
    #[test_case("熊本県" => Ok(Prefecture::Kumamoto))]
    #[test_case("熊本" => Ok(Prefecture::Kumamoto))]
    #[test_case("大分県" => Ok(Prefecture::Oita))]
    #[test_case("大分" => Ok(Prefecture::Oita))]
    #[test_case("宮崎県" => Ok(Prefecture::Miyazaki))]
    #[test_case("宮崎" => Ok(Prefecture::Miyazaki))]
    #[test_case("鹿児島県" => Ok(Prefecture::Kagoshima))]
    #[test_case("鹿児島" => Ok(Prefecture::Kagoshima))]
    #[test_case("沖縄県" => Ok(Prefecture::Okinawa))]
    #[test_case("沖縄" => Ok(Prefecture::Okinawa))]
    #[test_case("None" => Err(Error::InvalidPrefectureName("None".to_string())))]
    fn find_by_kanji_tests(kanji: &str) -> Result<Prefecture, Error> {
        find_by_kanji(kanji)
    }

    #[test_case("ほっかいどう" => Ok(Prefecture::Hokkaido))]
    #[test_case("あおもりけん" => Ok(Prefecture::Aomori))]
    #[test_case("あおもり" => Ok(Prefecture::Aomori))]
    #[test_case("いわてけん" => Ok(Prefecture::Iwate))]
    #[test_case("いわて" => Ok(Prefecture::Iwate))]
    #[test_case("みやぎけん" => Ok(Prefecture::Miyagi))]
    #[test_case("みやぎ" => Ok(Prefecture::Miyagi))]
    #[test_case("あきたけん" => Ok(Prefecture::Akita))]
    #[test_case("あきた" => Ok(Prefecture::Akita))]
    #[test_case("やまがたけん" => Ok(Prefecture::Yamagata))]
    #[test_case("やまがた" => Ok(Prefecture::Yamagata))]
    #[test_case("ふくしまけん" => Ok(Prefecture::Fukushima))]
    #[test_case("ふくしま" => Ok(Prefecture::Fukushima))]
    #[test_case("いばらきけん" => Ok(Prefecture::Ibaraki))]
    #[test_case("いばらき" => Ok(Prefecture::Ibaraki))]
    #[test_case("とちぎけん" => Ok(Prefecture::Tochigi))]
    #[test_case("とちぎ" => Ok(Prefecture::Tochigi))]
    #[test_case("ぐんまけん" => Ok(Prefecture::Gunma))]
    #[test_case("ぐんま" => Ok(Prefecture::Gunma))]
    #[test_case("さいたまけん" => Ok(Prefecture::Saitama))]
    #[test_case("さいたま" => Ok(Prefecture::Saitama))]
    #[test_case("ちばけん" => Ok(Prefecture::Chiba))]
    #[test_case("ちば" => Ok(Prefecture::Chiba))]
    #[test_case("とうきょうと" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょう" => Ok(Prefecture::Tokyo))]
    #[test_case("かながわけん" => Ok(Prefecture::Kanagawa))]
    #[test_case("かながわ" => Ok(Prefecture::Kanagawa))]
    #[test_case("にいがたけん" => Ok(Prefecture::Niigata))]
    #[test_case("にいがた" => Ok(Prefecture::Niigata))]
    #[test_case("とやまけん" => Ok(Prefecture::Toyama))]
    #[test_case("とやま" => Ok(Prefecture::Toyama))]
    #[test_case("いしかわけん" => Ok(Prefecture::Ishikawa))]
    #[test_case("いしかわ" => Ok(Prefecture::Ishikawa))]
    #[test_case("ふくいけん" => Ok(Prefecture::Fukui))]
    #[test_case("ふくい" => Ok(Prefecture::Fukui))]
    #[test_case("やまなしけん" => Ok(Prefecture::Yamanashi))]
    #[test_case("やまなし" => Ok(Prefecture::Yamanashi))]
    #[test_case("ながのけん" => Ok(Prefecture::Nagano))]
    #[test_case("ながの" => Ok(Prefecture::Nagano))]
    #[test_case("ぎふけん" => Ok(Prefecture::Gifu))]
    #[test_case("ぎふ" => Ok(Prefecture::Gifu))]
    #[test_case("しずおかけん" => Ok(Prefecture::Shizuoka))]
    #[test_case("しずおか" => Ok(Prefecture::Shizuoka))]
    #[test_case("あいちけん" => Ok(Prefecture::Aichi))]
    #[test_case("あいち" => Ok(Prefecture::Aichi))]
    #[test_case("みえけん" => Ok(Prefecture::Mie))]
    #[test_case("みえ" => Ok(Prefecture::Mie))]
    #[test_case("しがけん" => Ok(Prefecture::Shiga))]
    #[test_case("しが" => Ok(Prefecture::Shiga))]
    #[test_case("きょうとふ" => Ok(Prefecture::Kyoto))]
    #[test_case("きょうと" => Ok(Prefecture::Kyoto))]
    #[test_case("おおさかふ" => Ok(Prefecture::Osaka))]
    #[test_case("おおさか" => Ok(Prefecture::Osaka))]
    #[test_case("ひょうごけん" => Ok(Prefecture::Hyogo))]
    #[test_case("ひょうご" => Ok(Prefecture::Hyogo))]
    #[test_case("ならけん" => Ok(Prefecture::Nara))]
    #[test_case("なら" => Ok(Prefecture::Nara))]
    #[test_case("わかやまけん" => Ok(Prefecture::Wakayama))]
    #[test_case("わかやま" => Ok(Prefecture::Wakayama))]
    #[test_case("とっとりけん" => Ok(Prefecture::Tottori))]
    #[test_case("とっとり" => Ok(Prefecture::Tottori))]
    #[test_case("しまねけん" => Ok(Prefecture::Shimane))]
    #[test_case("しまね" => Ok(Prefecture::Shimane))]
    #[test_case("おかやまけん" => Ok(Prefecture::Okayama))]
    #[test_case("おかやま" => Ok(Prefecture::Okayama))]
    #[test_case("ひろしまけん" => Ok(Prefecture::Hiroshima))]
    #[test_case("ひろしま" => Ok(Prefecture::Hiroshima))]
    #[test_case("やまぐちけん" => Ok(Prefecture::Yamaguchi))]
    #[test_case("やまぐち" => Ok(Prefecture::Yamaguchi))]
    #[test_case("とくしまけん" => Ok(Prefecture::Tokushima))]
    #[test_case("とくしま" => Ok(Prefecture::Tokushima))]
    #[test_case("かがわけん" => Ok(Prefecture::Kagawa))]
    #[test_case("かがわ" => Ok(Prefecture::Kagawa))]
    #[test_case("えひめけん" => Ok(Prefecture::Ehime))]
    #[test_case("えひめ" => Ok(Prefecture::Ehime))]
    #[test_case("こうちけん" => Ok(Prefecture::Kochi))]
    #[test_case("こうち" => Ok(Prefecture::Kochi))]
    #[test_case("ふくおかけん" => Ok(Prefecture::Fukuoka))]
    #[test_case("ふくおか" => Ok(Prefecture::Fukuoka))]
    #[test_case("さがけん" => Ok(Prefecture::Saga))]
    #[test_case("さが" => Ok(Prefecture::Saga))]
    #[test_case("ながさきけん" => Ok(Prefecture::Nagasaki))]
    #[test_case("ながさき" => Ok(Prefecture::Nagasaki))]
    #[test_case("くまもとけん" => Ok(Prefecture::Kumamoto))]
    #[test_case("くまもと" => Ok(Prefecture::Kumamoto))]
    #[test_case("おおいたけん" => Ok(Prefecture::Oita))]
    #[test_case("おおいた" => Ok(Prefecture::Oita))]
    #[test_case("みやざきけん" => Ok(Prefecture::Miyazaki))]
    #[test_case("みやざき" => Ok(Prefecture::Miyazaki))]
    #[test_case("かごしまけん" => Ok(Prefecture::Kagoshima))]
    #[test_case("かごしま" => Ok(Prefecture::Kagoshima))]
    #[test_case("おきなわけん" => Ok(Prefecture::Okinawa))]
    #[test_case("おきなわ" => Ok(Prefecture::Okinawa))]
    #[test_case("None" => Err(Error::InvalidPrefectureName("None".to_string())))]
    fn find_by_hiragana_tests(hiragana: &str) -> Result<Prefecture, Error> {
        find_by_hiragana(hiragana)
    }

    #[test_case("ホッカイドウ" => Ok(Prefecture::Hokkaido))]
    #[test_case("アオモリケン" => Ok(Prefecture::Aomori))]
    #[test_case("アオモリ" => Ok(Prefecture::Aomori))]
    #[test_case("イワテケン" => Ok(Prefecture::Iwate))]
    #[test_case("イワテ" => Ok(Prefecture::Iwate))]
    #[test_case("ミヤギケン" => Ok(Prefecture::Miyagi))]
    #[test_case("ミヤギ" => Ok(Prefecture::Miyagi))]
    #[test_case("アキタケン" => Ok(Prefecture::Akita))]
    #[test_case("アキタ" => Ok(Prefecture::Akita))]
    #[test_case("ヤマガタケン" => Ok(Prefecture::Yamagata))]
    #[test_case("ヤマガタ" => Ok(Prefecture::Yamagata))]
    #[test_case("フクシマケン" => Ok(Prefecture::Fukushima))]
    #[test_case("フクシマ" => Ok(Prefecture::Fukushima))]
    #[test_case("イバラキケン" => Ok(Prefecture::Ibaraki))]
    #[test_case("イバラキ" => Ok(Prefecture::Ibaraki))]
    #[test_case("トチギケン" => Ok(Prefecture::Tochigi))]
    #[test_case("トチギ" => Ok(Prefecture::Tochigi))]
    #[test_case("グンマケン" => Ok(Prefecture::Gunma))]
    #[test_case("グンマ" => Ok(Prefecture::Gunma))]
    #[test_case("サイタマケン" => Ok(Prefecture::Saitama))]
    #[test_case("サイタマ" => Ok(Prefecture::Saitama))]
    #[test_case("チバケン" => Ok(Prefecture::Chiba))]
    #[test_case("チバ" => Ok(Prefecture::Chiba))]
    #[test_case("トウキョウト" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウ" => Ok(Prefecture::Tokyo))]
    #[test_case("カナガワケン" => Ok(Prefecture::Kanagawa))]
    #[test_case("カナガワ" => Ok(Prefecture::Kanagawa))]
    #[test_case("ニイガタケン" => Ok(Prefecture::Niigata))]
    #[test_case("ニイガタ" => Ok(Prefecture::Niigata))]
    #[test_case("トヤマケン" => Ok(Prefecture::Toyama))]
    #[test_case("トヤマ" => Ok(Prefecture::Toyama))]
    #[test_case("イシカワケン" => Ok(Prefecture::Ishikawa))]
    #[test_case("イシカワ" => Ok(Prefecture::Ishikawa))]
    #[test_case("フクイケン" => Ok(Prefecture::Fukui))]
    #[test_case("フクイ" => Ok(Prefecture::Fukui))]
    #[test_case("ヤマナシケン" => Ok(Prefecture::Yamanashi))]
    #[test_case("ヤマナシ" => Ok(Prefecture::Yamanashi))]
    #[test_case("ナガノケン" => Ok(Prefecture::Nagano))]
    #[test_case("ナガノ" => Ok(Prefecture::Nagano))]
    #[test_case("ギフケン" => Ok(Prefecture::Gifu))]
    #[test_case("ギフ" => Ok(Prefecture::Gifu))]
    #[test_case("シズオカケン" => Ok(Prefecture::Shizuoka))]
    #[test_case("シズオカ" => Ok(Prefecture::Shizuoka))]
    #[test_case("アイチケン" => Ok(Prefecture::Aichi))]
    #[test_case("アイチ" => Ok(Prefecture::Aichi))]
    #[test_case("ミエケン" => Ok(Prefecture::Mie))]
    #[test_case("ミエ" => Ok(Prefecture::Mie))]
    #[test_case("シガケン" => Ok(Prefecture::Shiga))]
    #[test_case("シガ" => Ok(Prefecture::Shiga))]
    #[test_case("キョウトフ" => Ok(Prefecture::Kyoto))]
    #[test_case("キョウト" => Ok(Prefecture::Kyoto))]
    #[test_case("オオサカフ" => Ok(Prefecture::Osaka))]
    #[test_case("オオサカ" => Ok(Prefecture::Osaka))]
    #[test_case("ヒョウゴケン" => Ok(Prefecture::Hyogo))]
    #[test_case("ヒョウゴ" => Ok(Prefecture::Hyogo))]
    #[test_case("ナラケン" => Ok(Prefecture::Nara))]
    #[test_case("ナラ" => Ok(Prefecture::Nara))]
    #[test_case("ワカヤマケン" => Ok(Prefecture::Wakayama))]
    #[test_case("ワカヤマ" => Ok(Prefecture::Wakayama))]
    #[test_case("トットリケン" => Ok(Prefecture::Tottori))]
    #[test_case("トットリ" => Ok(Prefecture::Tottori))]
    #[test_case("シマネケン" => Ok(Prefecture::Shimane))]
    #[test_case("シマネ" => Ok(Prefecture::Shimane))]
    #[test_case("オカヤマケン" => Ok(Prefecture::Okayama))]
    #[test_case("オカヤマ" => Ok(Prefecture::Okayama))]
    #[test_case("ヒロシマケン" => Ok(Prefecture::Hiroshima))]
    #[test_case("ヒロシマ" => Ok(Prefecture::Hiroshima))]
    #[test_case("ヤマグチケン" => Ok(Prefecture::Yamaguchi))]
    #[test_case("ヤマグチ" => Ok(Prefecture::Yamaguchi))]
    #[test_case("トクシマケン" => Ok(Prefecture::Tokushima))]
    #[test_case("トクシマ" => Ok(Prefecture::Tokushima))]
    #[test_case("カガワケン" => Ok(Prefecture::Kagawa))]
    #[test_case("カガワ" => Ok(Prefecture::Kagawa))]
    #[test_case("エヒメケン" => Ok(Prefecture::Ehime))]
    #[test_case("エヒメ" => Ok(Prefecture::Ehime))]
    #[test_case("コウチケン" => Ok(Prefecture::Kochi))]
    #[test_case("コウチ" => Ok(Prefecture::Kochi))]
    #[test_case("フクオカケン" => Ok(Prefecture::Fukuoka))]
    #[test_case("フクオカ" => Ok(Prefecture::Fukuoka))]
    #[test_case("サガケン" => Ok(Prefecture::Saga))]
    #[test_case("サガ" => Ok(Prefecture::Saga))]
    #[test_case("ナガサキケン" => Ok(Prefecture::Nagasaki))]
    #[test_case("ナガサキ" => Ok(Prefecture::Nagasaki))]
    #[test_case("クマモトケン" => Ok(Prefecture::Kumamoto))]
    #[test_case("クマモト" => Ok(Prefecture::Kumamoto))]
    #[test_case("オオイタケン" => Ok(Prefecture::Oita))]
    #[test_case("オオイタ" => Ok(Prefecture::Oita))]
    #[test_case("ミヤザキケン" => Ok(Prefecture::Miyazaki))]
    #[test_case("ミヤザキ" => Ok(Prefecture::Miyazaki))]
    #[test_case("カゴシマケン" => Ok(Prefecture::Kagoshima))]
    #[test_case("カゴシマ" => Ok(Prefecture::Kagoshima))]
    #[test_case("オキナワケン" => Ok(Prefecture::Okinawa))]
    #[test_case("オキナワ" => Ok(Prefecture::Okinawa))]
    #[test_case("None" => Err(Error::InvalidPrefectureName("None".to_string())))]
    fn find_by_katakana_tests(katakana: &str) -> Result<Prefecture, Error> {
        find_by_katakana(katakana)
    }

    #[test_case("hokkaido" => Ok(Prefecture::Hokkaido))]
    #[test_case("aomori" => Ok(Prefecture::Aomori))]
    #[test_case("iwate" => Ok(Prefecture::Iwate))]
    #[test_case("miyagi" => Ok(Prefecture::Miyagi))]
    #[test_case("akita" => Ok(Prefecture::Akita))]
    #[test_case("yamagata" => Ok(Prefecture::Yamagata))]
    #[test_case("fukushima" => Ok(Prefecture::Fukushima))]
    #[test_case("ibaraki" => Ok(Prefecture::Ibaraki))]
    #[test_case("tochigi" => Ok(Prefecture::Tochigi))]
    #[test_case("gunma" => Ok(Prefecture::Gunma))]
    #[test_case("saitama" => Ok(Prefecture::Saitama))]
    #[test_case("chiba" => Ok(Prefecture::Chiba))]
    #[test_case("tokyo" => Ok(Prefecture::Tokyo))]
    #[test_case("kanagawa" => Ok(Prefecture::Kanagawa))]
    #[test_case("niigata" => Ok(Prefecture::Niigata))]
    #[test_case("toyama" => Ok(Prefecture::Toyama))]
    #[test_case("ishikawa" => Ok(Prefecture::Ishikawa))]
    #[test_case("fukui" => Ok(Prefecture::Fukui))]
    #[test_case("yamanashi" => Ok(Prefecture::Yamanashi))]
    #[test_case("nagano" => Ok(Prefecture::Nagano))]
    #[test_case("gifu" => Ok(Prefecture::Gifu))]
    #[test_case("shizuoka" => Ok(Prefecture::Shizuoka))]
    #[test_case("aichi" => Ok(Prefecture::Aichi))]
    #[test_case("mie" => Ok(Prefecture::Mie))]
    #[test_case("shiga" => Ok(Prefecture::Shiga))]
    #[test_case("kyoto" => Ok(Prefecture::Kyoto))]
    #[test_case("osaka" => Ok(Prefecture::Osaka))]
    #[test_case("hyogo" => Ok(Prefecture::Hyogo))]
    #[test_case("nara" => Ok(Prefecture::Nara))]
    #[test_case("wakayama" => Ok(Prefecture::Wakayama))]
    #[test_case("tottori" => Ok(Prefecture::Tottori))]
    #[test_case("shimane" => Ok(Prefecture::Shimane))]
    #[test_case("okayama" => Ok(Prefecture::Okayama))]
    #[test_case("hiroshima" => Ok(Prefecture::Hiroshima))]
    #[test_case("yamaguchi" => Ok(Prefecture::Yamaguchi))]
    #[test_case("tokushima" => Ok(Prefecture::Tokushima))]
    #[test_case("kagawa" => Ok(Prefecture::Kagawa))]
    #[test_case("ehime" => Ok(Prefecture::Ehime))]
    #[test_case("kochi" => Ok(Prefecture::Kochi))]
    #[test_case("fukuoka" => Ok(Prefecture::Fukuoka))]
    #[test_case("saga" => Ok(Prefecture::Saga))]
    #[test_case("nagasaki" => Ok(Prefecture::Nagasaki))]
    #[test_case("kumamoto" => Ok(Prefecture::Kumamoto))]
    #[test_case("oita" => Ok(Prefecture::Oita))]
    #[test_case("miyazaki" => Ok(Prefecture::Miyazaki))]
    #[test_case("kagoshima" => Ok(Prefecture::Kagoshima))]
    #[test_case("okinawa" => Ok(Prefecture::Okinawa))]
    #[test_case("None" => Err(Error::InvalidPrefectureName("None".to_string())))]
    fn find_by_english_tests_from_lower_case(english: &str) -> Result<Prefecture, Error> {
        find_by_english(english)
    }

    #[test_case("Hokkaido" => Ok(Prefecture::Hokkaido))]
    #[test_case("Aomori" => Ok(Prefecture::Aomori))]
    #[test_case("Iwate" => Ok(Prefecture::Iwate))]
    #[test_case("Miyagi" => Ok(Prefecture::Miyagi))]
    #[test_case("Akita" => Ok(Prefecture::Akita))]
    #[test_case("Yamagata" => Ok(Prefecture::Yamagata))]
    #[test_case("Fukushima" => Ok(Prefecture::Fukushima))]
    #[test_case("Ibaraki" => Ok(Prefecture::Ibaraki))]
    #[test_case("Tochigi" => Ok(Prefecture::Tochigi))]
    #[test_case("Gunma" => Ok(Prefecture::Gunma))]
    #[test_case("Saitama" => Ok(Prefecture::Saitama))]
    #[test_case("Chiba" => Ok(Prefecture::Chiba))]
    #[test_case("Tokyo" => Ok(Prefecture::Tokyo))]
    #[test_case("Kanagawa" => Ok(Prefecture::Kanagawa))]
    #[test_case("Niigata" => Ok(Prefecture::Niigata))]
    #[test_case("Toyama" => Ok(Prefecture::Toyama))]
    #[test_case("Ishikawa" => Ok(Prefecture::Ishikawa))]
    #[test_case("Fukui" => Ok(Prefecture::Fukui))]
    #[test_case("Yamanashi" => Ok(Prefecture::Yamanashi))]
    #[test_case("Nagano" => Ok(Prefecture::Nagano))]
    #[test_case("Gifu" => Ok(Prefecture::Gifu))]
    #[test_case("Shizuoka" => Ok(Prefecture::Shizuoka))]
    #[test_case("Aichi" => Ok(Prefecture::Aichi))]
    #[test_case("Mie" => Ok(Prefecture::Mie))]
    #[test_case("Shiga" => Ok(Prefecture::Shiga))]
    #[test_case("Kyoto" => Ok(Prefecture::Kyoto))]
    #[test_case("Osaka" => Ok(Prefecture::Osaka))]
    #[test_case("Hyogo" => Ok(Prefecture::Hyogo))]
    #[test_case("Nara" => Ok(Prefecture::Nara))]
    #[test_case("Wakayama" => Ok(Prefecture::Wakayama))]
    #[test_case("Tottori" => Ok(Prefecture::Tottori))]
    #[test_case("Shimane" => Ok(Prefecture::Shimane))]
    #[test_case("Okayama" => Ok(Prefecture::Okayama))]
    #[test_case("Hiroshima" => Ok(Prefecture::Hiroshima))]
    #[test_case("Yamaguchi" => Ok(Prefecture::Yamaguchi))]
    #[test_case("Tokushima" => Ok(Prefecture::Tokushima))]
    #[test_case("Kagawa" => Ok(Prefecture::Kagawa))]
    #[test_case("Ehime" => Ok(Prefecture::Ehime))]
    #[test_case("Kochi" => Ok(Prefecture::Kochi))]
    #[test_case("Fukuoka" => Ok(Prefecture::Fukuoka))]
    #[test_case("Saga" => Ok(Prefecture::Saga))]
    #[test_case("Nagasaki" => Ok(Prefecture::Nagasaki))]
    #[test_case("Kumamoto" => Ok(Prefecture::Kumamoto))]
    #[test_case("Oita" => Ok(Prefecture::Oita))]
    #[test_case("Miyazaki" => Ok(Prefecture::Miyazaki))]
    #[test_case("Kagoshima" => Ok(Prefecture::Kagoshima))]
    #[test_case("Okinawa" => Ok(Prefecture::Okinawa))]
    #[test_case("None" => Err(Error::InvalidPrefectureName("None".to_string())))]
    fn find_by_english_tests_from_upper_case(english: &str) -> Result<Prefecture, Error> {
        find_by_english(english)
    }

    #[test_case("東京都" => Ok(Prefecture::Tokyo))]
    #[test_case("東京" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょうと" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょう" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウト" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウ" => Ok(Prefecture::Tokyo))]
    #[test_case("tokyo" => Ok(Prefecture::Tokyo))]
    #[test_case("HoKkaido" => Ok(Prefecture::Hokkaido))]
    #[test_case("none" => Err(Error::InvalidPrefectureName("none".to_string())))]
    fn find_tests(s: &str) -> Result<Prefecture, Error> {
        find(s)
    }

    #[test_case("東京都" => Ok(Prefecture::Tokyo))]
    #[test_case("東京" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょうと" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょう" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウト" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウ" => Ok(Prefecture::Tokyo))]
    #[test_case("tokyo" => Ok(Prefecture::Tokyo))]
    #[test_case("HoKkaido" => Ok(Prefecture::Hokkaido))]
    #[test_case("error" => Err(Error::InvalidPrefectureName("error".to_string())))]
    fn from_str_tests(s: &str) -> Result<Prefecture, Error> {
        Prefecture::from_str(s)
    }
}
