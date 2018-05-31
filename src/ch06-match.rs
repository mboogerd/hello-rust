enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

fn value_in_cents(coin: Coin) -> u32 {
    // match expressions evaluates the first expression
    // whose pattern in a sequence of
    // `[pattern] => [expression]` phrases
    // satisfies its input.
    // Also comes with exhaustion checking (non-exhaustion => compile error!)
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Curly braces are optional
        // matches allow destructuring
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        // _ => 0 // We could add a default here, but we don't need one
    }
}

fn main() {
    println!("Value in Cents (Penny) {}",
        value_in_cents(Coin::Quarter(UsState::Alabama)));
}