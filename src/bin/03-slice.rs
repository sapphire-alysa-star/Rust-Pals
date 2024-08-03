fn main() {
    let s = String::from("Hello World");

    let slice: &str = &s[0..]; // type is &str. You can omit the explicit type. Compiler can infer its &str.

    println!("{:?}", slice);

    let hello = &s[0..5]; // compiler can infer type
    println!("{:?}", hello);
}


// A String is basically a wrapped pointer to a str. It stores the location of the string and its length.
// It also has some methods like length. Things like this are called 'smart pointers'.
// Smart pointers important in Solana!