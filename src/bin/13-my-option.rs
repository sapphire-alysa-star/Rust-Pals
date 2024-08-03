// Self is the type of the current object. 
// It may appear either in a trait or an impl, 
// but appears most often in trait where it is a stand-in for whatever type will end up implementing the trait (which is unknown when defining the trait):

// impl Clone for MyType {
//     // I can use either the concrete type (known here)
//     fn clone(&self) -> MyType;

//     // Or I can use Self again, it's shorter after all!
//     fn clone(&self) -> Self;
// }

// if using self, the function introduced is a method
// if using any other name, the function introduced is an associated function

enum MyOption<T> {
    MySome {value: T},
    MyNone
}

impl<T> MyOption<T> {
    // method since first argument is &self
    fn unwrap(&self) -> &T {
        match &self {
            MyOption::MySome {value} => value,
            MyOption::MyNone => panic!("Attempted to unwrap MyNone!")
        }
    }

    fn is_some(&self) -> bool {
        match &self {
            MyOption::MySome {value: _v} => true,
            MyOption::MyNone => false
        }
    }

    // association function
    pub fn some(value: T) -> MyOption<T> {
        MyOption::MySome {
            value: value
        }
    }

    pub fn none() -> MyOption<T> {
        MyOption::MyNone
    }
}


fn main () {
    let my_some_1 = MyOption::some("Hello");
    let my_some_2 = MyOption::some("Celene");
    let my_none = MyOption::none();

    for i in [&my_some_1, &my_some_2, &my_none] {
        if i.is_some() {
            println!("{}", i.unwrap())
        } else {
            println!("Bad idea to unwrap a None")
        }
    }

}


