fn main() {
    let s = String::from("hello world");

    println!("{}", s);
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // // Range references consist of:
    // // - a pointer to the location in the string / array
    // // - the length of their fragment

    // if you want to start at the first index (zero),
    let _from_beginning = &s[0..2];
    // you can drop the value before the two periods
    let _from_beginning = &s[..2];

    // if you want to include the last byte
    let len = s.len();
    let _until_end = &s[3..len];
    // you can drop the trailing number
    let _until_end = &s[3..];

    // finally, if you want a slice of the whole string
    let _all = &s[0..len];
    // you can drop both
    let _all = &s[..];

    let first = first_word(&s);
    println!("First Word: {}", first);

    // Would produce error:
    // cannot borrow immutable local variable `s` as mutable
    // s.clear();

    let s2 = s.clone();
    donothing(&s2);
    // s2.clear(); // still produces that error

    // type of String literals is also slice:
    let _s: &str = "somestring";
}

// The type of string slice is `&str`
fn first_word(s: &String) -> &str {
    // Using &str is more general, as taking a slice
    // is possible and cheap, whereas making a String
    // literal from a slice is not?
    first_word_improved(&s[..])
}

fn donothing(_s: &String) {}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}