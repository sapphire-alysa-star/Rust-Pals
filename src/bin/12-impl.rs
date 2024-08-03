struct Pair<'a> {
    name: &'a str,
    value: f64,
}

// impl of Val
impl<'a> Pair<'a> {
    fn get_value(&self) -> &f64 {
        &self.value
    }

    fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, new_name: &'a str) {
        self.name = new_name;
    }
}

fn main() {

    let mut p1 = Pair {
        name: "Celene",
        value: -100.0
    };

    println!("Name: {}. Value: {}.", p1.get_name(), p1.get_value());

    let new_name = String::from("April"); // 
    p1.set_name(&new_name);
    p1.set_value(512.0);

    println!("Name: {}. Value: {}.", p1.get_name(), p1.get_value());

    // Name: Celene. Value: -100.
    // Name: April. Value: 512.
}