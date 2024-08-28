// Box and unique_pointer are extremely similar. 

// So lets look at putting stuff on the heap. And some manual memory allocation!

#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::{alloc, Layout};
use std::slice::from_raw_parts_mut;

const LENGTH: usize = 8192*1024*10; // 10x my available stack memory in bytes.

fn stack_overflow() -> Box<[u8; LENGTH]> {

    let boxed_array = Box::new([0 as u8; LENGTH]);

    return boxed_array

    // this should obviously work but it does not lol. 
}

fn big_box(val: u8, length: usize) -> Box<[u8]> {
    let boxed_slice: Box<[u8]> = vec![val; length].into_boxed_slice();

    boxed_slice

    // This is surely the best plan but maybe we can do it more directly for learning.
}

fn big_allocation(val: u8, length: usize) -> Box<[u8]> {

    unsafe {
        let layout = Layout::array::<u8>(length).unwrap();
        let p = alloc(layout);

        for i in 0..length {
            *p.wrapping_add(i) = val;
        }

        let slice = from_raw_parts_mut(p, length);

        Box::<[u8]>::from_raw(slice as *mut [u8])

        // p as used above is a thin pointer. Its 'just' a memory address with a type.
        // *mut [u8] is a 'enhanced pointer'. It stores both a length and an address. As well as being typed.
    }
    
}

fn main() {

    // stack_overflow(); 
    // this stack overflows as you might guess

    let boxed_slice_1 = big_box(5, LENGTH);
    println!("Boxxed Slice One has first element {} and length {}", boxed_slice_1[0], boxed_slice_1.len());
    // Boxxed Slice One has first element 5 and length 83886080

    let boxed_slice_2 = big_allocation(7, LENGTH + 920);
    println!("Boxxed Slice Two has 99th element {} and length {}", boxed_slice_2[99], boxed_slice_2.len());
    // Boxxed Slice Two has 99th element 7 and length 83887000
}