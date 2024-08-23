fn dangle (val: i32) -> *const i32 {
    let x: i32 = val;

    let p: *const i32 = &x;

    println!("address of x: {:?}", p);

    p
}

fn main() {
    let correct_val = 7;
    let dangling_pointer: *const i32 = dangle(correct_val);

    dangle(42);
    
    println!("address of dangling pointer: {:?}", dangling_pointer);

    unsafe {
        println!("The value should be {}, but of course its {:?}", correct_val, *dangling_pointer)
    }

    // The value should be 7, but of course its 42
    // This kinda makes sense since same location is used. And still has an integer in there!


    // Amaingly if I remove the first two println! I get different behavior!
    // The value should be 7, but of course its -538107904
}

// C++ version:

// int* dangle() {
//     int x = 5;
//     int* p;

//     p = &x;

//     cout << "variable x has address: " << p << endl;

//     return p;
// }


// int main() {

//     int* p = dangle();

//     cout << "dangling pointer address: " << p << endl;
//     cout << "dereference p to get: " << *p << endl;

//     // dereference p to get: 32619 . -- undefined but does not error on my machine in cpp land.

//     return 0;
// }