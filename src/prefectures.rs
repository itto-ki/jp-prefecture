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
//! assert_eq!(tokyo.unwrap().kanji(), "東京都");
//! assert_eq!(tokyo.unwrap().kanji_short(), "東京");
//! assert_eq!(tokyo.unwrap().kanji_short(), "東京");
//! assert_eq!(tokyo.unwrap().hiragana(), "とうきょうと");
//! assert_eq!(tokyo.unwrap().hiragana_short(), "とうきょう");
//! assert_eq!(tokyo.unwrap().katakana(), "トウキョウト");
//! assert_eq!(tokyo.unwrap().katakana_short(), "トウキョウ");
//! assert_eq!(tokyo.unwrap().english(), "tokyo");
//! ```

use std::collections::HashMap;

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
    pub fn jis_x_0401_code(self) -> u32 {
        self as u32
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
    /// assert_eq!(tokyo.kanji(), "東京都");
    /// ```
    pub fn kanji(self) -> &'static str {
        PREFECTURE_MAP.get(&self).expect("Unexpected error").kanji
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
    /// assert_eq!(tokyo.kanji_short(), "東京");
    /// ```
    pub fn kanji_short(self) -> &'static str {
        let kanji = self.kanji();
        match self {
            Prefecture::Hokkaido => kanji,
            Prefecture::Tokyo => kanji.trim_end_matches('都'),
            Prefecture::Kyoto | Prefecture::Osaka => kanji.trim_end_matches('府'),
            _ => kanji.trim_end_matches('県'),
        }
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
    /// assert_eq!(tokyo.hiragana(), "とうきょうと");
    /// ```
    pub fn hiragana(&self) -> &'static str {
        PREFECTURE_MAP.get(self).expect("Unexpected error").hiragana
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
    /// assert_eq!(tokyo.hiragana_short(), "とうきょう");
    /// ```
    pub fn hiragana_short(&self) -> &'static str {
        let hiragana = self.hiragana();
        match self {
            Prefecture::Hokkaido => hiragana,
            Prefecture::Tokyo => hiragana.trim_end_matches('と'),
            Prefecture::Kyoto | Prefecture::Osaka => hiragana.trim_end_matches('ふ'),
            _ => hiragana.trim_end_matches("けん"),
        }
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
    /// assert_eq!(tokyo.katakana(), "トウキョウト");
    /// ```
    pub fn katakana(&self) -> &'static str {
        PREFECTURE_MAP.get(self).expect("Unexpected error").katakana
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
    /// assert_eq!(tokyo.katakana_short(), "トウキョウ");
    /// ```
    pub fn katakana_short(&self) -> &'static str {
        let katakana = self.katakana();
        match self {
            Prefecture::Hokkaido => katakana,
            Prefecture::Tokyo => katakana.trim_end_matches('ト'),
            Prefecture::Kyoto | Prefecture::Osaka => katakana.trim_end_matches('フ'),
            _ => katakana.trim_end_matches("ケン"),
        }
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
    /// assert_eq!(tokyo.english(), "tokyo");
    /// ```
    pub fn english(&self) -> &'static str {
        PREFECTURE_MAP.get(self).expect("Unexpected error").english
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
pub fn find_by_code(code: u32) -> Result<Prefecture, Error<'static>> {
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
/// assert_eq!(prefectures::find_by_kanji("東京県"), Err(Error::InvalidPrefectureName("東京県")));
/// ```
pub fn find_by_kanji(kanji: &str) -> Result<Prefecture, Error> {
    let mut map: HashMap<&str, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.kanji(), *pref);
        map.insert(pref.kanji_short(), *pref);
    });
    map.get(kanji)
        .ok_or_else(|| Error::InvalidPrefectureName(kanji))
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
/// assert_eq!(prefectures::find_by_hiragana("とうきょうけん"), Err(Error::InvalidPrefectureName("とうきょうけん")));
/// ```
pub fn find_by_hiragana(hiragana: &str) -> Result<Prefecture, Error> {
    let mut map: HashMap<&str, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.hiragana(), *pref);
        map.insert(pref.hiragana_short(), *pref);
    });
    map.get(hiragana)
        .ok_or_else(|| Error::InvalidPrefectureName(hiragana))
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
/// assert_eq!(prefectures::find_by_katakana("トウキョウケン"), Err(Error::InvalidPrefectureName("トウキョウケン")));
/// ```
pub fn find_by_katakana(katakana: &str) -> Result<Prefecture, Error> {
    let mut map: HashMap<&str, Prefecture> = HashMap::new();
    PREFECTURE_MAP.iter().for_each(|(pref, _)| {
        map.insert(pref.katakana(), *pref);
        map.insert(pref.katakana_short(), *pref);
    });
    map.get(katakana)
        .ok_or_else(|| Error::InvalidPrefectureName(katakana))
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
/// assert_eq!(prefectures::find_by_english("tokyo~~~"), Err(Error::InvalidPrefectureName("tokyo~~~")));
/// ```
pub fn find_by_english(english: &str) -> Result<Prefecture, Error> {
    PREFECTURE_MAP
        .iter()
        .find(|(_, data)| data.english == english.to_ascii_lowercase())
        .map(|(pref, _)| *pref)
        .ok_or_else(|| Error::InvalidPrefectureName(english))
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
/// assert_eq!(prefectures::find("none"), Err(Error::InvalidPrefectureName("none")));
/// ```
pub fn find(s: &str) -> Result<Prefecture, Error> {
    Prefecture::try_from(s)
}

impl<'a> TryFrom<&'a str> for Prefecture {
    type Error = Error<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut map: HashMap<&str, Prefecture> = HashMap::new();
        PREFECTURE_MAP.iter().for_each(|(pref, _)| {
            map.insert(pref.kanji(), *pref);
            map.insert(pref.kanji_short(), *pref);
            map.insert(pref.hiragana(), *pref);
            map.insert(pref.hiragana_short(), *pref);
            map.insert(pref.katakana(), *pref);
            map.insert(pref.katakana_short(), *pref);
            map.insert(pref.english(), *pref);
        });
        map.get(s.to_ascii_lowercase().as_str())
            .copied()
            .ok_or_else(|| Self::Error::InvalidPrefectureName(s))
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

    #[test_case(Prefecture::Hokkaido => "北海道")]
    #[test_case(Prefecture::Aomori => "青森県")]
    #[test_case(Prefecture::Iwate => "岩手県")]
    #[test_case(Prefecture::Miyagi => "宮城県")]
    #[test_case(Prefecture::Akita => "秋田県")]
    #[test_case(Prefecture::Yamagata => "山形県")]
    #[test_case(Prefecture::Fukushima => "福島県")]
    #[test_case(Prefecture::Ibaraki => "茨城県")]
    #[test_case(Prefecture::Tochigi => "栃木県")]
    #[test_case(Prefecture::Gunma => "群馬県")]
    #[test_case(Prefecture::Saitama => "埼玉県")]
    #[test_case(Prefecture::Chiba => "千葉県")]
    #[test_case(Prefecture::Tokyo => "東京都")]
    #[test_case(Prefecture::Kanagawa => "神奈川県")]
    #[test_case(Prefecture::Niigata => "新潟県")]
    #[test_case(Prefecture::Toyama => "富山県")]
    #[test_case(Prefecture::Ishikawa => "石川県")]
    #[test_case(Prefecture::Fukui => "福井県")]
    #[test_case(Prefecture::Yamanashi => "山梨県")]
    #[test_case(Prefecture::Nagano => "長野県")]
    #[test_case(Prefecture::Gifu => "岐阜県")]
    #[test_case(Prefecture::Shizuoka => "静岡県")]
    #[test_case(Prefecture::Aichi => "愛知県")]
    #[test_case(Prefecture::Mie => "三重県")]
    #[test_case(Prefecture::Shiga => "滋賀県")]
    #[test_case(Prefecture::Kyoto => "京都府")]
    #[test_case(Prefecture::Osaka => "大阪府")]
    #[test_case(Prefecture::Hyogo => "兵庫県")]
    #[test_case(Prefecture::Nara => "奈良県")]
    #[test_case(Prefecture::Wakayama => "和歌山県")]
    #[test_case(Prefecture::Tottori => "鳥取県")]
    #[test_case(Prefecture::Shimane => "島根県")]
    #[test_case(Prefecture::Okayama => "岡山県")]
    #[test_case(Prefecture::Hiroshima => "広島県")]
    #[test_case(Prefecture::Yamaguchi => "山口県")]
    #[test_case(Prefecture::Tokushima => "徳島県")]
    #[test_case(Prefecture::Kagawa => "香川県")]
    #[test_case(Prefecture::Ehime => "愛媛県")]
    #[test_case(Prefecture::Kochi => "高知県")]
    #[test_case(Prefecture::Fukuoka => "福岡県")]
    #[test_case(Prefecture::Saga => "佐賀県")]
    #[test_case(Prefecture::Nagasaki => "長崎県")]
    #[test_case(Prefecture::Kumamoto => "熊本県")]
    #[test_case(Prefecture::Oita => "大分県")]
    #[test_case(Prefecture::Miyazaki => "宮崎県")]
    #[test_case(Prefecture::Kagoshima => "鹿児島県")]
    #[test_case(Prefecture::Okinawa => "沖縄県")]
    fn kanji_tests(prefecture: Prefecture) -> &'static str {
        prefecture.kanji()
    }

    #[test_case(Prefecture::Hokkaido => "北海道")]
    #[test_case(Prefecture::Aomori => "青森")]
    #[test_case(Prefecture::Iwate => "岩手")]
    #[test_case(Prefecture::Miyagi => "宮城")]
    #[test_case(Prefecture::Akita => "秋田")]
    #[test_case(Prefecture::Yamagata => "山形")]
    #[test_case(Prefecture::Fukushima => "福島")]
    #[test_case(Prefecture::Ibaraki => "茨城")]
    #[test_case(Prefecture::Tochigi => "栃木")]
    #[test_case(Prefecture::Gunma => "群馬")]
    #[test_case(Prefecture::Saitama => "埼玉")]
    #[test_case(Prefecture::Chiba => "千葉")]
    #[test_case(Prefecture::Tokyo => "東京")]
    #[test_case(Prefecture::Kanagawa => "神奈川")]
    #[test_case(Prefecture::Niigata => "新潟")]
    #[test_case(Prefecture::Toyama => "富山")]
    #[test_case(Prefecture::Ishikawa => "石川")]
    #[test_case(Prefecture::Fukui => "福井")]
    #[test_case(Prefecture::Yamanashi => "山梨")]
    #[test_case(Prefecture::Nagano => "長野")]
    #[test_case(Prefecture::Gifu => "岐阜")]
    #[test_case(Prefecture::Shizuoka => "静岡")]
    #[test_case(Prefecture::Aichi => "愛知")]
    #[test_case(Prefecture::Mie => "三重")]
    #[test_case(Prefecture::Shiga => "滋賀")]
    #[test_case(Prefecture::Kyoto => "京都")]
    #[test_case(Prefecture::Osaka => "大阪")]
    #[test_case(Prefecture::Hyogo => "兵庫")]
    #[test_case(Prefecture::Nara => "奈良")]
    #[test_case(Prefecture::Wakayama => "和歌山")]
    #[test_case(Prefecture::Tottori => "鳥取")]
    #[test_case(Prefecture::Shimane => "島根")]
    #[test_case(Prefecture::Okayama => "岡山")]
    #[test_case(Prefecture::Hiroshima => "広島")]
    #[test_case(Prefecture::Yamaguchi => "山口")]
    #[test_case(Prefecture::Tokushima => "徳島")]
    #[test_case(Prefecture::Kagawa => "香川")]
    #[test_case(Prefecture::Ehime => "愛媛")]
    #[test_case(Prefecture::Kochi => "高知")]
    #[test_case(Prefecture::Fukuoka => "福岡")]
    #[test_case(Prefecture::Saga => "佐賀")]
    #[test_case(Prefecture::Nagasaki => "長崎")]
    #[test_case(Prefecture::Kumamoto => "熊本")]
    #[test_case(Prefecture::Oita => "大分")]
    #[test_case(Prefecture::Miyazaki => "宮崎")]
    #[test_case(Prefecture::Kagoshima => "鹿児島")]
    #[test_case(Prefecture::Okinawa => "沖縄")]
    fn kanji_short_tests(prefecture: Prefecture) -> &'static str {
        prefecture.kanji_short()
    }

    #[test_case(Prefecture::Hokkaido => "ほっかいどう")]
    #[test_case(Prefecture::Aomori => "あおもりけん")]
    #[test_case(Prefecture::Iwate => "いわてけん")]
    #[test_case(Prefecture::Miyagi => "みやぎけん")]
    #[test_case(Prefecture::Akita => "あきたけん")]
    #[test_case(Prefecture::Yamagata => "やまがたけん")]
    #[test_case(Prefecture::Fukushima => "ふくしまけん")]
    #[test_case(Prefecture::Ibaraki => "いばらきけん")]
    #[test_case(Prefecture::Tochigi => "とちぎけん")]
    #[test_case(Prefecture::Gunma => "ぐんまけん")]
    #[test_case(Prefecture::Saitama => "さいたまけん")]
    #[test_case(Prefecture::Chiba => "ちばけん")]
    #[test_case(Prefecture::Tokyo => "とうきょうと")]
    #[test_case(Prefecture::Kanagawa => "かながわけん")]
    #[test_case(Prefecture::Niigata => "にいがたけん")]
    #[test_case(Prefecture::Toyama => "とやまけん")]
    #[test_case(Prefecture::Ishikawa => "いしかわけん")]
    #[test_case(Prefecture::Fukui => "ふくいけん")]
    #[test_case(Prefecture::Yamanashi => "やまなしけん")]
    #[test_case(Prefecture::Nagano => "ながのけん")]
    #[test_case(Prefecture::Gifu => "ぎふけん")]
    #[test_case(Prefecture::Shizuoka => "しずおかけん")]
    #[test_case(Prefecture::Aichi => "あいちけん")]
    #[test_case(Prefecture::Mie => "みえけん")]
    #[test_case(Prefecture::Shiga => "しがけん")]
    #[test_case(Prefecture::Kyoto => "きょうとふ")]
    #[test_case(Prefecture::Osaka => "おおさかふ")]
    #[test_case(Prefecture::Hyogo => "ひょうごけん")]
    #[test_case(Prefecture::Nara => "ならけん")]
    #[test_case(Prefecture::Wakayama => "わかやまけん")]
    #[test_case(Prefecture::Tottori => "とっとりけん")]
    #[test_case(Prefecture::Shimane => "しまねけん")]
    #[test_case(Prefecture::Okayama => "おかやまけん")]
    #[test_case(Prefecture::Hiroshima => "ひろしまけん")]
    #[test_case(Prefecture::Yamaguchi => "やまぐちけん")]
    #[test_case(Prefecture::Tokushima => "とくしまけん")]
    #[test_case(Prefecture::Kagawa => "かがわけん")]
    #[test_case(Prefecture::Ehime => "えひめけん")]
    #[test_case(Prefecture::Kochi => "こうちけん")]
    #[test_case(Prefecture::Fukuoka => "ふくおかけん")]
    #[test_case(Prefecture::Saga => "さがけん")]
    #[test_case(Prefecture::Nagasaki => "ながさきけん")]
    #[test_case(Prefecture::Kumamoto => "くまもとけん")]
    #[test_case(Prefecture::Oita => "おおいたけん")]
    #[test_case(Prefecture::Miyazaki => "みやざきけん")]
    #[test_case(Prefecture::Kagoshima => "かごしまけん")]
    #[test_case(Prefecture::Okinawa => "おきなわけん")]
    fn hiragana_tests(prefecture: Prefecture) -> &'static str {
        prefecture.hiragana()
    }

    #[test_case(Prefecture::Hokkaido => "ほっかいどう")]
    #[test_case(Prefecture::Aomori => "あおもり")]
    #[test_case(Prefecture::Iwate => "いわて")]
    #[test_case(Prefecture::Miyagi => "みやぎ")]
    #[test_case(Prefecture::Akita => "あきた")]
    #[test_case(Prefecture::Yamagata => "やまがた")]
    #[test_case(Prefecture::Fukushima => "ふくしま")]
    #[test_case(Prefecture::Ibaraki => "いばらき")]
    #[test_case(Prefecture::Tochigi => "とちぎ")]
    #[test_case(Prefecture::Gunma => "ぐんま")]
    #[test_case(Prefecture::Saitama => "さいたま")]
    #[test_case(Prefecture::Chiba => "ちば")]
    #[test_case(Prefecture::Tokyo => "とうきょう")]
    #[test_case(Prefecture::Kanagawa => "かながわ")]
    #[test_case(Prefecture::Niigata => "にいがた")]
    #[test_case(Prefecture::Toyama => "とやま")]
    #[test_case(Prefecture::Ishikawa => "いしかわ")]
    #[test_case(Prefecture::Fukui => "ふくい")]
    #[test_case(Prefecture::Yamanashi => "やまなし")]
    #[test_case(Prefecture::Nagano => "ながの")]
    #[test_case(Prefecture::Gifu => "ぎふ")]
    #[test_case(Prefecture::Shizuoka => "しずおか")]
    #[test_case(Prefecture::Aichi => "あいち")]
    #[test_case(Prefecture::Mie => "みえ")]
    #[test_case(Prefecture::Shiga => "しが")]
    #[test_case(Prefecture::Kyoto => "きょうと")]
    #[test_case(Prefecture::Osaka => "おおさか")]
    #[test_case(Prefecture::Hyogo => "ひょうご")]
    #[test_case(Prefecture::Nara => "なら")]
    #[test_case(Prefecture::Wakayama => "わかやま")]
    #[test_case(Prefecture::Tottori => "とっとり")]
    #[test_case(Prefecture::Shimane => "しまね")]
    #[test_case(Prefecture::Okayama => "おかやま")]
    #[test_case(Prefecture::Hiroshima => "ひろしま")]
    #[test_case(Prefecture::Yamaguchi => "やまぐち")]
    #[test_case(Prefecture::Tokushima => "とくしま")]
    #[test_case(Prefecture::Kagawa => "かがわ")]
    #[test_case(Prefecture::Ehime => "えひめ")]
    #[test_case(Prefecture::Kochi => "こうち")]
    #[test_case(Prefecture::Fukuoka => "ふくおか")]
    #[test_case(Prefecture::Saga => "さが")]
    #[test_case(Prefecture::Nagasaki => "ながさき")]
    #[test_case(Prefecture::Kumamoto => "くまもと")]
    #[test_case(Prefecture::Oita => "おおいた")]
    #[test_case(Prefecture::Miyazaki => "みやざき")]
    #[test_case(Prefecture::Kagoshima => "かごしま")]
    #[test_case(Prefecture::Okinawa => "おきなわ")]
    fn hiragana_short_tests(prefecture: Prefecture) -> &'static str {
        prefecture.hiragana_short()
    }

    #[test_case(Prefecture::Hokkaido => "ホッカイドウ")]
    #[test_case(Prefecture::Aomori => "アオモリケン")]
    #[test_case(Prefecture::Iwate => "イワテケン")]
    #[test_case(Prefecture::Miyagi => "ミヤギケン")]
    #[test_case(Prefecture::Akita => "アキタケン")]
    #[test_case(Prefecture::Yamagata => "ヤマガタケン")]
    #[test_case(Prefecture::Fukushima => "フクシマケン")]
    #[test_case(Prefecture::Ibaraki => "イバラキケン")]
    #[test_case(Prefecture::Tochigi => "トチギケン")]
    #[test_case(Prefecture::Gunma => "グンマケン")]
    #[test_case(Prefecture::Saitama => "サイタマケン")]
    #[test_case(Prefecture::Chiba => "チバケン")]
    #[test_case(Prefecture::Tokyo => "トウキョウト")]
    #[test_case(Prefecture::Kanagawa => "カナガワケン")]
    #[test_case(Prefecture::Niigata => "ニイガタケン")]
    #[test_case(Prefecture::Toyama => "トヤマケン")]
    #[test_case(Prefecture::Ishikawa => "イシカワケン")]
    #[test_case(Prefecture::Fukui => "フクイケン")]
    #[test_case(Prefecture::Yamanashi => "ヤマナシケン")]
    #[test_case(Prefecture::Nagano => "ナガノケン")]
    #[test_case(Prefecture::Gifu => "ギフケン")]
    #[test_case(Prefecture::Shizuoka => "シズオカケン")]
    #[test_case(Prefecture::Aichi => "アイチケン")]
    #[test_case(Prefecture::Mie => "ミエケン")]
    #[test_case(Prefecture::Shiga => "シガケン")]
    #[test_case(Prefecture::Kyoto => "キョウトフ")]
    #[test_case(Prefecture::Osaka => "オオサカフ")]
    #[test_case(Prefecture::Hyogo => "ヒョウゴケン")]
    #[test_case(Prefecture::Nara => "ナラケン")]
    #[test_case(Prefecture::Wakayama => "ワカヤマケン")]
    #[test_case(Prefecture::Tottori => "トットリケン")]
    #[test_case(Prefecture::Shimane => "シマネケン")]
    #[test_case(Prefecture::Okayama => "オカヤマケン")]
    #[test_case(Prefecture::Hiroshima => "ヒロシマケン")]
    #[test_case(Prefecture::Yamaguchi => "ヤマグチケン")]
    #[test_case(Prefecture::Tokushima => "トクシマケン")]
    #[test_case(Prefecture::Kagawa => "カガワケン")]
    #[test_case(Prefecture::Ehime => "エヒメケン")]
    #[test_case(Prefecture::Kochi => "コウチケン")]
    #[test_case(Prefecture::Fukuoka => "フクオカケン")]
    #[test_case(Prefecture::Saga => "サガケン")]
    #[test_case(Prefecture::Nagasaki => "ナガサキケン")]
    #[test_case(Prefecture::Kumamoto => "クマモトケン")]
    #[test_case(Prefecture::Oita => "オオイタケン")]
    #[test_case(Prefecture::Miyazaki => "ミヤザキケン")]
    #[test_case(Prefecture::Kagoshima => "カゴシマケン")]
    #[test_case(Prefecture::Okinawa => "オキナワケン")]
    fn katakana_tests(prefecture: Prefecture) -> &'static str {
        prefecture.katakana()
    }

    #[test_case(Prefecture::Hokkaido => "ホッカイドウ")]
    #[test_case(Prefecture::Aomori => "アオモリ")]
    #[test_case(Prefecture::Iwate => "イワテ")]
    #[test_case(Prefecture::Miyagi => "ミヤギ")]
    #[test_case(Prefecture::Akita => "アキタ")]
    #[test_case(Prefecture::Yamagata => "ヤマガタ")]
    #[test_case(Prefecture::Fukushima => "フクシマ")]
    #[test_case(Prefecture::Ibaraki => "イバラキ")]
    #[test_case(Prefecture::Tochigi => "トチギ")]
    #[test_case(Prefecture::Gunma => "グンマ")]
    #[test_case(Prefecture::Saitama => "サイタマ")]
    #[test_case(Prefecture::Chiba => "チバ")]
    #[test_case(Prefecture::Tokyo => "トウキョウ")]
    #[test_case(Prefecture::Kanagawa => "カナガワ")]
    #[test_case(Prefecture::Niigata => "ニイガタ")]
    #[test_case(Prefecture::Toyama => "トヤマ")]
    #[test_case(Prefecture::Ishikawa => "イシカワ")]
    #[test_case(Prefecture::Fukui => "フクイ")]
    #[test_case(Prefecture::Yamanashi => "ヤマナシ")]
    #[test_case(Prefecture::Nagano => "ナガノ")]
    #[test_case(Prefecture::Gifu => "ギフ")]
    #[test_case(Prefecture::Shizuoka => "シズオカ")]
    #[test_case(Prefecture::Aichi => "アイチ")]
    #[test_case(Prefecture::Mie => "ミエ")]
    #[test_case(Prefecture::Shiga => "シガ")]
    #[test_case(Prefecture::Kyoto => "キョウト")]
    #[test_case(Prefecture::Osaka => "オオサカ")]
    #[test_case(Prefecture::Hyogo => "ヒョウゴ")]
    #[test_case(Prefecture::Nara => "ナラ")]
    #[test_case(Prefecture::Wakayama => "ワカヤマ")]
    #[test_case(Prefecture::Tottori => "トットリ")]
    #[test_case(Prefecture::Shimane => "シマネ")]
    #[test_case(Prefecture::Okayama => "オカヤマ")]
    #[test_case(Prefecture::Hiroshima => "ヒロシマ")]
    #[test_case(Prefecture::Yamaguchi => "ヤマグチ")]
    #[test_case(Prefecture::Tokushima => "トクシマ")]
    #[test_case(Prefecture::Kagawa => "カガワ")]
    #[test_case(Prefecture::Ehime => "エヒメ")]
    #[test_case(Prefecture::Kochi => "コウチ")]
    #[test_case(Prefecture::Fukuoka => "フクオカ")]
    #[test_case(Prefecture::Saga => "サガ")]
    #[test_case(Prefecture::Nagasaki => "ナガサキ")]
    #[test_case(Prefecture::Kumamoto => "クマモト")]
    #[test_case(Prefecture::Oita => "オオイタ")]
    #[test_case(Prefecture::Miyazaki => "ミヤザキ")]
    #[test_case(Prefecture::Kagoshima => "カゴシマ")]
    #[test_case(Prefecture::Okinawa => "オキナワ")]
    fn katakana_short_tests(prefecture: Prefecture) -> &'static str {
        prefecture.katakana_short()
    }

    #[test_case(Prefecture::Hokkaido => "hokkaido")]
    #[test_case(Prefecture::Aomori => "aomori")]
    #[test_case(Prefecture::Iwate => "iwate")]
    #[test_case(Prefecture::Miyagi => "miyagi")]
    #[test_case(Prefecture::Akita => "akita")]
    #[test_case(Prefecture::Yamagata => "yamagata")]
    #[test_case(Prefecture::Fukushima => "fukushima")]
    #[test_case(Prefecture::Ibaraki => "ibaraki")]
    #[test_case(Prefecture::Tochigi => "tochigi")]
    #[test_case(Prefecture::Gunma => "gunma")]
    #[test_case(Prefecture::Saitama => "saitama")]
    #[test_case(Prefecture::Chiba => "chiba")]
    #[test_case(Prefecture::Tokyo => "tokyo")]
    #[test_case(Prefecture::Kanagawa => "kanagawa")]
    #[test_case(Prefecture::Niigata => "niigata")]
    #[test_case(Prefecture::Toyama => "toyama")]
    #[test_case(Prefecture::Ishikawa => "ishikawa")]
    #[test_case(Prefecture::Fukui => "fukui")]
    #[test_case(Prefecture::Yamanashi => "yamanashi")]
    #[test_case(Prefecture::Nagano => "nagano")]
    #[test_case(Prefecture::Gifu => "gifu")]
    #[test_case(Prefecture::Shizuoka => "shizuoka")]
    #[test_case(Prefecture::Aichi => "aichi")]
    #[test_case(Prefecture::Mie => "mie")]
    #[test_case(Prefecture::Shiga => "shiga")]
    #[test_case(Prefecture::Kyoto => "kyoto")]
    #[test_case(Prefecture::Osaka => "osaka")]
    #[test_case(Prefecture::Hyogo => "hyogo")]
    #[test_case(Prefecture::Nara => "nara")]
    #[test_case(Prefecture::Wakayama => "wakayama")]
    #[test_case(Prefecture::Tottori => "tottori")]
    #[test_case(Prefecture::Shimane => "shimane")]
    #[test_case(Prefecture::Okayama => "okayama")]
    #[test_case(Prefecture::Hiroshima => "hiroshima")]
    #[test_case(Prefecture::Yamaguchi => "yamaguchi")]
    #[test_case(Prefecture::Tokushima => "tokushima")]
    #[test_case(Prefecture::Kagawa => "kagawa")]
    #[test_case(Prefecture::Ehime => "ehime")]
    #[test_case(Prefecture::Kochi => "kochi")]
    #[test_case(Prefecture::Fukuoka => "fukuoka")]
    #[test_case(Prefecture::Saga => "saga")]
    #[test_case(Prefecture::Nagasaki => "nagasaki")]
    #[test_case(Prefecture::Kumamoto => "kumamoto")]
    #[test_case(Prefecture::Oita => "oita")]
    #[test_case(Prefecture::Miyazaki => "miyazaki")]
    #[test_case(Prefecture::Kagoshima => "kagoshima")]
    #[test_case(Prefecture::Okinawa => "okinawa")]
    fn english_tests(prefecture: Prefecture) -> &'static str {
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
    fn find_by_code_tests(code: u32) -> Result<Prefecture, Error<'static>> {
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
    #[test_case("None" => Err(Error::InvalidPrefectureName("None")))]
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
    #[test_case("None" => Err(Error::InvalidPrefectureName("None")))]
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
    #[test_case("None" => Err(Error::InvalidPrefectureName("None")))]
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
    #[test_case("None" => Err(Error::InvalidPrefectureName("None")))]
    fn find_by_english_tests(katakana: &str) -> Result<Prefecture, Error> {
        find_by_english(katakana)
    }

    #[test_case("東京都" => Ok(Prefecture::Tokyo))]
    #[test_case("東京" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょうと" => Ok(Prefecture::Tokyo))]
    #[test_case("とうきょう" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウト" => Ok(Prefecture::Tokyo))]
    #[test_case("トウキョウ" => Ok(Prefecture::Tokyo))]
    #[test_case("tokyo" => Ok(Prefecture::Tokyo))]
    #[test_case("HoKkaido" => Ok(Prefecture::Hokkaido))]
    #[test_case("none" => Err(Error::InvalidPrefectureName("none")))]
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
    #[test_case("error" => Err(Error::InvalidPrefectureName("error")))]
    fn try_from_tests(s: &str) -> Result<Prefecture, Error> {
        Prefecture::try_from(s)
    }
}
