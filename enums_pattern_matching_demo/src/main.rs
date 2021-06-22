// enum IppAddrKind {
//     V4,
//     V6,
// }
//
// struct IppAddr {
//     kind: IppAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => {
            println!("Lucky Penny !!");
            1
        }
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter(state) => {
            println!("The State is {:?}", state);
            25
        }
    }
}

fn main() {
    // let four = IppAddrKind::V4;
    // let six = IppAddrKind::V6;
    //
    // let home = IppAddr {
    //     kind: IppAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // println!("{}, {}, {}", four, six, home);
}
