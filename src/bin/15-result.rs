// That is, Result<T, E> could have one of two outcomes:

// Ok(T): An element T was found
// Err(E): An error was found with element E

// Lets clean this up AND add a custom error class that says which input was bad.

use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {

    let x = match first_number_str.parse::<i32>() {
        Err(e) => return Err(e),
        Ok(num) => num
    };

    let y = match second_number_str.parse::<i32>() {
        Err(e) => return Err(e),
        Ok(num) => num
    };

    Ok(x * y)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);
}