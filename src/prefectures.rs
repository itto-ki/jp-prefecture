/// Japanese prefectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn jis_x_0401_code(self) -> u32 {
        self as u32
    }

    pub fn kanji(self) -> &'static str {
        match self {
            Prefecture::Hokkaido => "北海道",
            Prefecture::Aomori => "青森県",
            Prefecture::Iwate => "岩手県",
            Prefecture::Miyagi => "宮城県",
            Prefecture::Akita => "秋田県",
            Prefecture::Yamagata => "山形県",
            Prefecture::Fukushima => "福島県",
            Prefecture::Ibaraki => "茨城県",
            Prefecture::Tochigi => "栃木県",
            Prefecture::Gunma => "群馬県",
            Prefecture::Saitama => "埼玉県",
            Prefecture::Chiba => "千葉県",
            Prefecture::Tokyo => "東京都",
            Prefecture::Kanagawa => "神奈川県",
            Prefecture::Niigata => "新潟県",
            Prefecture::Toyama => "富山県",
            Prefecture::Ishikawa => "石川県",
            Prefecture::Fukui => "福井県",
            Prefecture::Yamanashi => "山梨県",
            Prefecture::Nagano => "長野県",
            Prefecture::Gifu => "岐阜県",
            Prefecture::Shizuoka => "静岡県",
            Prefecture::Aichi => "愛知県",
            Prefecture::Mie => "三重県",
            Prefecture::Shiga => "滋賀県",
            Prefecture::Kyoto => "京都府",
            Prefecture::Osaka => "大阪府",
            Prefecture::Hyogo => "兵庫県",
            Prefecture::Nara => "奈良県",
            Prefecture::Wakayama => "和歌山県",
            Prefecture::Tottori => "鳥取県",
            Prefecture::Shimane => "島根県",
            Prefecture::Okayama => "岡山県",
            Prefecture::Hiroshima => "広島県",
            Prefecture::Yamaguchi => "山口県",
            Prefecture::Tokushima => "徳島県",
            Prefecture::Kagawa => "香川県",
            Prefecture::Ehime => "愛媛県",
            Prefecture::Kochi => "高知県",
            Prefecture::Fukuoka => "福岡県",
            Prefecture::Saga => "佐賀県",
            Prefecture::Nagasaki => "長崎県",
            Prefecture::Kumamoto => "熊本県",
            Prefecture::Oita => "大分県",
            Prefecture::Miyazaki => "宮崎県",
            Prefecture::Kagoshima => "鹿児島県",
            Prefecture::Okinawa => "沖縄県",
        }
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

    fn kana(&self) -> String {
        unimplemented!()
    }

    fn kana_short(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
}
