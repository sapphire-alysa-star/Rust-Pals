// int main() {
//     const int num_bytes = 8192*1024; 

//     // char chars[num_bytes]; // The max num_bytes I can use is not consistent on my machine. -- SEG FAULT HERE

//     char* chars = new char[num_bytes];       // Request memory for the variable. No seg faults here!

//     std::fill(chars, chars + num_bytes, '0'); // You dont actually need this line. But wanted to show they are filled.

//     return 0;
// }


fn main() {
    // I have approximately 8192 KB of stack memory.
    const NUM_BYTES: usize = 8192*1024; // to use as array size you need a const. const must be typed. usize is correct type.
    const LESS_BYTES: usize = 8100*1024;


    // let too_big: [u8; NUM_BYTES] = [0; NUM_BYTES];
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // Aborted

    let array: [u8; LESS_BYTES] = [0; LESS_BYTES]; // -- does not break on my machine.
}



