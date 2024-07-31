#![allow(unused_variables)]

// this straightforward seeming function does not work.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// } 

// 'a and 'a means the minimum lifetime
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

fn first_string<'a> (x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    let short = "";
    let long = "long???";

    let longer_string = longest(short, long);
    println!("{}", longer_string);

    let first = first_string(long, short);
    println!("{}", first);
}