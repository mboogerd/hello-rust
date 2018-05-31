use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Unrecoverable error
    // panic!("crash and burn");

    // Unrecoverable out of bounds panic
    // let v = vec![1, 2, 3];
    // v[99];

    /*
    like the Option enum, the Result enum and its variants
    have been imported in the prelude, so we don’t need to
    specify Result:: before the Ok and Err variants in the
    match arms.
    */

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // The ref in the pattern is needed so error is not moved
        // into the guard condition but is merely referenced by it
        // Otherwise: cannot bind by-move into a pattern guard.
        // & matches a reference and gives us its value,
        // but `ref` matches a value and gives us a reference to it.
        Err(ref error) if /* match guard */ error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but errored: {:?}",
                        e
                    )
                }
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        }
    };

    // Unwrap gets the Ok value if it exists or panics otherwise
    let f = File::open("hello.txt").unwrap();
    // Expect unwraps but adds an explanatory message to the error
    // if one occurs
    let f = File::open("hello.txt").expect("file could not be opened");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // ? makes the above operate like:
    /*
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    */
    // There is a difference:
    // error values used with ? go through the from function,
    // defined in the From trait in the standard library,
    // which is used to convert errors from one type into another
    //  When the question mark calls the from function, the error
    // type received is converted into the error type defined
    // in the return type of the current function. This is
    // useful when a function returns one error type to
    // represent all the ways a function might fail, even if
    // parts might fail for many different reasons. As long as
    // each error type implements the from function to define
    // how to convert itself to the returned error type, the
    // question mark operator takes care of the conversion
    // automatically.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/* 
Good page on general guidelines for recoverable vs unrecoverable errors
https://doc.rust-lang.org/book/second-edition/ch09-03-to-panic-or-not-to-panic.html

Fine places for unrecoverables:
- Examples / prototype code
- Tests
- When you have more information than the compiler

For production code, it can be used to handle broken invariants / contradictory / missing data under the condition:
- Bad state is truly unexpected
- The consecutive code needs to rely on not being in a bad state
- No good way of encoding the error

Panicking when a contract is violated makes sense because a contract violation always indicates a caller-side bug
*/

/* SMART CONSTRUCTORS */
pub struct Guess {
    // private! therefore Guess cannot be instantiated outside `impl`
    // nor can the value of an existing instance be changed
    value: u32,
}

impl Guess {
    // Smart constructor that validates
    // Consider an Error alternative and an unrecoverable reusage of that
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    // getter to access the private value
    pub fn value(&self) -> u32 {
        self.value
    }
}