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
    /// assert_eq!(tokyo.kanji(), "東京都")
    /// ```
    pub fn kanji(self) -> &'static str {
        PREFECTURE_MAP.get(&self).unwrap().kanji
    }

    pub fn kanji_short(self) -> &'static str {
        let kanji = self.kanji();
        match self {
            Prefecture::Hokkaido => kanji,
            Prefecture::Tokyo => kanji.trim_end_matches("都"),
            Prefecture::Kyoto | Prefecture::Osaka => kanji.trim_end_matches("府"),
            _ => self.kanji().trim_end_matches("県"),
        }
    }

    fn hiragana(&self) -> &'static str {
        let data = PREFECTURE_MAP.get(&self).unwrap();
        data.kana
    }

    fn hiragana_short(&self) -> String {
        unimplemented!()
    }

    pub fn find_by_kanji(kanji: impl Into<String>) -> Self {
        unimplemented!()
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
    fn converts_to_jis_x_0401_code(prefecture: Prefecture) -> u32 {
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
    fn converts_to_kanji(prefecture: Prefecture) -> &'static str {
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
    fn converts_to_kanji_short(prefecture: Prefecture) -> &'static str {
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
    fn convert_to_hiragana(prefecture: Prefecture) -> &'static str {
        prefecture.hiragana()
    }
}
