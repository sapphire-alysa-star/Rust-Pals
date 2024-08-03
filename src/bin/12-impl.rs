struct Pair<'a> {
    name: &'a str,
    value: f64,
}

// impl of Val
impl<'a> Pair<'a> {
    fn get_value(&self) -> &f64 {
        &self.value
    }

    // fn set_value(&self) -> Pair {
    //     &self.value = value
    // }

    fn get_name(&self) -> &str {
        &self.name
    }

    // fn set_name(&self, new_name: &str) -> Pair {
    //     let copied_name = String::from(new_name);
    //     &self.name = &copied_name[0..];
    // }
}

fn main() {

    let mut p1 = Pair {
        name: "Celene",
        value: -100.0
    };

    println!("Name: {}. Value: {}.", p1.get_name(), p1.get_value());
}