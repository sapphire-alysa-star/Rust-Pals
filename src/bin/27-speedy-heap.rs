#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::{alloc, Layout};
use std::slice::from_raw_parts_mut;
use std::time::SystemTime;

const LENGTH: usize = 8192*1024*10; // 10x my abailable stack memory in bytes.

fn big_allocation(val: u32, length: usize) -> Box<[u32]> {

    unsafe {
        let layout = Layout::array::<u32>(length).unwrap();
        let p: *mut u8 = alloc(layout);
        let p32 = p as *mut u32;

        for i in 0..length {
            *p32.wrapping_add(i) = val;
        }
        let slice = from_raw_parts_mut(p32, length);
        // slice.fill(val); // no loop sorry.

        Box::<[u32]>::from_raw(slice as *mut [u32])

        // p as used above is a thin pointer. Its 'just' a memory address with a type.
        // *mut [u8] is a 'fat pointer'. It stores both a length and an address. As well as being typed.
    }
    
}

fn main() {
    let t_alloc = SystemTime::now();
    let boxed_slice_alloc = big_allocation(7, LENGTH);
    let duration_alloc = t_alloc.elapsed();

    println!("Boxxed_Slice_Alloc has 99th element {} and length {}", boxed_slice_alloc[99], boxed_slice_alloc.len());
    println!("Time elapsed to run alloc version: {:?}", duration_alloc);

    let t_vec = SystemTime::now();
    let boxed_slice_vec = big_box(5, LENGTH);
    let duration_vec = t_vec.elapsed();

    println!("Boxxed_Slice_Vec has first element {} and length {}", boxed_slice_vec[0], boxed_slice_vec.len());
    println!("Time elapsed to run vec! version: {:?}", duration_vec);

    // Either they have similar speed. Or whichever runs second sometimes hits a speed bump.

    // Boxxed_Slice_Alloc has 99th element 7 and length 83886080
    // Time elapsed to run alloc version: Ok(651.759144ms)
    // Boxxed_Slice_Vec has first element 5 and length 83886080
    // Time elapsed to run vec! version: Ok(498.07131ms)
}

fn big_box(val: u32, length: usize) -> Box<[u32]> {
    let boxed_slice: Box<[u32]> = vec![val; length].into_boxed_slice();

    boxed_slice
}