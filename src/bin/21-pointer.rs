// We will follow cpp 01-06 in 21-26. As best we can. This will use unsafe rust. 

fn main () {
    let x = 5;

    let raw_pointer = &x as *const i32; // casts a reference as a raw pointer.

    println!("variable x: {}", x);

    println!("address of x: {:?}", raw_pointer); // no Display

    // You can actually hardcode memory addresses and insist content is an i32 if you want.
    let address = 0x012345usize;
    let hardcoded = address as *const i32;

    println!("address of hardcoded pointer: {:?}", hardcoded); 

    // variable x: 5
    // address of x: 0x7ffe39edfd94
    // address of hardcoded pointer: 0x12345


    // You need to be in an unsafe block to deference a raw pointer.
    unsafe {
        println!("Checking we get the same x value with deference. x = {}", *raw_pointer);
        // println!("This line will error if you run it. hardcocded = {}", *hardcoded);

        // Checking we get the same x value with deference. x = 5
        // Segmentation fault
    }
}

// C++ Version

// int main() {
//     int x; 
//     int *p; // note its genuinely int *p not int* p;

//     x = 5;
//     p = &x; // p points to x

//     cout << "variable x: " << x << endl;

//     printf("address of x: %p\n", p);  

//     return 0;
// }