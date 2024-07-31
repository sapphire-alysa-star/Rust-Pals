#[derive(Clone, Copy, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let black = Color {
        red: 0,
        green: 0,
        blue: 0
    };

    println!("{:?}", black);

    let mut my_color = black; // this requires the struct have Copy!

    my_color.green = 255;

    println!("{:?}", my_color); // this requires the Debug trait!

}