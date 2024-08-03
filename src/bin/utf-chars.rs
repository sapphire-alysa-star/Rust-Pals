use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // for i in [0, 1] {
    //     println!("{}", i);
    // }
    

    let s = "Sol";
    for b in s.bytes() {
        println!("{}", b);
    }
    // this prints:
    // 83
    // 111
    // 108

    let complex_string = "é"; // Note: 'e' with an acute accent
    for g in complex_string.graphemes(true) {
        println!("{}", g);
    }
    // prints: é

    let invalid_bytes = vec![82, 117, 115, 116, 255]; // The last byte is invalid UTF-8
    print_utf_chars(&invalid_bytes);

    let valid_bytes = vec![82, 117, 115, 116]; // The last byte is invalid UTF-8
    print_utf_chars(&valid_bytes);
    // prints:
    // Invalid UTF-8 sequence
    // R
    // u
    // s
    // t
}

fn print_utf_chars(bytes: &Vec<u8>) {
    match String::from_utf8(bytes.clone()) {
        Ok(s) => {
            for c in s.chars() {
                println!("{}", c);
            }
        },
        Err(_) => println!("Invalid UTF-8 sequence"),
    }
}

// targets:
// push
// pop
// [] operator - or at least get_at and set_at
// 


// string methods / Traits:
// Display
// push_str(&str)
// push(char)
// clear()
// ::from(&str)
// ::new()
