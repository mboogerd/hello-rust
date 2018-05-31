fn main () {
    // Stack
    // - last in, first out
    // - fixed size elements
    // - cheap to 'find/clean' space

    // Heap
    // - unknown / dynamically sized elements
    // - access requires a pointer indirection (slower)
    // - Allocations take time proportional to allocation-size


    // Ownership (tracking Heap allocated data)
    // - Each value in Rust has a variable that's called its owner
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    {   // `_s` is not valid here (undefined)
        let _s = "hello";
        // `_s` is valid from here (in scope)
    }   // `_s` is no longer valid (out of scope)

    // Heap allocated, mutable String
    {
        let mut s = String::from("hello");
        s.push_str(" , world!");
        println!("{}", s);
    }
    // s is no longer valid, and has been 'drop'ped (deallocated)
    // In C++, this pattern of deallocating resources at the end
    // of an item’s lifetime is sometimes called
    // Resource Acquisition Is Initialization (RAII)

    let x = String::from("hello");
    // x consists of:
    // - three stack elements
    //  - pointer to heap for string data
    //  - number of character elements
    //  - capacity of the allocated buffer on the heap
    // - a heap array with a sequence of characters

    let y = x;
    // y consists of a _copy_ of the stack part of `x`, i.e.
    // it points to the original heap data.

    // To prevent safety from double freeing, `x` is invalidated
    // so effectively the statement _moves_ the stack data from
    // x to y. I.e. it re-labels the data.

    // Cloning creates a copy of stack and heap data, preserving
    // the validity of the source variable.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // values with types including the Copy trait are copied when re-assigned
    // if a type or a constituent of the type has the Drop trait, it cannot have Copy
    // Copy'able datatypes are the stack-based ones:
    // - integers, boolean, char, floating point, but also:
    // - tuples; if their constituents are Copy (stack-based)

    /* Ownership and Functions */

    {
        let s = String::from("hello");  // s comes into scope.

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here.

        let x = 5;                      // x comes into scope.

        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use x afterward.
    }
    // Here, x goes out of scope, then s. But since s's value was moved, nothing
    // special happens.

    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1.

        let s2 = String::from("hello");     // s2 comes into scope.

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3.
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}