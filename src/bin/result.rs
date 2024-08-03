// panic! is not recoverable. 
// Probably people 'handle errors' too much
// But you definitely need to understand this stuff and return helpful error messages

// That is, Result<T, E> could have one of two outcomes:

// Ok(T): An element T was found
// Err(E): An error was found with element E

// Lets clean this up AND add a custom error class that says which input was bad.

use std::num::ParseIntError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct WrappedError {
    error: ParseIntError,
    variable: u16,
}
// impl Error for WrappedError {
//     fn description(&mut self) -> &str {
//         self.description = 
//     }
// }

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write into the supplied output tream: `f`.
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed. 

        write!(f, "Issue with variable: {}. {}", self.variable, self.error.description().to_string())
    }
}

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, WrappedError> {

    let x = match first_number_str.parse::<i32>() {
        Err(e) => return Err( WrappedError{
            error: e,
            variable: 0
        }),
        Ok(num) => num
    };

    let y = match second_number_str.parse::<i32>() {
        Err(e) => return Err( WrappedError{
            error: e,
            variable: 1
        } ),
        Ok(num) => num
    };

    Ok(x * y)
}

fn print(result: Result<i32, WrappedError>) {
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

    // The following now provides a much more helpful error message.
    let tt = multiply("50", "100000000000000000000000");
    print(tt);
}