
fn fails() {
    let maybe_name = Some(String::from("Alice"));
    // The variable 'maybe_name' is consumed here ...
    match maybe_name {
        Some(n) => println!("Hello, {n}"),
        _ => println!("Hello, world"),
    }
    // ... and is now unavailable.
    println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
}

fn works () {
    let maybe_name = Some(String::from("Alice"));
    // Using `ref`, the value is borrowed, not moved ...
    match maybe_name {
        Some(ref n) => println!("Hello, {n}"),
        _ => println!("Hello, world"),
    }
    // ... so it's available here!
    println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
}

fn main() {
    // fails();
    works();
}