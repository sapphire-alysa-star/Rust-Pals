// You can also restrict your functions or impl to types that has a certain trait.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;



fn display<T> (x: T) 
where
T: Display {
    println!("{}", x);
} 

struct FullName<'a, 'b> {
    first: &'a str,
    last: &'b str
}
impl<'a, 'b> Display for FullName<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write into the supplied output tream: `f`.
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed. 

        write!(f, "{} {}", self.first, self.last)
    }
}


fn main() {
    let name = String::from("Celene");

    display(name);

    let full = FullName {
        first: "Sol",
        last: "Lovelace"
    };

    display(full);
    // Celene
    // Sol Lovelace
}