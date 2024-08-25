use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    // This variable declaration is where its value is specified.

    let data = vec![1, 2, 3];
    let second_data = vec![4, 5, 6];

    // move converts any variables captured by reference or mutable reference to variables captured by value.
    let closure = move || println!("captured {data:?} by value");
    closure();

    // data is no longer available, it is owned by the closure
    println!("{:?}", second_data); // second data however is not captured

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a
        // reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }

    // Make sure all Arc instances are printed from spawned threads.
    thread::sleep(Duration::from_secs(1));
}