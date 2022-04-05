use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::prefectures::Prefecture;

pub(crate) struct PrefectureData {
    pub jis_x_0401_code: u32,
    pub kanji: &'static str,
    pub hiragana: &'static str,
    pub romaji: &'static str,
}

impl PrefectureData {
    fn new(
        jis_x_0401_code: u32,
        kanji: &'static str,
        hiragana: &'static str,
        romaji: &'static str,
    ) -> Self {
        Self {
            jis_x_0401_code,
            kanji,
            hiragana,
            romaji,
        }
    }
}

pub(crate) static PREFECTURE_MAP: Lazy<HashMap<Prefecture, PrefectureData>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        Prefecture::Hokkaido,
        PrefectureData::new(
            Prefecture::Hokkaido as u32,
            "北海道",
            "ほっかいどう",
            "hokkaido",
        ),
    );
    map.insert(
        Prefecture::Aomori,
        PrefectureData::new(
            Prefecture::Aomori as u32,
            "青森県",
            "あおもりけん",
            "aomori",
        ),
    );
    map.insert(
        Prefecture::Iwate,
        PrefectureData::new(Prefecture::Iwate as u32, "岩手県", "いわてけん", "iwate"),
    );
    map.insert(
        Prefecture::Miyagi,
        PrefectureData::new(Prefecture::Miyagi as u32, "宮城県", "みやぎけん", "miyagi"),
    );
    map.insert(
        Prefecture::Akita,
        PrefectureData::new(Prefecture::Akita as u32, "秋田県", "あきたけん", "akita"),
    );
    map.insert(
        Prefecture::Yamagata,
        PrefectureData::new(
            Prefecture::Yamagata as u32,
            "山形県",
            "やまがたけん",
            "yamagata",
        ),
    );
    map.insert(
        Prefecture::Fukushima,
        PrefectureData::new(
            Prefecture::Fukushima as u32,
            "福島県",
            "ふくしまけん",
            "fukushima",
        ),
    );
    map.insert(
        Prefecture::Ibaraki,
        PrefectureData::new(
            Prefecture::Ibaraki as u32,
            "茨城県",
            "いばらきけん",
            "ibaraki",
        ),
    );
    map.insert(
        Prefecture::Tochigi,
        PrefectureData::new(
            Prefecture::Tochigi as u32,
            "栃木県",
            "とちぎけん",
            "tochigi",
        ),
    );
    map.insert(
        Prefecture::Gunma,
        PrefectureData::new(Prefecture::Gunma as u32, "群馬県", "ぐんまけん", "gunma"),
    );
    map.insert(
        Prefecture::Saitama,
        PrefectureData::new(
            Prefecture::Saitama as u32,
            "埼玉県",
            "さいたまけん",
            "saitama",
        ),
    );
    map.insert(
        Prefecture::Chiba,
        PrefectureData::new(Prefecture::Chiba as u32, "千葉県", "ちばけん", "chiba"),
    );
    map.insert(
        Prefecture::Tokyo,
        PrefectureData::new(Prefecture::Tokyo as u32, "東京都", "とうきょうと", "tokyo"),
    );
    map.insert(
        Prefecture::Kanagawa,
        PrefectureData::new(
            Prefecture::Kanagawa as u32,
            "神奈川県",
            "かながわけん",
            "kanagawa",
        ),
    );
    map.insert(
        Prefecture::Niigata,
        PrefectureData::new(
            Prefecture::Niigata as u32,
            "新潟県",
            "にいがたけん",
            "niigata",
        ),
    );
    map.insert(
        Prefecture::Toyama,
        PrefectureData::new(Prefecture::Toyama as u32, "富山県", "とやまけん", "toyama"),
    );
    map.insert(
        Prefecture::Ishikawa,
        PrefectureData::new(
            Prefecture::Ishikawa as u32,
            "石川県",
            "いしかわけん",
            "ishikawa",
        ),
    );
    map.insert(
        Prefecture::Fukui,
        PrefectureData::new(Prefecture::Fukui as u32, "福井県", "ふくいけん", "fukui"),
    );
    map.insert(
        Prefecture::Yamanashi,
        PrefectureData::new(
            Prefecture::Yamanashi as u32,
            "山梨県",
            "やまなしけん",
            "yamanashi",
        ),
    );
    map.insert(
        Prefecture::Nagano,
        PrefectureData::new(Prefecture::Nagano as u32, "長野県", "ながのけん", "nagano"),
    );
    map.insert(
        Prefecture::Gifu,
        PrefectureData::new(Prefecture::Gifu as u32, "岐阜県", "ぎふけん", "gifu"),
    );
    map.insert(
        Prefecture::Shizuoka,
        PrefectureData::new(
            Prefecture::Shizuoka as u32,
            "静岡県",
            "しずおかけん",
            "shizuoka",
        ),
    );
    map.insert(
        Prefecture::Aichi,
        PrefectureData::new(Prefecture::Aichi as u32, "愛知県", "あいちけん", "aichi"),
    );
    map.insert(
        Prefecture::Mie,
        PrefectureData::new(Prefecture::Mie as u32, "三重県", "みえけん", "mie"),
    );
    map.insert(
        Prefecture::Shiga,
        PrefectureData::new(Prefecture::Shiga as u32, "滋賀県", "しがけん", "shiga"),
    );
    map.insert(
        Prefecture::Kyoto,
        PrefectureData::new(Prefecture::Kyoto as u32, "京都府", "きょうとふ", "kyoto"),
    );
    map.insert(
        Prefecture::Osaka,
        PrefectureData::new(Prefecture::Osaka as u32, "大阪府", "おおさかふ", "osaka"),
    );
    map.insert(
        Prefecture::Hyogo,
        PrefectureData::new(Prefecture::Hyogo as u32, "兵庫県", "ひょうごけん", "hyogo"),
    );
    map.insert(
        Prefecture::Nara,
        PrefectureData::new(Prefecture::Nara as u32, "奈良県", "ならけん", "nara"),
    );
    map.insert(
        Prefecture::Wakayama,
        PrefectureData::new(
            Prefecture::Wakayama as u32,
            "和歌山県",
            "わかやまけん",
            "wakayama",
        ),
    );
    map.insert(
        Prefecture::Tottori,
        PrefectureData::new(
            Prefecture::Tottori as u32,
            "鳥取県",
            "とっとりけん",
            "tottori",
        ),
    );
    map.insert(
        Prefecture::Shimane,
        PrefectureData::new(
            Prefecture::Shimane as u32,
            "島根県",
            "しまねけん",
            "shimane",
        ),
    );
    map.insert(
        Prefecture::Okayama,
        PrefectureData::new(
            Prefecture::Okayama as u32,
            "岡山県",
            "おかやまけん",
            "okayama",
        ),
    );
    map.insert(
        Prefecture::Hiroshima,
        PrefectureData::new(
            Prefecture::Hiroshima as u32,
            "広島県",
            "ひろしまけん",
            "hiroshima",
        ),
    );
    map.insert(
        Prefecture::Yamaguchi,
        PrefectureData::new(
            Prefecture::Yamaguchi as u32,
            "山口県",
            "やまぐちけん",
            "yamaguchi",
        ),
    );
    map.insert(
        Prefecture::Tokushima,
        PrefectureData::new(
            Prefecture::Tokushima as u32,
            "徳島県",
            "とくしまけん",
            "tokushima",
        ),
    );
    map.insert(
        Prefecture::Kagawa,
        PrefectureData::new(Prefecture::Kagawa as u32, "香川県", "かがわけん", "kagawa"),
    );
    map.insert(
        Prefecture::Ehime,
        PrefectureData::new(Prefecture::Ehime as u32, "愛媛県", "えひめけん", "ehime"),
    );
    map.insert(
        Prefecture::Kochi,
        PrefectureData::new(Prefecture::Kochi as u32, "高知県", "こうちけん", "kochi"),
    );
    map.insert(
        Prefecture::Fukuoka,
        PrefectureData::new(
            Prefecture::Fukuoka as u32,
            "福岡県",
            "ふくおかけん",
            "fukuoka",
        ),
    );
    map.insert(
        Prefecture::Saga,
        PrefectureData::new(Prefecture::Saga as u32, "佐賀県", "さがけん", "saga"),
    );
    map.insert(
        Prefecture::Nagasaki,
        PrefectureData::new(
            Prefecture::Nagasaki as u32,
            "長崎県",
            "ながさきけん",
            "nagasaki",
        ),
    );
    map.insert(
        Prefecture::Kumamoto,
        PrefectureData::new(
            Prefecture::Kumamoto as u32,
            "熊本県",
            "くまもとけん",
            "kumamoto",
        ),
    );
    map.insert(
        Prefecture::Oita,
        PrefectureData::new(Prefecture::Oita as u32, "大分県", "おおいたけん", "oita"),
    );
    map.insert(
        Prefecture::Miyazaki,
        PrefectureData::new(
            Prefecture::Miyazaki as u32,
            "宮崎県",
            "みやざきけん",
            "miyazaki",
        ),
    );
    map.insert(
        Prefecture::Kagoshima,
        PrefectureData::new(
            Prefecture::Kagoshima as u32,
            "鹿児島県",
            "かごしまけん",
            "kagoshima",
        ),
    );
    map.insert(
        Prefecture::Okinawa,
        PrefectureData::new(
            Prefecture::Okinawa as u32,
            "沖縄県",
            "おきなわけん",
            "okinawa",
        ),
    );
    map
});
