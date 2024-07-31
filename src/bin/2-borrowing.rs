fn print_string(s: String) {
    // dont do anything
    println!("{:?}", s);
}

fn main() {
    let x = String::from("Isha");
    print_string(x); // x is now uhhh 'dead'
    // print_string(x); // this line will kill us!
    // let z = x; // this will also kill us!

    
    let y = String::from("Celene");
    print_string(y.clone());
    print_string(y.clone());


    let z = String::from("April");
    let ref_z = &z;
    let april_plus_one = plus_one(ref_z);
    let april_plus_two = plus_one(&april_plus_one);

    println!("{:?}", april_plus_two);
}

fn plus_one(s: &String) -> String {
    format!("{} +1", s)
}