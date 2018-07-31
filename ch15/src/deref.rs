/// Smart pointers
/// ==============
/// 
/// Smart pointer have additional metadata and capabilities
/// besides their pointed-ness.
/// 
/// References only borrow data, in many cases, smart pointers
/// own the data they point to.
/// 
/// Smart pointers are structs that implement the Deref and
/// Drop traits
/// 
/// - Box<T> => Allocate values on the heap without performance
///   overhead. Useful for dynamically sized data, preventing
///   copies when transfering ownership
/// - Rc<T> => Reference counted type for multiple ownership
/// - Ref<T> and RefMut<T> accessed through RefCell<T>
///   enforcing runtime rather than compile time borrowing.
/// 
/// (Add our own for signal like programming?)

use std::ops::Deref;
use List::{Cons, Nil};

fn main() {

    // Boxes
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let mb = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // dereferencing boxes and references produce the same result
    assert_eq!(5, *z);
    // dereferencing our MyBox type
    assert_eq!(5, *mb);

    // Deref coercion happens automatically when we pass a
    // reference to a particular type’s value as an argument
    // to a function or method that doesn’t match the parameter
    // type in the function or method definition
    let x = MyBox::new(String::from("Rust"));
    hello(&x);

    // deref coercion also kicks in when assigning
    let _y: &str = &x;

    // Macros require some extra care though, as this won't compile
    // println!("Hello, {}!", &x);

    // More precisely, deref coercion takes place in three cases:
    // 1) From &T to &U when T: Deref<Target=U>
    // 2) From &mut T to &mut U when T: DerefMut<Target=U>
    // 3) From &mut T to &U when T: Deref<Target=U>
    // (3) because a mutable reference is allowed to be
    // treated as an immutable one when borrowed, but not
    // the opposite.
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    // Recursive but known size because a pointer takes fixed size
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}