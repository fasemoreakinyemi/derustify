//Scalars

//intergers
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
 
//numeric
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

//floating points
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}


//character
// character literal with single quotes
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
I
//String
// String literal wit double quotes
// special and complex in rust
    let data = "initial contents";

    let s = data.to_string();
    let s = String::from("initial contents");
    s.push_str(", world!"); // push_str() appends a literal to a String


// Dictionary

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
