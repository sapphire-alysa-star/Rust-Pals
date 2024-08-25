use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::{thread, time};
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let files = ["foo.txt", "bar.txt", "baz.txt"];

    thread::scope(|scope| {
        for file in files {
            scope.spawn(move || {
                let contents = fs::read_to_string(file);
                // ...
            });
        }
    });

    // Receive messages from the channel
    for received in rx {
        println!("Got: {:?}", received);
    }
}

// Common Prejudice Against Threads

// https://vorner.github.io/async-bench.html 

// Asynchronous programming is often seen as the solution to improving performance and scalability of I/O-bound workloads.

// In a recent benchmark, traditional threading outperformed the async approach in scenarios with a limited number of threads. This underscores the core premise that, in real-world applications, the performance difference between the two approaches is often negligible, if not slightly favoring threads. Thus, it's crucial not to gravitate towards async Rust driven solely by anticipated performance gains.

// Thread-based frameworks, like the now-inactive iron, showcased the capability to effortlessly handle tens of thousands of requests per second. This is further complemented by the fact modern Linux systems can manage tens of thousands of threads.

// Traditional arguments against threads simply don't apply to Rust. Threaded code in Rust is protected from data races, null dereferences, and dangling references, ensuring a level of safety that prevents many common pitfalls found in other languages.

