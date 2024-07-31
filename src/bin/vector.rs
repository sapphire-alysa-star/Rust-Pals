// Lets learn about vectors and do some problems!

// If I comment right after or next to a print I am posted the output!

fn main() {
    let one_two_three = vec![1, 2, 3, 4];
    println!("{:?}", one_two_three); // [1, 2, 3, 4]

    let third: &i32 = &one_two_three[2];
    println!("The third element is {third}"); // 3

    // Accessing out of bound element throws a runtime error!
    // let out_of_bound: &i32 = &one_two_three[10];

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v); // [5, 6, 7, 8]


    // iterating over vectors
    let mut v1 = vec![100, 32, 57];

    for i in &v1 {
        println!("{i}");
    }
    // 100
    // 32
    // 57

    // iterating over a reference and modifying elements!
    for i in &mut v1 {
        *i += 50;
    }
    for i in &v1 {
        println!("{i}");
    }
    // 150
    // 82
    // 107

    // obviously you can do some computations

    let mut x = vec![1, 2, 3, 4, 0];
    x[4] = 5; // set value directly

    let mut sum = 0;
    for i in x {
        sum += i;
    }
    println!("The sum of elements is: {}", sum); // The sum of elements is: 15


}