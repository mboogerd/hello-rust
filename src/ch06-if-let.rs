fn main() {
    let some_u8_value = Some(0u8);

    // Single pattern match for side-effect
    if let Some(3) = some_u8_value {
        println!("three");
    } // otherwise defaults to no action

    if let Some(2) = some_u8_value {
        println!("two?");
    } else {
        println!("wasn't two!");
    }
    
    /* Syntax sugar for:
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    */
}