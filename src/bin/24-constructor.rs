// Rust does not have constructors. 

// The Rust Destructor equivalent is the Drop trait you can implement

#![allow(unused_variables)]
#![allow(dead_code)]

pub struct Clock {
    name: &'static str,
    seconds: u8,
    hours: u8,
    minutes: u8,
}

impl Clock {
    fn new(name: &'static str, seconds: u8, hours: u8, minutes: u8) -> Self {
        Self {
            name,
            seconds,
            hours,
            minutes
        }
    }
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Clock {
    fn drop(&mut self) {
        println!("Dropping da clock: {}", self.name);
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self { 
            name: "",
            seconds: 0,
            hours: 0,
            minutes: 0,
        }
    }
}

fn stack_frame() {
    let x = Clock {
        name: "Baller Man",
        ..Default::default()
    };
}

fn another_frame() {
    let x = Clock::new("Tik Tok", 0, 0, 0);
}

fn main() {

    stack_frame();
    another_frame();

    // Dropping da clock: 
    // Dropping da clock: Baller Man
    // Dropping da clock: Tik Tok

    // Note: Default::default() actually creates and then discards a new instance.

    // To my knowlege there is no way to FORCE users to create a struct through new.
    // So gotta be tricky with pointer defaults if doing low level. 
    // However you do have a destructor hook to use to free memory

}