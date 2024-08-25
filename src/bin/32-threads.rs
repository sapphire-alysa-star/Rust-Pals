use std::thread;
use std::time::Duration;
use rand;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..11 {
            let duration = rand::random::<u8>();
            thread::sleep(Duration::from_millis(duration as u64));

            println!("hi number {i} from the spawned thread!");
        }
    });

    for i in 1..6 {
        let duration = rand::random::<u8>();
        thread::sleep(Duration::from_millis(2*duration as u64));

        println!("--- main {i} ---");
    }

    handle.join().unwrap();
}

// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// --- main 1 ---
// hi number 4 from the spawned thread!
// --- main 2 ---
// hi number 5 from the spawned thread!
// --- main 3 ---
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// --- main 4 ---
// --- main 5 ---
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 10 from the spawned thread!