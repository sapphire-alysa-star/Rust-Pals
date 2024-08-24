#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::{alloc, Layout};
use std::slice::from_raw_parts_mut;
use std::time::SystemTime;
use std::ptr::write_bytes;

const LENGTH: usize = 8192*1024*10; // 10x my abailable stack memory in bytes.

// For u8's you can use write_bytes which is similar to memset to move rpaidly!
// You can also use write_bytes for zero'd data. 
// Vec does both the fast way!

fn big_allocation(val: u8, length: usize) -> Box<[u8]> {

    unsafe {
        let layout = Layout::array::<u32>(length).unwrap();
        let p: *mut u8 = alloc(layout);

        write_bytes(p, val, length);

        let slice = from_raw_parts_mut(p, length);

        Box::<[u8]>::from_raw(slice as *mut [u8])
    }
    
}

fn main() {
    // Gonna change the order and see if that changes something.
    let t_vec = SystemTime::now();
    let boxed_vec = big_box(5, LENGTH);
    let duration_vec = t_vec.elapsed();

    println!("Time elapsed to run vec! version: {:?}", duration_vec);

    let t_alloc = SystemTime::now();
    let boxed_allocation = big_allocation(5, LENGTH);
    let duration_alloc = t_alloc.elapsed();

    println!("Time elapsed to run alloc version: {:?}", duration_alloc);

    // Ok sometimes the second one runs slower. We have the correct implementation and learned stuff!

    // Before swapped order:

    // Time elapsed to run alloc version: Ok(8.963157ms)
    // Time elapsed to run vec! version: Ok(42.518575ms)

    // Time elapsed to run alloc version: Ok(18.928455ms)
    // Time elapsed to run vec! version: Ok(23.303216ms)


    // After I swapped the order:

    // Time elapsed to run vec! version: Ok(8.745203ms)
    // Time elapsed to run alloc version: Ok(15.427522ms)

    // Time elapsed to run vec! version: Ok(17.209153ms)
    // Time elapsed to run alloc version: Ok(75.184056ms)

}

fn big_box(val: u8, length: usize) -> Box<[u8]> {
    let boxed_slice: Box<[u8]> = vec![val; length].into_boxed_slice();

    boxed_slice
}