// We are just double checking our work to make sure we understand

use std::cell::RefCell; 
use std::rc::Rc;

#[derive(Debug)]
struct Counter {
    strikes: Vec<u8>,
}

struct Incrementor {
    pointer: *mut Counter,
    cell: Rc<RefCell<Counter>> // you cannot make these mutable normie refs.
}

impl Incrementor {
    fn increment_via_pointer(&mut self) {
        unsafe {
            (*self.pointer).strikes.push(0);
        }
    }

    fn increment_via_rc(&mut self) {
        self.cell.borrow_mut().strikes.push(0);
    }

    fn print_via_rc(&self) {
        println!("Count via safe rust: {:?}", self.cell.borrow().strikes.len());
    }

    fn print_via_pointer(&self) {
        unsafe {
            println!("Count via pointer: {}", (*self.pointer).strikes.len());
        }
    }
}


fn main() {
    let mut counter = Counter {
        strikes: vec![]
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
    // p1: 0x7ffc3549d290
    // p2: 0x7ffc3549d2d8
    // p3: 0x558b0d0d9b98

    let reference_count = Rc::strong_count(&rc);
    println!("Ref Count: {}", reference_count);
    // Ref Count: 3

    for i in 0..10000 {
        i1.increment_via_pointer();
        i1.increment_via_rc();
        i2.increment_via_pointer();
        i2.increment_via_rc();
    }

    // Count via safe rust: 40000
    // Count via pointer: 40000
    // Count via safe rust: 40000
    // Count via pointer: 40000

}