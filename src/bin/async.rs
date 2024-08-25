// fn get_two_sites() {
//     // Spawn two threads to do work.
//     let thread_one = thread::spawn(|| download("https://www.foo.com"));
//     let thread_two = thread::spawn(|| download("https://www.bar.com"));

//     // Wait for both threads to complete.
//     thread_one.join().expect("thread one panicked");
//     thread_two.join().expect("thread two panicked");
// }

// async fn get_two_sites_async() {
//     // Create two different "futures" which, when run to completion,
//     // will asynchronously download the webpages.
//     let future_one = download_async("https://www.foo.com");
//     let future_two = download_async("https://www.bar.com");

//     // Run both futures to completion at the same time.
//     join!(future_one, future_two);
// }

use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::{thread, time};

fn main() {
    thread::scope(|scope| {
        // worker thread 1
        scope.spawn(|| {
            let contents = fs::read_to_string("foo.txt");
            // do something with contents
        });

        // worker thread 2
        scope.spawn(|| {
            let contents = fs::read_to_string("bar.txt");
            // ...
        });

        // worker thread 3
        scope.spawn(|| {
            let contents = fs::read_to_string("baz.txt");
            // ...
        });
    });
    
    // No join; threads get joined
    // automatically once the scope ends
}