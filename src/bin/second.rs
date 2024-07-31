#![allow(dead_code)]

fn print_length_wrong(s: String) {
    println!("The strings length is {:?}", s.len());
}

fn print_length_right(s: &String) {
    println!("The strings length is {:?}", s.len());
}

fn main() {
    // immutable by default
    let x = String::from("Hello World");

    // print_length_wrong(x);
    // print_length_wrong(x); // And disaster strikes! 

    // this method works fine. &x is a reference to x.
    // references are not exactly like C pointers but similar concept. More later on thinking about them.
    print_length_right(&x);
    print_length_right(&x);


    print_length_three_times(&x);

}

fn print_length_three_times(s: &String) {
    println!("First: {:?}", (*s).len());
    println!("Second: {:?}", (&s).len());
    println!("Third: {:?}", (&&&&&&&&s).len());

    // stuff that does not work:
    // println!("Fails: {:?}", (**s).len()); // error -  `str` is a primitive type and therefore doesn't have fields
    // println!("Fails: {:?}",  *s.len()); // you cannot dereference a usize. usize is the size of a string. unbounded!
}

// Vocab and notes:
// &variable - reference to the variable. Kind of like a C pointer.
// *reference - dereferences the variable. 
// Rust has automatic referencing and dereferencing. So in many places any of var, &var, *var all work 

// A String is basically a wrapped pointer to a str. It stores the location of the string and its length.
// It also has some methods like length. Things like this are called 'smart pointers'.