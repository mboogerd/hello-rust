use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("map: {:?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Rust can collect into many collections, but its parameters are known
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("map: {:?}", scores);

    // We can loop over the key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Overwriting a value */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Overwriting Blue {:?}", scores);

    /* Insert if not exists */
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Insert Yellow/Blue with 50 if not exists {:?}", scores);

    /* Updating a value */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Fetch the values' mutable reference for the word
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}