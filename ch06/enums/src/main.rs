enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

// enums can have methods
impl Message {
    fn call(&self) {
        // code
    }
}

#[derive(Debug)]
enum Province {
    BC,
    AB,
    SK,
    MB,
    ON,
    QB,
    NS,
    NB,
    PEI,
    NL,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province),
    Loonie,
    Toonie,
}

fn main() {
    // declaring enums
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    // passing as arguments
    route(home);
    route(loopback);

    // using enum method
    let m = Message::Write(String::from("hello"));
    m.call();

    // use pattern matching example
    let my_quarter = Coin::Quarter(Province::AB);
    value_in_cents(my_quarter);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    }

    let coin = Coin::Penny;

    let mut count = 0;
    if let Coin::Quarter(province) = coin {
        println!("Provincial quarter from {:?}", province);
    } else {
        count += 1;
    }
}

// example of function wtih enum parameter
fn route(ip_kind: IpAddrKind) {
    // code
}

// pattern matching examples
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province) => {
            println!("Provincial quarter from {:?}!", province);
            25
        }
        Coin::Loonie => 100,
        Coin::Toonie => 200,
    }
}
