#[derive(Debug)]
// User struct definition. It has a set of key-value pairs
// with an associated name. This is a block definition, no ;
struct User {
    // Use String rather than &str, s.t. the struct owns all its data
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs have a sequence of values without keys/names
// but also carry a name. This is a statement, so ; terminated
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // User struct instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // In order to be able to mutate an attribute of User
    // the whole instance needs to be mutable
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("me@example.com"),
        String::from("me")
        );
    println!("User: {:?}", user2);

    let user2 = login_user(user2);
    println!("User: {:?}", user2);

    /* Tuple struct usage */
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}

fn build_user(email: String, username: String) -> User {
    // Note that we can leave out the attribute names when
    // the variable names whose values are assigned match
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn login_user(user: User) -> User {
    // A new User instance can be set with a subset of the fields
    // and have its remaining fields copied from another instance
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user
    }
}