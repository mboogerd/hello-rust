/*

- Lifetimes prevent dangling references.
References to values of an inner scope, once shared with an
outer scope, would become dangling as soon as the inner scope
ends. I.e. instead Rust fails to compile:
    
    _ does not live long enough

(subject of the reference does not live as long as the reference)

Lifetime annotation syntax.
- Lifetime annotations relate the lifetimes of multiple references to each other
- Lifetime syntax is about connecting the lifetimes of various arguments and return values of functions
- Does not change any lifetime, just demands alignment.
- go on the function signature, not in the body
&i32        // reference
&'a i32     // reference with lifetime identifier
&'a mut i32 // mutable reference with lifetime identifier
*/

// -'a will stand for the shortest lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

/*
Lifetime elision rules - particular lifetime scenarios that
the rust compiler will automatically recognize.

- each parameter that is a reference gets its own lifetime parameter.
  in other words, a function with one parameter gets one lifetime parameter,
  a function with two arguments gets two separate lifetime parameters.
- if there is exactly one input lifetime parameter, that lifetime is
  assigned to all output parameters
- if there are multiple input lifetime parameters, but one of them
  is &self or &mut self because this is a method, then the lifetime of self
  is assigned to all output parameters.
*/

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main() {
    // static lifetime equals the lifetime of the application
    let longString: &'static str = "long string is long";
    let string1 = String::from(longString);

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}