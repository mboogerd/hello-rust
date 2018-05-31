// use gui::{Screen, Button};
// use gui::Draw;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// A generic type parameter can only be substituted with one
// concrete type at a time, whereas trait objects allow for
// multiple concrete types to fill in for the trait object at
// runtime

// SAFE for trait objects: no type parameters
pub trait Draw {
    // SAFE for trait objects: return type isn't Self
    fn draw(&self);
}

// Trait objects can be used for any type, as long as it implements Draw
// However, it uses dynamic dispatch, adding lookup, preventing inlining
pub struct Screen {
    // Allow for any concrete type that implements Draw
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Trait bounds can only be substituted with one concrete type at a time
// However it uses static dispatch \o/
pub struct ScreenT<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenT<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
        println!("Drawing {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
        println!("Drawing {:?}", self);
    }
}