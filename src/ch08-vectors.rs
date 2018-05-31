fn main() {
    // create a new, empty vector
    // include type annotation to establish concrete vector type
    let v: Vec<i32> = Vec::new();

    // vec! annotation to initialize a non-empty vector
    let v = vec![1, 2, 3];
    // v is now immutable though
    // v.push(4); (compile-error: cannot borrow mutably)

    {
        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
    }   // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];

    // Unsafe, but valid access
    let third: &i32 = &v[2];
    // Safe valid access
    let third: Option<&i32> = v.get(2);

    // Unsafe, illegal access would cause runtime IOB PANIC!!!
    // let does_not_exist = &v[100];
    // Safe illegal access
    let does_not_exist = v.get(100);
    println!("does not exist value: {:?}", does_not_exist);

    /* Invalid references */
    // In general we cannot have mutable and immutable
    // references in the same scope; same for vectors...
    let mut v = vec![1,2,3,4];
    // immutable reference in scope
    let first = &v[0];
    // mutable borrow would be attempted, does not compile
    // v.push(6);

    // ^^  adding a new element onto the end of the vector
    // might require allocating new memory and copying the
    // old elements to the new space if there isn’t enough
    // room to put all the elements next to each other where
    // the vector was. In that case, the reference to the
    // first element would be pointing to deallocated memory

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}