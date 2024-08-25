use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn read_file() {
    // Create a path to the desired file
    let path = Path::new("./src/data/wish.txt"); // I am running this from rust pals root!
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}\n", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn write_file() {
    let path = Path::new("./src/data/lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn concat_lines(filename: &str) -> String {
    let mut result = String::new();

    for line in read_to_string(filename).unwrap().lines() {
        if result.len() > 0 {
            result.push(' ');
        }

        result.push_str(line);
    }

    result
}

fn main() {
    read_file();

    write_file();

    let lyrics_1 = concat_lines("./src/data/wish.txt");
    println!("{}", lyrics_1);
}