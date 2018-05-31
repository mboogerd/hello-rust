enum IpAddrKind {
    V4,
    V6
}

// candidate IP representation
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// But better use ADT! \o/
#[derive(Debug)]
enum IpAddrString {
    // attach data to each variant of the enum directly
    V4(String),
    V6(String),
}

// Different data can be carried by each enum literal \o/
#[derive(Debug)]
enum IpAddr {
    // Uses tuple struct syntax, but struct syntax is allowed as well!
    V4(u8, u8, u8, u8),
    V6(String),
}

// We can also have implementations for enum types
impl IpAddr {
    fn ping(&self) {
        println!("Pinging {:?}", self);
    }
}

fn route_kind(_ip_type: IpAddrKind) {
    // can't yet do much here
}

fn route(ip: IpAddrString) {
    println!("Message to: {:?}", ip);
}

fn main () {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route_kind(four);
    route_kind(six);

    route(IpAddrString::V4(String::from("127.0.0.1")));
    route(IpAddrString::V6(String::from("::1")));

    IpAddr::V4(127, 0, 0, 1).ping();
    IpAddr::V6(String::from("::1")).ping();

    // Options are available
    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use None rather than Some, we need to tell Rust
    // what type of Option<T> we have, because the compiler
    // can’t infer the type
    let absent_number: Option<i32> = None;
}