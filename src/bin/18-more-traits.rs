use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Self means the current object.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Same thing: Note that the T is gone inside the function.
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Pair<T> {
//         Pair { x, y }
//     }
// }

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let nums = Pair::new(10, 20);

    Pair::cmp_display(&nums);
}