#[derive(Clone, Debug)]
struct Pair<'a, 'b> {
    a: &'a str,
    b: &'b str,
}

// struct Pair<'a> {
//     a: &'a str,
//     b: &'a str,
// } // this version fails

fn main() {
    let hello = String::from("hello");
    let world = String::from("world");

    let my_pair = Pair { a: &hello[0..], b: &world[0..] }; // &hello also works.
    println!("{}", my_pair.a);

    let b = my_pair.b;

    drop(my_pair);
    drop(hello);

    println!("{}", b);
}

// you cannot derive Copy here. Doesnt work with Drop.