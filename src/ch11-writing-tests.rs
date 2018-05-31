fn main() {

}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }

    #[test]
    fn it_works2() {
        assert_ne!(true, false);
    }

    #[test]
    #[ignore]
    fn it_fails() {
        assert_ne!(true, true);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    // expected tests for containment of substring in err msg.
    #[should_panic(expected = "Panic required in this test")]
    fn should_panic() {
        panic!("Panic required in this test");
    }

    /*
    Running tests:
    - Separate arguments for cargo and the test binary by `--`
      - cargo test --help (cargo's help)
      - cargo test -- --help (display possible options after --)
        [OPTIONS] [FILTER (test-name contains)]
        --nocapture (prints test run output for succesful tests too)

    - By default tests are parallellized
    */
}