use jp_prefecture::prefectures::Prefecture;

fn main() {
    let hokkaido = Prefecture::Hokkaido;

    consume(hokkaido);
    consume(hokkaido);
}

fn consume(pref: Prefecture) {
    println!("{:?}", pref.code());
}
