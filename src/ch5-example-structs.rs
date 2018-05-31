#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Simple Area: {}", area_simple(width1, height1));
    println!("Tupled Area: {}", area_tupled((width1, height1)));
    
    let rect = Rectangle {
        width: width1,
        height: height1
        };

    println!("Area of {:#?}: {}", rect, area_rect(&rect));
}

fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tupled(dimensions: (u32, u32)) -> u32 {
    area_simple(dimensions.0, dimensions.1)
}

fn area_rect(rect: &Rectangle) -> u32 {
    // Does not compile, rectangle -/> tuple, even though isomorphic
    // area_tupled(rect)
    area_simple(rect.width, rect.height)
}