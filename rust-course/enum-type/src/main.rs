#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

#[derive(Debug)]
enum PokerCardEmo {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
    dbg!(c1);
    dbg!(c2);

    let c1 = PokerCardEmo::Spades(5);
    let c2 = PokerCardEmo::Diamonds('A');
    dbg!(c1);
    dbg!(c2);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);
    dbg!(m1);
    dbg!(m2);
    dbg!(m3);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}
