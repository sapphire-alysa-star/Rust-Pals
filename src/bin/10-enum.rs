// Enums store data of mixed type.
// They can store an enum, a struct, or nothing

#[derive(Debug, Clone, Copy)]
enum MixedData<'a> {
    Name {first: &'a str, last: &'a str},
    Number(u8), // this is a tuple. so access with .0
    Pair(u8, u8),
    Unit // if a variant has no data its called unit type.
}

fn main() {
    let num = MixedData::Number(42);
    let pair = MixedData::Pair(0, 1);
    let sapph = MixedData::Name { first: "Sapphire", last: "Star" };
    let unit_type = MixedData::Unit;

    for item in [&num, &pair, &sapph, &unit_type] { // this is an array
        println!("Debug Works fine: {:?}: ", item);
    }
    // Prints:
    // Debug Works fine: Number(42): 
    // Debug Works fine: Pair(0, 1):
    // Debug Works fine: Name { first: "Sapphire", last: "Star" }:
    // Debug Works fine: Unit:


    // lets format this ourselves with match:
    for item in [&num, &pair, &sapph, &unit_type] { // [&MixedData, 4];
        match *item {
            MixedData::Name {first, last} => println!("First name: {first}, Last: {last}"),
            MixedData::Number(num) => println!("Number Variant: {num}"),
            MixedData::Pair(x, y) => println!("Pair Variant: x - {x}, y - {y}"),
            MixedData::Unit => println!("Unit Type with no data to print.")
        }
    }
    // Prints:
    // Number Variant: 42
    // Pair Variant: x - 0, y - 1
    // First name: Sapphire, Last: Star
    // Unit Type with no data to print.
    
}