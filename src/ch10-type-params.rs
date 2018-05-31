/* Compute largest for i32 and char... */
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* A generic version of largest for Copyable stuff */
fn largest_copyable<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* A generic version of largest */
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

/* A generic datatype, both x and y need to be the same type */
struct Point<T> {
    x: T,
    y: T,
}

/* A generic implementation of Point, [point].x is defined for all Point<T> */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);


    println!("Computing largest generically");
    println!("[copyable] The largest number is {}",
        largest_copyable(&number_list));
    println!("[copyable] The largest char is {}",
        largest_copyable(&char_list));
    println!("[generic] The largest number is {}",
        largest(&number_list));
    println!("[generic] The largest char is {}",
        largest(&char_list));
}

/*

Rust performs monomorphization of code using generics at compile time.

Monomorphization is the process of turning generic code into
specific code with the concrete type that are actually used
filled in.

*/