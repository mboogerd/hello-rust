use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // Single free variable closure.
    // Braces are optional for single-expression body
    let closure1 = |a| a ;
    // Without an invocation of the closure, not providing
    // type parameters to the definition is a syntax error...
    closure1(&"");
    // However, if we call it later with a different type
    // closure1(1); // <-- type mismatch (&&str, integral)

    // Conclusion: a closure always has a fully specified type signature, but the assignment of concrete types may be delayed

    // Closure with two free variables
    let _closure2 = |a: &str, b: &str| { (a, b) };


    // Closures can capture variables in scope, whereas functions can't
    let x = 4;
    let equal_to_x = |z| z == x;
    // VV compile error
    // fn equal_to_x(z: i32) -> bool { z == x }
    assert!(equal_to_x(4));

    /*
    When a closure captures a value from its environment, it uses memory to store the values for use in the closure body
    - FnOnce _consumes_ the variables it captures from its enclosing scope, known as the closure’s environment
    - Fn _immutably borrows_ values from the environment.
    - FnMut can change the environment because it _mutably borrows_ values.

    If we want to force the closure to take ownership of the values it uses in the environment, we can use the move keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.
    */

}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

struct Cacher2<F, A: Debug, B: Debug> where F: Fn(A) -> B {
    calculation: F,
    value: HashMap<A, B>
}

impl<F, A: Eq + Hash + Copy + Debug, B: Debug> Cacher2<F, A, B> where F: Fn(A) -> B {
    fn new(calculation: F) -> Cacher2<F, A, B> {
        Cacher2 {
            calculation,
            value: HashMap::new()
        }
    }

    fn value<'b>(&'b mut self, arg: A) -> &'b B {
        // Make an immutable reference to the function
        let f = &self.calculation;
        self.value
            .entry(arg)
            .or_insert_with(|| (f)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let mut expensive_result = Cacher::new(|intensity| simulated_expensive_calculation(intensity));

    let mut expensive_result = Cacher2::new(|intensity| simulated_expensive_calculation(intensity));

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}