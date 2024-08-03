// T is the parameter for a generic type. 
struct GenVal<T> {
    value: T,
}
impl<T> GenVal<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let x = GenVal { value: 3.0 };
    
    let three = "three";
    let y = GenVal { value: three };

    println!("{}, {}", x.get_value(), y.get_value());
}