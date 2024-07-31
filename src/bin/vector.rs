// Important! Other tutorials and explanations will talk about 'borrowing' and the 'borrow checkers'. As will errors.
// The borrow checker is the thing that prevents memory errors from occuring at runtime. It checks at compile time.

// Lets learn about vectors and do some problems!

// If I comment right after or next to a print I am posted the output!

fn main() {
    let one_two_three: Vec<i32> = vec![1, 2, 3, 4];
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

    // Its important to iterate over a vector reference. Dont 'take ownership' of the elements of x!
    // You can also take slices of vectors. Similar to strings.

    for i in &v1[0..] {  // &v1 works in place of &v[0..]. 
        println!("{}", *i);
    }
    // 100
    // 32
    // 57

    // iterating over a reference and modifying elements!
    for i in &mut v1 {
        *i += 50;
    }
    for i in &v1 {
        println!("{i}"); // as usual people omit the *i a lot.
    }
    // 150
    // 82
    // 107

    // obviously you can do some computations

    let mut x = vec![1, 2, 3, 4, 0];
    x[4] = 5; // set value directly

    let mut sum = 0;
    for i in &x {
        sum += *i;
    }
    println!("The sum of elements is: {}", sum); // The sum of elements is: 15

    // lets do this with a function
    println!("The sum of elements is still: {}", vector_sum(&x));


    // Rust obviously has If statements
    if 5 > 0 {
        println!("5 is posiitive");
    } else {
        println!("wtf is going on");
    }

}

fn vector_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in &v[0..] {
        sum += *i;
    }

    sum
}

// Problem Set: Assume all vectors

// 