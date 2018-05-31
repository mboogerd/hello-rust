fn main() {
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("foobar? {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    // push character
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
    // the compiler can (deref) coerce the &String argument into a &str

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // a bit unwieldy
    let s = s1 + "-" + &s2 + "-" + &s3;
    // format to the rescue
    let s1 = String::from("tic"); // (original s1 moved to s)
    let s = format!("{}-{}-{}", s1, s2, s3);

    // use ranges carefully, it can crash your program...
    // UTF8 String characters take two bytes each
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; // PANIC

    // preferably, use `chars` instead
    for c in hello.chars() {
        println!("{}", c);
    }
}