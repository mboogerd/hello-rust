#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block
impl Rectangle {
    // Methods are functions defined in the context of a struct
    // Their first parameter is always `self`, an instance of the struct
    // we don’t want to take ownership (using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and we want to prevent the caller from using the original instance after the transformation.)
    // we just want to read the data in the struct, not write to it
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // An associated function is one that does not take a self
    // instance parameter. Use [struct-type]::[fn name] syntax to invoke
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Multiple implementation blocks are valid, enabling extension methods
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { width: 40, height: 20 };
    
    println!("rect2 > rect1: {}", rect2.can_hold(&rect1));
    println!("rect1 > rect2: {}", rect1.can_hold(&rect2));

    println!("Square size 40: {:?}", Rectangle::square(40));
}