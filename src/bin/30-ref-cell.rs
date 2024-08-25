use std::cell::Cell;
use std::cell::RefCell; // A RefCell is a single-threaded RwLock (not mutex). 
use std::rc::Rc; // you can use methods of a value directly!

// RefCell methods:
// .borrow() and .borrow_mut() - mutiple borrow muts banned but checked at compile
// .replace

#[derive(Debug)]
struct Counter {
    val: u32,
}

struct Incrementor {
    pointer: *mut Counter,
    cell: Rc<RefCell<Counter>> // you cannot make these mutable normie refs.
}

impl Incrementor {
    fn increment_via_pointer(&mut self) {
        unsafe {
            (*self.pointer).val += 1;
        }
    }

    fn increment_via_rc(&mut self) {
        self.cell.borrow_mut().val += 1;
    }

    fn print_via_rc(&self) {
        println!("Count via safe rust: {:?}", self.cell.borrow().val);
    }

    fn print_via_pointer(&self) {
        unsafe {
            println!("Count via pointer: {}", (*self.pointer).val);
        }
    }
}


fn main() {
    let mut counter = Counter {
        val: 0
    };
    let p1 = &mut counter as *mut Counter;
    let ref_cell = RefCell::new(counter);


    let p2 = ref_cell.as_ptr();
    let rc = Rc::new(ref_cell);

    let p3 = rc.as_ptr(); // remember this is calling .as_ptr() on the underlying RC.

    println!("p1: {:?}\np2: {:?}\np3: {:?}", p1, p2, p3);

    // p1: 0x7ffc8771733c
    // p2: 0x7ffc87717350
    // p3: 0x555ee4287af8
    // both news copy the underlying data in this case. 
    // make sure to grab the right pointer if you ever do unsafe rust for real!

    let mut i1 = Incrementor {
        pointer: p3,
        cell: rc.clone()
    };

    let mut i2 = Incrementor {
        pointer: p3,
        cell: rc.clone()
    };

    let reference_count = Rc::strong_count(&rc);
    println!("Ref Count: {}", reference_count);
    // Ref Count: 3

    i1.increment_via_pointer();
    i1.increment_via_rc();
    i1.print_via_rc();
    i1.print_via_pointer();

    i2.increment_via_pointer();
    i2.increment_via_rc();
    i1.print_via_rc();
    i2.print_via_pointer();

    // Count via safe rust: 2
    // Count via pointer: 2
    // Count via safe rust: 4
    // Count via pointer: 4

}

// Another example program:

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;

// pub struct ArrayMap<T> {
//     map: HashMap<String, Rc<RefCell<Vec<T>>>>,
// }

// impl<T> ArrayMap<T> {
//     pub fn new() -> Self {
//         ArrayMap { map: HashMap::new() }
//     }

//     pub fn add(&mut self, key: &str, data: Vec<T>) {
//         self.map.insert(key.to_string(), Rc::new(RefCell::new(data)));
//     }

//     pub fn get(&mut self, key: &str) -> Option<Rc<RefCell<Vec<T>>>> {
//         self.map.get(key).cloned()
//     }
// }

// fn main() {
//     let mut am = ArrayMap::new();

//     am.add("a", vec![1, 2, 3]);
//     am.add("b", vec![4, 5, 6]);

//     let a1 = am.get("a").unwrap();
//     let a2 = am.get("a").unwrap();

//     let b1 = am.get("b").unwrap();
//     let b2 = am.get("b").unwrap();

//     for a in a1.borrow_mut().iter_mut() {
//         *a *= 2;
//     }

//     for b in b2.borrow_mut().iter_mut() {
//         *b *= 3;
//     }

//     println!("{a1:?}, {a2:?}");
//     println!("{b1:?}, {b2:?}")
// }