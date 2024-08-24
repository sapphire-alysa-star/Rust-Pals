// Rust does not have constructors. 

// The Rust Destructor equivalent is the Drop trait you can implement

#![allow(unused_variables)]

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn stack_frame() {
    let x = Droppable {
        name: "Baller Man"
    };
}

fn main() {

    stack_frame();

    stack_frame();

}