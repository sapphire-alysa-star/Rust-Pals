use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
struct WrappedError {
    error: ParseIntError,
    variable: u16,
}

// Display is a Trait. We are writing a custom implementation. 
impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write into the supplied output tream: `f`.
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed. 

        write!(f, "Issue with variable: {}. {}", self.variable, self.error)
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

fn print(result: &Result<i32, WrappedError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn print_debug(result: &Result<i32, WrappedError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(&twenty);
    // n is 20


    let tt = multiply("t", "2");
    print(&tt);
    print_debug(&tt);
    // Error: Issue with variable: 0. invalid digit found in string
    // Error: WrappedError { error: ParseIntError { kind: InvalidDigit }, variable: 0 }


    let tt = multiply("50", "100000000000000000000000");
    print(&tt);
    print_debug(&tt);
    // Error: Issue with variable: 1. number too large to fit in target type
    // Error: WrappedError { error: ParseIntError { kind: PosOverflow }, variable: 1 }
}