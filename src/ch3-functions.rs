fn main() {
    println!("Hello World");
    another_function(5, 6);

    // statement
    let y = 6;

    // a statement cannot be used as an expression
    // as it produces effects, but do not return values
    // let x = (let y = 6);

    // However, this block forms an expression
    let y = {
        let x = 3;
        // Because this part is an expression, which can be
        // seen by its lack of terminal semicolon!
        x + 1
    };
    println!("y should be 4, is it? {}", y);

    let z = five();
    println!("The value of evaluating five() is {}", z);

    let one = 1;
    println!("1 incremented = {}", increment(one));
}

// Functions can be defined before and after calling-sites.
fn another_function(x: i32, y: i32) {
    println!("Xs' value is {}", x);
    println!("Ys' value is {}", y);
}

// Functions can return a result; their return type is
// set by '-> [type]' after the function definition
fn five() -> i32 {
    // This is the expression whose value we want to return
    5
}

// Functions can both receive AND return values (who knew!)
fn increment(i: i32) -> i32 {
    i + 1
}