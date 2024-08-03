// Tuples are pretty easy tbh.
// mixed data allowed. Access with .0, .1, .., .n

fn main () {
    let pair: (char, i32) = ('a', 17);
    println!("{}", pair.0); // a
    println!("{}", pair.1); // 17

    let (some_char, some_int) = ('b', 42);
    println!("{}", some_char); // b
    println!("{}", some_int); // 42
}