// When you want to reduce the size of an object

// To store almost never initialized optional values e.g. errors

// Moving is not free, so sometimes Box<T> could be faster

// When you want to move values between threads

// You can't use atomic instructions on large types, pointers are often used in the Lock-Free programming

// Dynamic dispatching: Box<dyn Trait>

// Recursive data structures

// Box is roughly functionally equivalent to C++’s unique_ptr, which is considered a smart pointer in C++. 
// So people sometimes refer to it as one because C++ people understand that immediately. 
// It’s not wrong to call it one; ownership may not seem magic in Rust but it is to other programmers!

// I think the term "normal pointer" is a bit misleading when working with Rust. When you use a pointer in C, you need to decide how it is managed. There are several choices:

// The pointer is a temporary view into some other data.

// The pointer has ownership of the allocation. You created it with malloc and you need to clean it up with free at the end.

// Something more complicated. E.g., the object is reference counted.

// In C, the way you pick between these is that either you insert a call to free after using it, or you don't. There's nothing in the type to tell you which one it is.

// In Rust, each use-case has a different type. The temporary view is the reference. The one with ownership of the allocation is the Box. If the value is reference counted, then it is Rc or Arc depending on thread safety. If you want to fall back to the C way of doing it, then it is the raw pointer.

// Given this, I don't think there's a clear way to say which pointer is the normal pointer. I don't think the reference is more "fundamental" than the box.

fn boxed_int () -> Box<i32> {
    let i: i32 = 6;

    return Box::new(i)
}

fn main () {
    let boxed = boxed_int();

    println!("{}", *boxed);

}