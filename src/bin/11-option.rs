// Option enum has 2 variants.
// 1- None is used to indicate failure or no value
// 2- Some which is tuple-struct that wraps the value

// Options let you avoid runtime errors like v[out_of_bounds]

// Note Im gonna use some arrays. 


fn main() {
    // vec.get instead of v[]

    let v = vec![0, 1, 2, 3, 4, 5];
    let _res1 = v.get(0); // 0
    let _res2 = v.get(6); // None

    // Method 1: match
    println!("Method 1: Match");
    let indices = [0, 1, 6]; // NOTE: This is an array. Works how you would guess.
    
    for i in &indices {
        match v.get(*i) { // you need the dereference operator *
            Some(res) => {
                println!("At index {i} we get: {res}");
            }, 
            None => {
                println!("Index {i} is out of bounds");
            }
        }
    }

    // Method 2: Explicitly calling .unwrap() on the option. This comes up in Solana. 
    // Check using is_some or is_none.
    println!("Method 2: .unwrap()");
    let indices = [2, 3, 7];
    for i in &indices {
        let res = v.get(*i);
        if res.is_some() {
            println!("At index {i} we get: {}", res.unwrap());
        } else {
            println!("Index {i} is out of bounds");
        }
    }

    // Method 3: If Let syntactic sugar
    // Common sugar. I dont find the convention the easiest to remmber but its super common.
    println!("Method 3: If Let");
    let indices = [4, 5, 8];

    for i in &indices {
        if let Some(res) = v.get(*i) {
            println!("At index {i} we get: {}", res);
        } else {
            println!("Index {i} is out of bounds");
        }
    }

    let mut some_num = Some(7);
    some_num = plus_one(some_num);
    some_num = plus_one(some_num); // Homework Problem XXX refers to this line. 

    println!("Some num is now: {}", some_num.unwrap()); // 
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
