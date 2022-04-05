use std::collections::HashMap;

use crate::mapping::PREFECTURE_MAP;

/// Japanese prefectures
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
    /// let hokkaido = Prefecture::Hokkaido;
    ///
    /// assert_eq!(hokkaido.jis_x_0401_code(), 1);
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
            Prefecture::Tokyo => kanji.trim_end_matches("都"),
            Prefecture::Kyoto | Prefecture::Osaka => kanji.trim_end_matches("府"),
            _ => kanji.trim_end_matches("県"),
        }
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
    /// assert_eq!(tokyo.hiragana(), "とうきょうと");
    /// ```
    pub fn hiragana(&self) -> &'static str {
        PREFECTURE_MAP
            .get(&self)
            .expect("Unexpected error")
            .hiragana
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
    /// assert_eq!(tokyo.hiragana_short(), "とうきょう");
    /// ```
    pub fn hiragana_short(&self) -> &'static str {
        let hiragana = self.hiragana();
        match self {
            Prefecture::Hokkaido => hiragana,
            Prefecture::Tokyo => hiragana.trim_end_matches("と"),
            Prefecture::Kyoto | Prefecture::Osaka => hiragana.trim_end_matches("ふ"),
            _ => hiragana.trim_end_matches("けん"),
        }
    }

    /// Return a prefecture name in romaji
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// let tokyo = Prefecture::Tokyo;
    ///
    /// assert_eq!(tokyo.romaji(), "tokyo");
    /// ```
    pub fn romaji(&self) -> &'static str {
        PREFECTURE_MAP.get(&self).expect("Unexpected error").romaji
    }

    /// Return iterator of all prefectures
    pub fn all() {
        unimplemented!()
    }

    /// Return a short prefecture name in kanji
    ///
    /// # Examples
    ///
    /// ```
    /// use jp_prefecture::prefectures::Prefecture;
    ///
    /// assert_eq!(Prefecture::find_by_kanji("東京都"), Some(Prefecture::Tokyo))
    /// assert_eq!(Prefecture::find_by_kanji("東京"), Some(Prefecture::Tokyo))
    /// ```
    pub fn find_by_kanji(kanji: &'static str) -> Option<Self> {
        let mut map: HashMap<&str, Prefecture> = HashMap::new();
        PREFECTURE_MAP.iter().for_each(|(pref, _)| {
            map.insert(pref.kanji(), *pref);
            map.insert(pref.kanji_short(), *pref);
        });
        map.get(kanji).map(|result| *result)
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

    #[test_case("北海道" => Some(Prefecture::Hokkaido))]
    #[test_case("青森県" => Some(Prefecture::Aomori))]
    #[test_case("青森" => Some(Prefecture::Aomori))]
    #[test_case("岩手県" => Some(Prefecture::Iwate))]
    #[test_case("岩手" => Some(Prefecture::Iwate))]
    #[test_case("宮城県" => Some(Prefecture::Miyagi))]
    #[test_case("宮城" => Some(Prefecture::Miyagi))]
    #[test_case("秋田県" => Some(Prefecture::Akita))]
    #[test_case("秋田" => Some(Prefecture::Akita))]
    #[test_case("山形県" => Some(Prefecture::Yamagata))]
    #[test_case("山形" => Some(Prefecture::Yamagata))]
    #[test_case("福島県" => Some(Prefecture::Fukushima))]
    #[test_case("福島" => Some(Prefecture::Fukushima))]
    #[test_case("茨城県" => Some(Prefecture::Ibaraki))]
    #[test_case("茨城" => Some(Prefecture::Ibaraki))]
    #[test_case("栃木県" => Some(Prefecture::Tochigi))]
    #[test_case("栃木" => Some(Prefecture::Tochigi))]
    #[test_case("群馬県" => Some(Prefecture::Gunma))]
    #[test_case("群馬" => Some(Prefecture::Gunma))]
    #[test_case("埼玉県" => Some(Prefecture::Saitama))]
    #[test_case("埼玉" => Some(Prefecture::Saitama))]
    #[test_case("千葉県" => Some(Prefecture::Chiba))]
    #[test_case("千葉" => Some(Prefecture::Chiba))]
    #[test_case("東京都" => Some(Prefecture::Tokyo))]
    #[test_case("東京" => Some(Prefecture::Tokyo))]
    #[test_case("神奈川県" => Some(Prefecture::Kanagawa))]
    #[test_case("神奈川" => Some(Prefecture::Kanagawa))]
    #[test_case("新潟県" => Some(Prefecture::Niigata))]
    #[test_case("新潟" => Some(Prefecture::Niigata))]
    #[test_case("富山県" => Some(Prefecture::Toyama))]
    #[test_case("富山" => Some(Prefecture::Toyama))]
    #[test_case("石川県" => Some(Prefecture::Ishikawa))]
    #[test_case("石川" => Some(Prefecture::Ishikawa))]
    #[test_case("福井県" => Some(Prefecture::Fukui))]
    #[test_case("福井" => Some(Prefecture::Fukui))]
    #[test_case("山梨県" => Some(Prefecture::Yamanashi))]
    #[test_case("山梨" => Some(Prefecture::Yamanashi))]
    #[test_case("長野県" => Some(Prefecture::Nagano))]
    #[test_case("長野" => Some(Prefecture::Nagano))]
    #[test_case("岐阜県" => Some(Prefecture::Gifu))]
    #[test_case("岐阜" => Some(Prefecture::Gifu))]
    #[test_case("静岡県" => Some(Prefecture::Shizuoka))]
    #[test_case("静岡" => Some(Prefecture::Shizuoka))]
    #[test_case("愛知県" => Some(Prefecture::Aichi))]
    #[test_case("愛知" => Some(Prefecture::Aichi))]
    #[test_case("三重県" => Some(Prefecture::Mie))]
    #[test_case("三重" => Some(Prefecture::Mie))]
    #[test_case("滋賀県" => Some(Prefecture::Shiga))]
    #[test_case("滋賀" => Some(Prefecture::Shiga))]
    #[test_case("京都府" => Some(Prefecture::Kyoto))]
    #[test_case("京都" => Some(Prefecture::Kyoto))]
    #[test_case("大阪府" => Some(Prefecture::Osaka))]
    #[test_case("大阪" => Some(Prefecture::Osaka))]
    #[test_case("兵庫県" => Some(Prefecture::Hyogo))]
    #[test_case("兵庫" => Some(Prefecture::Hyogo))]
    #[test_case("奈良県" => Some(Prefecture::Nara))]
    #[test_case("奈良" => Some(Prefecture::Nara))]
    #[test_case("和歌山県" => Some(Prefecture::Wakayama))]
    #[test_case("和歌山" => Some(Prefecture::Wakayama))]
    #[test_case("鳥取県" => Some(Prefecture::Tottori))]
    #[test_case("鳥取" => Some(Prefecture::Tottori))]
    #[test_case("島根県" => Some(Prefecture::Shimane))]
    #[test_case("島根" => Some(Prefecture::Shimane))]
    #[test_case("岡山県" => Some(Prefecture::Okayama))]
    #[test_case("岡山" => Some(Prefecture::Okayama))]
    #[test_case("広島県" => Some(Prefecture::Hiroshima))]
    #[test_case("広島" => Some(Prefecture::Hiroshima))]
    #[test_case("山口県" => Some(Prefecture::Yamaguchi))]
    #[test_case("山口" => Some(Prefecture::Yamaguchi))]
    #[test_case("徳島県" => Some(Prefecture::Tokushima))]
    #[test_case("徳島" => Some(Prefecture::Tokushima))]
    #[test_case("香川県" => Some(Prefecture::Kagawa))]
    #[test_case("香川" => Some(Prefecture::Kagawa))]
    #[test_case("愛媛県" => Some(Prefecture::Ehime))]
    #[test_case("愛媛" => Some(Prefecture::Ehime))]
    #[test_case("高知県" => Some(Prefecture::Kochi))]
    #[test_case("高知" => Some(Prefecture::Kochi))]
    #[test_case("福岡県" => Some(Prefecture::Fukuoka))]
    #[test_case("福岡" => Some(Prefecture::Fukuoka))]
    #[test_case("佐賀県" => Some(Prefecture::Saga))]
    #[test_case("佐賀" => Some(Prefecture::Saga))]
    #[test_case("長崎県" => Some(Prefecture::Nagasaki))]
    #[test_case("長崎" => Some(Prefecture::Nagasaki))]
    #[test_case("熊本県" => Some(Prefecture::Kumamoto))]
    #[test_case("熊本" => Some(Prefecture::Kumamoto))]
    #[test_case("大分県" => Some(Prefecture::Oita))]
    #[test_case("大分" => Some(Prefecture::Oita))]
    #[test_case("宮崎県" => Some(Prefecture::Miyazaki))]
    #[test_case("宮崎" => Some(Prefecture::Miyazaki))]
    #[test_case("鹿児島県" => Some(Prefecture::Kagoshima))]
    #[test_case("鹿児島" => Some(Prefecture::Kagoshima))]
    #[test_case("沖縄県" => Some(Prefecture::Okinawa))]
    #[test_case("沖縄" => Some(Prefecture::Okinawa))]
    #[test_case("None" => None)]
    fn find_by_kanji_tests(kanji: &'static str) -> Option<Prefecture> {
        Prefecture::find_by_kanji(kanji)
    }
}
