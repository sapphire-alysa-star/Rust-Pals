struct Val {
    val: f64,
}
// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// T is the parameter for a generic type. 
struct GenVal<T> {
    gen_val: T,
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    
    let three: i32 = 3;
    let y = GenVal { gen_val: three };

    println!("{}, {}", x.value(), y.value());
}