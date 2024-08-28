// I thought about a string class from class. But its just extra work on top of Vector. So made that.

use std::ptr;
use std::alloc::{self, Layout};

use std::option::Option;

pub struct MyVec
{
    ptr: *mut u32,
    cap: usize,
    len: usize,
}

impl MyVec 
{
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn setup (&mut self) {
        self.cap = 1;

        let layout = Layout::array::<u32>(1).unwrap(); // new stuff here

        unsafe {
            // this is confusing. alloc always returns a u8 pointer. pointer to first byte.
            self.ptr = alloc::alloc(layout) as *mut u32;
        }
    }

    fn grow(&mut self) {

        if self.cap == 0 {
            return self.setup();
        }

        let new_cap = 2*self.cap;
        let new_layout = Layout::array::<u32>(new_cap).unwrap();

        let old_layout = Layout::array::<u32>(self.cap).unwrap();
        let old_ptr = self.ptr as *mut u8;

        unsafe { 
            self.ptr = alloc::realloc(old_ptr, old_layout, new_layout.size()) as *mut u32;
        }

        self.cap = new_cap;
    }

    fn push(&mut self, val: u32) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            // self.ptr + self.len effectively
            *self.ptr.wrapping_add(self.len) = val;
        }

        self.len += 1;
    }

    fn pop(&mut self) -> Option<u32> {
        if self.len == 0 {
            return None
        }

        let offset = self.len - 1;

        self.len -= 1;
        
        unsafe {
            return Some(*self.ptr.wrapping_add(offset))
        }
        // we would need to drop if we were storing complicated stuff.
    }

    fn get(&self, i: usize) -> Option<u32> {
        if i < self.len {
            unsafe {
                let val = *self.ptr.wrapping_add(i);
                return Some(val)
            }
        } else {
            return None
        }
    }

    fn set(&self, i: usize, new_val: u32) -> Option<u32> {
        if i < self.len {
            unsafe {
                let target = self.ptr.wrapping_add(i);
                let val = *target;

                *target = new_val;

                return Some(val)
            }
        } else {
            return None
        }

    }
}

impl Drop for MyVec 
{
    fn drop(&mut self) {
        if self.cap != 0 {

            let layout = Layout::array::<u32>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}



fn main() {
    let mut my_vec = MyVec::new();

    for i in 0..100 {
        my_vec.push(i);
    }

    my_vec.set(99, 255);

    println!(
        "Length: {}. First Element: {}. 50th element: {}. Last Element: {}", 
        my_vec.len, 
        my_vec.get(0).unwrap(), 
        my_vec.get(49).unwrap(),
        my_vec.get(99).unwrap()
    );

    for _i in 0..5 {
        let val = my_vec.pop().unwrap();

        println!("Popped off: {}", val);
    }
    println!("New Length: {}", my_vec.len);


}