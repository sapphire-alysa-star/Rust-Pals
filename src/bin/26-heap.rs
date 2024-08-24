// Box and unique_pointer are extremely similar. 

#![allow(dead_code)]
#![allow(unused_variables)]


const LENGTH: usize = 8192*1024*10; // 10x my abailable stack memory in bytes.


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

fn main() {

    // stack_overflow(); 
    // this stack overflows as you might guess

    let boxed_slice_1: Box<[u8]> = vec![0; LENGTH].into_boxed_slice();

    let boxed_slice_2 = big_box(7, LENGTH);

    println!("Boxxed Slice Two has first element {} and length {}", boxed_slice_2[0], boxed_slice_2.len());
    // Boxxed Slice Two has first element 7 and length 83886080


    // let mut b = Vec::with_capacity(LENGTH);
    // b.extend(std::iter::repeat(v).take(1000));
    // b.into_boxed_slice().try_into().unwrap();

}

// fn main() {
//     const MAX_STACK_BYTES: usize = 8192*1024; 

//     // Rust is truly amazingly poorly implemented and allocating directly with Box does NOT work lol.
//     // let boxed_array = Box::new([0 as u8; 10*MAX_STACK_BYTES]); 
// }

//     // // I have approximately 8192 KB of stack memory.
//     // const NUM_BYTES: usize = 8192*1024; // to use as array size you need a const. const must be typed. usize is correct type.
//     // const LESS_BYTES: usize = 8100*1024;

//     // // let too_big: [u8; NUM_BYTES] = [0; NUM_BYTES];
//     // // thread 'main' has overflowed its stack
//     // // fatal runtime error: stack overflow
//     // // Aborted

//     // let array: [u8; LESS_BYTES] = [0; LESS_BYTES]; // -- does not break on my machine.
//     // let mem_size_in_bytes = size_of_val(&array);

//     // let kilo_bytes = mem_size_in_bytes / 1024;
//     // let extra_bytes = mem_size_in_bytes % 1024;