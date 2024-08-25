// I thought about a string class from class. But its just extra work on top of Vector. So made that.

use std::ptr;
use std::alloc::{self, Layout};

use std::option::Option;

pub struct MyVec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> 
where
    T: Copy
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

        let layout = Layout::array::<T>(1).unwrap();

        unsafe {
            self.ptr = alloc::alloc(layout) as *mut T;
        }
    }

    fn grow(&mut self) {

        if self.cap == 0 {
            return self.setup();
        }

        let new_cap = 2*self.cap;
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let old_layout = Layout::array::<T>(self.cap).unwrap();
        let old_ptr = self.ptr as *mut u8;

        unsafe { 
            self.ptr = alloc::realloc(old_ptr, old_layout, new_layout.size()) as *mut T;
        }

        self.cap = new_cap;
    }

    fn push(&mut self, val: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            *self.ptr.wrapping_add(self.len) = val;
        }

        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None
        }

        let offset = self.len - 1;

        self.len -= 1;
        
        unsafe {
            return Some(*self.ptr.wrapping_add(offset))
        }

    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.len {
            unsafe {
                let val = *self.ptr.wrapping_add(i);
                return Some(val)
            }
        } else {
            return None
        }
    }

    fn set(&self, i: usize, new_val: T) -> Option<T> {
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



fn main() {
    let mut my_vec: MyVec<u8> = MyVec::new();

    for i in 0..100 {
        my_vec.push(i);
    }

    println!(
        "Length: {}. First Element: {}. 50th element: {}. Last Element: {}", 
        my_vec.len, 
        my_vec.get(0).unwrap(), 
        my_vec.get(49).unwrap(),
        my_vec.get(99).unwrap()
    );

    for i in 0..5 {
        let val = my_vec.pop().unwrap();

        println!("Popped off: {}", val);
    }
    println!("New Length: {}", my_vec.len);


}