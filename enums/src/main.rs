#[derive(Debug)]
enum UsState {
    Albama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

fn main() {
    println!("Cents = {}", Coin::Penny.value_in_cents());
    println!("Cents = {}", Coin::Nickel.value_in_cents());
    println!("Cents = {}", Coin::Dime.value_in_cents());
    println!("Cents = {}", Coin::Quarter(UsState::Albama).value_in_cents());
    println!("Cents = {}", Coin::Quarter(UsState::Alaska).value_in_cents());
}
