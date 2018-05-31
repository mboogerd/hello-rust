use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>,

        // RefCell<T> keeps track of how many Ref<T> and RefMut<T>
        // smart pointers are currently active
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // We can’t modify the MockMessenger to keep track of the
            // messages because the send method takes an immutable
            // reference to self
            //self.sent_messages.push(String::from(message));

            // borrow_mut returns the smart pointer type RefMut<T>
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        //assert_eq!(mock_messenger.sent_messages.len(), 1);

        // borrow method returns the smart pointer type Ref<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    #[should_panic]
    fn refcell_allows_only_one_mutable_borrow() {
        let rc: RefCell<Vec<String>> = RefCell::new(vec![]);
        let mut _one_borrow = rc.borrow_mut();
        // 'already borrowed: BorrowMutError'
        let mut _two_borrow = rc.borrow_mut();
    }

    #[test]
    #[should_panic]
    fn refcell_disallows_cross_mutable_borrow() {
        let rc: RefCell<Vec<String>> = RefCell::new(vec![]);
        let mut _one_borrow = rc.borrow_mut();
        let _two_borrow = rc.borrow();
    }

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use self::List::{Cons, Nil};

    #[test]
    fn it_should_allow_shared_mutable_data() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}