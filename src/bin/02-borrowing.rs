fn print_string(s: String) {
    // dont do anything
    println!("{:?}", s);
}

fn main() {
    let x = String::from("Isha");
    print_string(x); // x is now uhhh 'dead'. YES. We effectively called move.

    // print_string(x); // this line will kill us!

    // RULES: Not really about memory. About Concurrency.
    // -- Only one mutable reference to a variable.
    // -- Can't have BOTH a mutable AND an immutable reference.

    // let z = x; // this will also kill us!. z = move(x);

    
    let y = String::from("Celene");
    print_string(y.clone());
    print_string(y.clone());


    let z = String::from("April");
    let ref_z = &z;
    let april_plus_one = plus_one(ref_z);
    let april_plus_two = plus_one(&april_plus_one);

    println!("{:?}", april_plus_two);

    // rules:
    // At most 1 mutable reference
    // As many immutable references as desired
    // No mixing! If you have a mutable reference you cannot have any immutable ones
}

fn plus_one(s: &String) -> String {
    format!("{} +1", s)
}