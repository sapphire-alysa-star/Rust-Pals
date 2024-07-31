#![allow(unused_variables)]
#![allow(unused_mut)]

// some quick notes:
// rust has numerous number types

fn main() {
    let mut a: u8 = 255; // 0 to 255. Important type in Solana!

    // Overflows throw errors unless you run in release. Locally I can do: cargo run --bin 1-numbers --release
    // Solana code has the release mode behavior!
    
    // a = a + 1; // 
    // println!("Overflow to zero: {:?}", a);

    let mut b: u64 = 10 * 10 * 10 * 10 * 10;

    // debug printing. I basically recommend its use. I honestly dont remember exactly what fails if you use normie string interpolation.
    println!("b: {:?}", b);

    b = b * 2;
    println!("b: {:?}", b);

    let c: i32 = -130; // signed integer. (approx -2billion to 2billion)

    let d: f64 = 10.0; // you use the . to make something a float.
    let e = d / 20.; // you can also ommit the decimal. Note d / 5 fails!

    println!("b: {:?}", e);

    let mut hello = String::from("Hello");
    hello.push(' ');
    hello.push('W');
    hello.push_str("orld!");

    let length = hello.len(); 
    // length has type usize. usize is of unbounded size!

    println!("Hello World has length: {:?}", length);
}