#![allow(dead_code)]

fn print_length_three_times(s: &String) {
    println!("First: {:?}", (*s).len());
    println!("Second: {:?}", (&s).len());
    println!("Third: {:?}", (&&&&&&&&s).len());

    // stuff that does not work:
    // println!("Fails: {:?}", (**s).len()); // error -  `str` is a primitive type and therefore doesn't have fields
    // println!("Fails: {:?}",  *s.len()); // you cannot dereference a usize. usize is the size of a string. unbounded!
}

fn main() {
    // immutable by default
    let x = String::from("Hello World");

    print_length_three_times(&x);
}

// Vocab and notes:
// &variable - reference to the variable. Kind of like a C pointer.
// *reference - dereferences the variable. 
// Rust has automatic referencing and dereferencing. So in many places any of var, &var, *var all work 