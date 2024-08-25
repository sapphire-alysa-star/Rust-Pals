use std::sync::{Arc, Mutex};
use std::thread;
use rand;
use std::time::Duration;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
fn main() {
    let arc_data: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let arc_data_1 = arc_data.clone();
    let arc_data_2 = arc_data.clone();

    let feed1 = thread::spawn(move || {
        for i in 1..6 {
            let duration = rand::random::<u8>();
            thread::sleep(Duration::from_millis(duration as u64));

            let mut tx_guard = arc_data_1.lock().unwrap();

            let message = format!("1-{}", i);
            tx_guard.push(message);
        }
    });

    let feed2 = thread::spawn(move || {
        for i in 1..6 {
            let duration = rand::random::<u8>();
            thread::sleep(Duration::from_millis(duration as u64));

            let mut tx_guard = arc_data_2.lock().unwrap();

            let message = format!("2-{}", i);
            tx_guard.push(message);
        }
    });

    feed1.join().unwrap();
    feed2.join().unwrap();

    let log = arc_data.lock().unwrap();

    println!("{:?}", log);

    // A few runs:

    // ["1-1", "2-1", "2-2", "1-2", "1-3", "1-4", "2-3", "1-5", "2-4", "2-5"]

    // ["2-1", "2-2", "1-1", "2-3", "1-2", "1-3", "2-4", "1-4", "2-5", "1-5"]

    // ["2-1", "1-1", "1-2", "2-2", "1-3", "2-3", "1-4", "1-5", "2-4", "2-5"]
}

// https://gist.github.com/autodidaddict/b7e5a2d051b8430a5cd88fc302b84eb7