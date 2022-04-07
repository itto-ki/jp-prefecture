use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::prefectures::Prefecture;

pub(crate) struct PrefectureData {
    pub kanji: &'static str,
    pub hiragana: &'static str,
    pub katakana: &'static str,
    pub english: &'static str,
}

impl PrefectureData {
    fn new(
        kanji: &'static str,
        hiragana: &'static str,
        katakana: &'static str,
        english: &'static str,
    ) -> Self {
        Self {
            kanji,
            hiragana,
            katakana,
            english,
        }
    }
}

pub(crate) static PREFECTURE_MAP: Lazy<HashMap<Prefecture, PrefectureData>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        Prefecture::Hokkaido,
        PrefectureData::new("北海道", "ほっかいどう", "ホッカイドウ", "hokkaido"),
    );
    map.insert(
        Prefecture::Aomori,
        PrefectureData::new("青森県", "あおもりけん", "アオモリケン", "aomori"),
    );
    map.insert(
        Prefecture::Iwate,
        PrefectureData::new("岩手県", "いわてけん", "イワテケン", "iwate"),
    );
    map.insert(
        Prefecture::Miyagi,
        PrefectureData::new("宮城県", "みやぎけん", "ミヤギケン", "miyagi"),
    );
    map.insert(
        Prefecture::Akita,
        PrefectureData::new("秋田県", "あきたけん", "アキタケン", "akita"),
    );
    map.insert(
        Prefecture::Yamagata,
        PrefectureData::new("山形県", "やまがたけん", "ヤマガタケン", "yamagata"),
    );
    map.insert(
        Prefecture::Fukushima,
        PrefectureData::new("福島県", "ふくしまけん", "フクシマケン", "fukushima"),
    );
    map.insert(
        Prefecture::Ibaraki,
        PrefectureData::new("茨城県", "いばらきけん", "イバラキケン", "ibaraki"),
    );
    map.insert(
        Prefecture::Tochigi,
        PrefectureData::new("栃木県", "とちぎけん", "トチギケン", "tochigi"),
    );
    map.insert(
        Prefecture::Gunma,
        PrefectureData::new("群馬県", "ぐんまけん", "グンマケン", "gunma"),
    );
    map.insert(
        Prefecture::Saitama,
        PrefectureData::new("埼玉県", "さいたまけん", "サイタマケン", "saitama"),
    );
    map.insert(
        Prefecture::Chiba,
        PrefectureData::new("千葉県", "ちばけん", "チバケン", "chiba"),
    );
    map.insert(
        Prefecture::Tokyo,
        PrefectureData::new("東京都", "とうきょうと", "トウキョウト", "tokyo"),
    );
    map.insert(
        Prefecture::Kanagawa,
        PrefectureData::new("神奈川県", "かながわけん", "カナガワケン", "kanagawa"),
    );
    map.insert(
        Prefecture::Niigata,
        PrefectureData::new("新潟県", "にいがたけん", "ニイガタケン", "niigata"),
    );
    map.insert(
        Prefecture::Toyama,
        PrefectureData::new("富山県", "とやまけん", "トヤマケン", "toyama"),
    );
    map.insert(
        Prefecture::Ishikawa,
        PrefectureData::new("石川県", "いしかわけん", "イシカワケン", "ishikawa"),
    );
    map.insert(
        Prefecture::Fukui,
        PrefectureData::new("福井県", "ふくいけん", "フクイケン", "fukui"),
    );
    map.insert(
        Prefecture::Yamanashi,
        PrefectureData::new("山梨県", "やまなしけん", "ヤマナシケン", "yamanashi"),
    );
    map.insert(
        Prefecture::Nagano,
        PrefectureData::new("長野県", "ながのけん", "ナガノケン", "nagano"),
    );
    map.insert(
        Prefecture::Gifu,
        PrefectureData::new("岐阜県", "ぎふけん", "ギフケン", "gifu"),
    );
    map.insert(
        Prefecture::Shizuoka,
        PrefectureData::new("静岡県", "しずおかけん", "シズオカケン", "shizuoka"),
    );
    map.insert(
        Prefecture::Aichi,
        PrefectureData::new("愛知県", "あいちけん", "アイチケン", "aichi"),
    );
    map.insert(
        Prefecture::Mie,
        PrefectureData::new("三重県", "みえけん", "ミエケン", "mie"),
    );
    map.insert(
        Prefecture::Shiga,
        PrefectureData::new("滋賀県", "しがけん", "シガケン", "shiga"),
    );
    map.insert(
        Prefecture::Kyoto,
        PrefectureData::new("京都府", "きょうとふ", "キョウトフ", "kyoto"),
    );
    map.insert(
        Prefecture::Osaka,
        PrefectureData::new("大阪府", "おおさかふ", "オオサカフ", "osaka"),
    );
    map.insert(
        Prefecture::Hyogo,
        PrefectureData::new("兵庫県", "ひょうごけん", "ヒョウゴケン", "hyogo"),
    );
    map.insert(
        Prefecture::Nara,
        PrefectureData::new("奈良県", "ならけん", "ナラケン", "nara"),
    );
    map.insert(
        Prefecture::Wakayama,
        PrefectureData::new("和歌山県", "わかやまけん", "ワカヤマケン", "wakayama"),
    );
    map.insert(
        Prefecture::Tottori,
        PrefectureData::new("鳥取県", "とっとりけん", "トットリケン", "tottori"),
    );
    map.insert(
        Prefecture::Shimane,
        PrefectureData::new("島根県", "しまねけん", "シマネケン", "shimane"),
    );
    map.insert(
        Prefecture::Okayama,
        PrefectureData::new("岡山県", "おかやまけん", "オカヤマケン", "okayama"),
    );
    map.insert(
        Prefecture::Hiroshima,
        PrefectureData::new("広島県", "ひろしまけん", "ヒロシマケン", "hiroshima"),
    );
    map.insert(
        Prefecture::Yamaguchi,
        PrefectureData::new("山口県", "やまぐちけん", "ヤマグチケン", "yamaguchi"),
    );
    map.insert(
        Prefecture::Tokushima,
        PrefectureData::new("徳島県", "とくしまけん", "トクシマケン", "tokushima"),
    );
    map.insert(
        Prefecture::Kagawa,
        PrefectureData::new("香川県", "かがわけん", "カガワケン", "kagawa"),
    );
    map.insert(
        Prefecture::Ehime,
        PrefectureData::new("愛媛県", "えひめけん", "エヒメケン", "ehime"),
    );
    map.insert(
        Prefecture::Kochi,
        PrefectureData::new("高知県", "こうちけん", "コウチケン", "kochi"),
    );
    map.insert(
        Prefecture::Fukuoka,
        PrefectureData::new("福岡県", "ふくおかけん", "フクオカケン", "fukuoka"),
    );
    map.insert(
        Prefecture::Saga,
        PrefectureData::new("佐賀県", "さがけん", "サガケン", "saga"),
    );
    map.insert(
        Prefecture::Nagasaki,
        PrefectureData::new("長崎県", "ながさきけん", "ナガサキケン", "nagasaki"),
    );
    map.insert(
        Prefecture::Kumamoto,
        PrefectureData::new("熊本県", "くまもとけん", "クマモトケン", "kumamoto"),
    );
    map.insert(
        Prefecture::Oita,
        PrefectureData::new("大分県", "おおいたけん", "オオイタケン", "oita"),
    );
    map.insert(
        Prefecture::Miyazaki,
        PrefectureData::new("宮崎県", "みやざきけん", "ミヤザキケン", "miyazaki"),
    );
    map.insert(
        Prefecture::Kagoshima,
        PrefectureData::new("鹿児島県", "かごしまけん", "カゴシマケン", "kagoshima"),
    );
    map.insert(
        Prefecture::Okinawa,
        PrefectureData::new("沖縄県", "おきなわけん", "オキナワケン", "okinawa"),
    );
    map
});
