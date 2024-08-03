fn main() {
    // given the ! this is a macro not a function.More about that later. 
    println!("Hello Friends"); // this is a string literal

    // immutable by default
    let x = String::from("Hello World");

    println!("{}", x);

    let length = x.len();
    println!("x has length: {}", length);

    // debug format printing. Honestly works better for print debugging
    println!("{:?}", x);

    let f0 = "April";
    let f1 = "Celene";
    let f2 = "Ichi";
    let f3 = "Isha";

    println!("Rust Pals: {}, {}, {}, {}", f0, f1, f2, f3);
}

// nothing here should be confusing. 