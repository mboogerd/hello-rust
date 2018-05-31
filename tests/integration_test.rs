// We would normally have to import the crate under test
// In this project we often switchj
// extern crate adder;

#[test]
fn it_adds_two() {
    assert!(true, true);
}

/*
    Common test fixtures can go into a mod.rs file, which
    does not itself count as a module from the perspective
    of a test.
*/