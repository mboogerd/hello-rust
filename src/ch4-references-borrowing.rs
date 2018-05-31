fn main () {
    let s1 = String::from("hello");

    // reference to `s1` is passed to `calculate_length`
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&s1);

    let mut s2 = s1.clone();
    // mutable references have one big restriction: you can only have one
    // mutable reference to a particular piece of data in a
    // particular scope. i.e. no simultaneously existing mut refs.

    // Here is one
    change_for_real(&mut s2);
    // And now it's gone

    {
        // Here is a new one
        let r1 = &mut s2;
        // This code would thus fail:
        // (cannot borrow `s2` as mutable more than once at a time)
        // let r2 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s2;

    // Combining mutable and immutable references is forbidden.
    // We cannot guarantee the immutability of a value if there
    // is a simultaneous mutable reference
    // let r3 = &s2;
    
    // Dropping yourself doesn't prevent compilation errors
    // drop(r2);
    // let r3 = &s2;


    /* Dangling references */
    // The potential of dangling references is prevented
    // by means of compilation error
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // calculate_length does not take ownership of `s`
    // s is only a pointer to the original value that will be
    // freed once access borrowing is no longer needed.
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


fn change(some_string: &String) {
    // Does not compile (cannot borrow as mutable)
    // some_string.push_str(", world");
}

fn change_for_real(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// ^^ fixed 
fn dangle() -> String {
    let s = String::from("hello");
    s // we return the actual value
} //  Ownership is moved out, and nothing is deallocated.

/* RULES OF REFERENCES
1) At any given time, you can have _either_ but not both of:
    - One mutable reference.
    - Any number of immutable references.

2) References must always be valid
*/