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

}

// nothing here should be confusing. 