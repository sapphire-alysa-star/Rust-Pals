// If you just need an error message, you can make use of From<&str> being implemented for Box<dyn Error>, and write:
// return Err(Box::from("your message here"));

// Err(Box::from("Not even!")) has type Box<dyn std::error::Error>
// I will use a declarion to shorten std::error::Error to Error
// A box puts something 'on the heap' so its around without an owner
// dyn means 'dynamic dispatch'. It annotates a trait. So this is a box of a 'thing that has the error trait'
// More on this stuff later.

use std::error::Error;

fn halves_if_even(i: i32) -> Result<i32, Box<dyn Error>> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err(Box::from("Not even!"))
    }
}

// we are just passing up the result for example of how this can work.
fn do_the_thing(i: i32) -> Result<i32, Box<dyn Error>> {
    match halves_if_even(i) {
        Ok(i) => Ok(i),
        Err(e) => return Err(e),
    }
}

fn main () -> Result<(), Box<dyn Error>> {
    // do_the_thing(5)?;
    // Error: "Not even!"

    do_the_thing(4)?;

    Ok(())
}

// fn do_the_thing(i: i32) -> Result<i32, Error> {
//     let i = halves_if_even(i)?;

//     // use `i`
// }