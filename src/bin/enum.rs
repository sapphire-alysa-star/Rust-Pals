#[derive(Debug, Clone, Copy)]
enum MixedData<'a> {
    Name {first: &'a str, last: &'a str},
    Number(u8), // this is a tuple. so access with .0
    Pair(u8, u8)
}

fn main() {
    let num = MixedData::Number(42);
    let pair = MixedData::Pair(0, 1);
    let sapph = MixedData::Name { first: "Sapphire", last: "Star" };

    for item in [&num, &pair, &sapph] {
        println!("Debug Works fine: {:?}: ", item);
    }

    // lets format this ourselves with match:

    for item in [&num, &pair, &sapph] {
        match *item {
            MixedData::Name {first, last} => println!("First name: {first}, Last: {last}"),
            MixedData::Number(num) => println!("Number Variant: {num}"),
            MixedData::Pair(x, y) => println!("Pair Variant: x - {x}, y - {y}")
        }
    }
    
}