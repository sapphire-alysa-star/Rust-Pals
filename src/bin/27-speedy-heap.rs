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

        let slice = from_raw_parts_mut(p32, length);
        slice.fill(val); // no loop sorry.

        Box::<[u32]>::from_raw(slice as *mut [u32])

        // p as used above is a thin pointer. Its 'just' a memory address with a type.
        // *mut [u8] is a 'fat pointer'. It stores both a length and an address. As well as being typed.
    }
    
}

fn main() {
    let t1 = SystemTime::now();
    let boxed_slice_1 = big_box(5, LENGTH);
    let duration_1 = t1.elapsed();

    println!("Boxxed Slice One has first element {} and length {}", boxed_slice_1[0], boxed_slice_1.len());
    println!("Time elapsed to run vec! version: {:?}", duration_1);

    let t2 = SystemTime::now();
    let boxed_slice_2 = big_allocation(7, LENGTH + 920);
    let duration_2 = t2.elapsed();

    println!("Boxxed Slice Two has 99th element {} and length {}", boxed_slice_2[99], boxed_slice_2.len());
    println!("Time elapsed to run alloc version: {:?}", duration_2);

    // for u32 they run in the same time! Though variance. sometimes either wins the race
    // Time elapsed to run vec! version: Ok(423.783211ms)
    // Time elapsed to run alloc version: Ok(435.575445ms)
}

fn big_box(val: u32, length: usize) -> Box<[u32]> {
    let boxed_slice: Box<[u32]> = vec![val; length].into_boxed_slice();

    boxed_slice
}