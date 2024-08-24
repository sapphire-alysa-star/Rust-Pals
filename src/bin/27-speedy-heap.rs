// Box and unique_pointer are extremely similar. 

// So lets look at putting stuff on the heap. And some manual memory allocation!

#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::{alloc, Layout};
use std::slice::from_raw_parts_mut;
use std::time::SystemTime;
use std::ptr::copy_nonoverlapping;

const LENGTH: usize = 8192*1024*10; // 10x my abailable stack memory in bytes.

// From elem actually faster for specifically u8???
// impl SpecFromElem for u8 {
//     #[inline]
//     fn from_elem<A: Allocator>(elem: u8, n: usize, alloc: A) -> Vec<u8, A> {
//         if elem == 0 {
//             return Vec { buf: RawVec::with_capacity_zeroed_in(n, alloc), len: n };
//         }
//         let mut v = Vec::with_capacity_in(n, alloc);
//         unsafe {
//             ptr::write_bytes(v.as_mut_ptr(), elem, n);
//             v.set_len(n);
//         }
//         v
//     }
// }

fn big_box(val: u32, length: usize) -> Box<[u32]> {
    let boxed_slice: Box<[u32]> = vec![val; length].into_boxed_slice();

    boxed_slice

    // This is surely the best plan but maybe we can do it more directly for learning.
}

fn big_allocation(val: u32, length: usize) -> Box<[u32]> {

    unsafe {
        let layout = Layout::array::<u32>(length).unwrap();
        let p: *mut u8 = alloc(layout);
        let p32 = p as *mut u32;

        // this loop is very slow sadly. But you know whatever. For learning.
        // for i in 0..length {
        //     *p.wrapping_add(i) = val;
        // }

        let slice = from_raw_parts_mut(p32, length);
        slice.fill(val); // this is 5x faster but still slower than vec!

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