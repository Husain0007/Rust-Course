enum IpAddrKind {
    //enum variants
    V4(String), 
    V6(String),
}
enum IpAddrTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] // to make it easy to print an enum
enum USState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}
enum Coin{
    Penny,
    Nickel,
    Dime, 
    Quarter(USState),
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let localhostv4: IpAddrTypes = IpAddrTypes::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    println!("{}",x+y.unwrap_or(0));

    // When z is None
    let z: Option<i8> = None;
    println!("{}",x+z.unwrap_or(0));

    // Using Matching Expression
    value_in_cents(Coin::Quarter(USState::Alaska));

    // Combining the option enum and match expression

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
fn route(ip_kind: IpAddrKind){}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin:: Penny => 1,
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin:: Quarter(state ) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        //None => None,
        Some(i) => Some(i+1),
        _ => None,
    }
}
