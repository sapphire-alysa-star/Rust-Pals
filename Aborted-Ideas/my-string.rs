use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // let s = "Hello, Rust!";
    // for c in s.chars() {
    //     println!("{}", c);
    // }
​
    let complex_string = "é"; // Note: 'e' with an acute accent
    for g in complex_string.graphemes(true) {
        println!("{}", g);
    }
}