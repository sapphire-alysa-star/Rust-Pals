use std::mem;
use std::ptr;
use std::alloc::{self, Layout};

use std::option::Option;

pub struct Vec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> 
where
    T: Copy
{
    pub fn new() -> Self {
        Vec {
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

        self.len - 1;
        
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

}



// C++ Code:

// class String
// {
//     public:
//         char* _text {nullptr};

//         String(const char* ch)
//         {
//             size_t sizeOfText = strlen(ch) + 1; // +1 to account for trailing NULL

//             // Dynamically allocate the correct amount of memory.
//             _text = new char[sizeOfText];

//             // If the allocation succeeds, copy the initialization string.
//             if (_text)
//             {
//                 strcpy(_text, ch);
//             }
//         }

//         ~String()
//         {
//             // Deallocate the memory that was previously reserved for the string.
//             delete[] _text;
//         }

//         void print() {
//             // so we can check things actually work
//             cout << this->_text << endl;
//         }
// };





