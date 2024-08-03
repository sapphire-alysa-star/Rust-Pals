struct Val {
    value: f64,
}
// impl of Val
impl Val {
    fn get_value(&self) -> &f64 {
        &self.value
    }
}

// T is the parameter for a generic type. 
struct GenVal<T> {
    value: T,
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let x = Val { value: 3.0 };
    
    let three: i32 = 3;
    let y = GenVal { value: three };

    println!("{}, {}", x.get_value(), y.get_value());
}