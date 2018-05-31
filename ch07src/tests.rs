use super::client;

#[test]
fn it_adds() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_works() {
    // start from the root
    ::client::connect();
    // or from parent (in this case equivalent)
    super::client::connect();
    // or just from the imported super::client above
    client::connect();
}
