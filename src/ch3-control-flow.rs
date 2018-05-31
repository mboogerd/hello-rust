fn main() {
    // Control flow
    let number = 3;

    // if-else-if statement; if expects an expression that
    // evaluates to boolean.
    if number < 5 {
        println!("{} < 5", number);
    } else if number > 5 {
        println!("{} > 5", number);
    } else {
        println!("{} = 5", number);
    }

    // this would be a compile error
    // if 1 {}

    // if is an expression
    let number = if true { 5 } else { 6 };
    println!("if true 5 else 6: {}", number);

    // this is 'of course' a compile error (no sum types)
    // let something = if false { 5 } else { "six" }

    // Yaay, loops...
    loop {
        println!("Infinite run!!!");
        // if it were not for this break
        break;
    }
    println!("Infinite run stopped!!!");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // we could use it to loop over arrays
    let a = [10, 20, 30, 40];
    let mut index = 0;
    while index < 4 {
        println!("The value on position {} is {}", index + 1, a[index]);
        index = index + 1;
    }

    // but Rust has iterators, shiney! (you don't get access to the 
    // loop counter though)
    for element in a.iter() {
        println!("[Iterator] The value on is {}", element);
    }

    // we can define ranges and loop over those
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
