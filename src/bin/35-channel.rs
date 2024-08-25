use std::sync::mpsc;
use std::thread;
use rand;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("--- A"),
            String::from("--- B"),
            String::from("--- C"),
            String::from("--- D"),
            String::from("--- E"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            let duration = rand::random::<u8>();
            thread::sleep(Duration::from_millis(duration as u64));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Never"),
            String::from("wanna"),
            String::from("give"),
            String::from("you"),
            String::from("up!")
        ];

        for val in vals {
            tx.send(val).unwrap();
            let duration = rand::random::<u8>();
            thread::sleep(Duration::from_millis(duration as u64));
        }
    });

    // this will not return until all the senders are dropped. So dont clone tx an extra time!
    for received in rx {
        println!("{received}");
    }
}

// --- A
// Never
// wanna
// --- B
// --- C
// give
// --- D
// --- E
// you
// up!

// Common Prejudice Against Threads

// https://vorner.github.io/async-bench.html 

// Asynchronous programming is often seen as the solution to improving performance and scalability of I/O-bound workloads.

// In a recent benchmark, traditional threading outperformed the async approach in scenarios with a limited number of threads. This underscores the core premise that, in real-world applications, the performance difference between the two approaches is often negligible, if not slightly favoring threads. Thus, it's crucial not to gravitate towards async Rust driven solely by anticipated performance gains.

// Thread-based frameworks, like the now-inactive iron, showcased the capability to effortlessly handle tens of thousands of requests per second. This is further complemented by the fact modern Linux systems can manage tens of thousands of threads.

// Traditional arguments against threads simply don't apply to Rust. Threaded code in Rust is protected from data races, null dereferences, and dangling references, ensuring a level of safety that prevents many common pitfalls found in other languages.

