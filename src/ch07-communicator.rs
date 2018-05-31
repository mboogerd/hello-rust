extern crate communicator;

fn main() {
    //  The default state of all code in Rust is private:
    // no one else is allowed to use the code

    communicator::client::connect();
}