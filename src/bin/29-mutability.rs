// A magical journy through mutation.


#![allow(dead_code)]
#![allow(unused_variables)]

 fn borrow_checker() {
    let mut s = 0;

    let r1 = &mut s;
    let r2 = &mut s; 
    
    // This line will trigger the borrow checker. Though only if you use r1 and r2 later. If we dont use the variables no borrow check.

    // println!("{}, {}", r1, r2); 
 }

 fn raw_pointers() {
    let mut s = 0;

    let p1 = &mut s as *mut i32;
    let p2 = &mut s as *mut i32;

    let p3 = std::ptr::addr_of_mut!(s);

    unsafe {
        *p1 += 1;
        *p2 += 1;
        *p3 += 1;
    }

    println!("Value of s: {}", s);
    // Value of s: 3
}

fn main() {
    borrow_checker();

    raw_pointers();

}