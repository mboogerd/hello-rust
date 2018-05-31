fn main() {
    // This variable must be mutable
    let mut x = 5;
    println!("the value of x is {}", x);
    // or this line will fail to compile
    x = 6;
    println!("the value of x is {}", x);

    // This variable must be type anotated for `parse` to derive
    // what to extract from the string. Without, no compile!
    let guess: u32 = "42".parse().expect("Not a number!");

    
    /* Integer types: signed or unsigned; explicit size
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    arch	isize	usize
    ^^ 64-bits if you’re on a 64-bit architecture and 32-bits
    if you’re on a 32-bit architecture.

    Rust’s defaults are generally good choices, and integer
    types default to i32: it’s generally the fastest, even on
    64-bit systems
    */

    /* Integer literals
    Number literals	Example
    Decimal	        98_222
    Hex	            0xff
    Octal	        0o77
    Binary	        0b1111_0000
    Byte (u8 only)	b'A'
    */

    /* Floating point types
    Size    Type    Literal
    32-bit  f32     2.0
    64-bit  f64     2.0
    ^^ default
    */

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;

    /* Booleans */
    let t = true;
    let f: bool = false;

    /* Char types*/
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Unicode Scalar Values range from U+0000 to U+D7FF and
    // U+E000 to U+10FFFF inclusive.

    /* === Tuples === */

    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let tup3 = (500, 6.4, 1.0);
    tup = tup2;
    // this does not compile, as we would change `tup`s type
    // tup = tup3;

    let (x, y, z) = tup;
    println!("The value of z is: {}", z);

    // Tuples are accessible through their zero-indexed position
    let x = tup.0;

    /* === Arrays === */
    // Stack allocated, fixed size sequence
    let a = [1,2,3,4,5];

    // Arrays can be accessed through their zero-indexed position
    let x = a[0];
    println!("The value of a[0] is: {}", x);

    // Out of bounds selection is a runtime error
    let x = a[5];
}