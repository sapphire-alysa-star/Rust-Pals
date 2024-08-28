

// Shorthand for initializing a `String`.
macro_rules! S {
    ($e:expr) => {String::from($e)};
}

macro_rules! four {
    () => { 1 + 3 };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => { $a * ($b + $c) };
}

fn main() {
    let world = S!("World");

    let f1 = four!{};
    let f2 = four!();
    let f3 = four![];

    let ten = multiply_add!(2, 2, 3);

    println!("Hello, {}! Four: {} - {} - {}. Ten: {}", world, f1, f2, f3, ten);
}



// block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
// expr: an expression
// ident: an identifier (this includes keywords)
// item: an item, like a function, struct, module, impl, etc.
// lifetime: a lifetime (e.g. 'foo, 'static, ...)
// literal: a literal (e.g. "Hello World!", 3.14, '🦀', ...)
// meta: a meta item; the things that go inside the #[...] and #![...] attributes
// pat: a pattern
// path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
// stmt: a statement
// tt: a single token tree
// ty: a type
// vis: a possible empty visibility qualifier (e.g. pub, pub(in crate), ...)

















// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums

// Attribute-like macros that define custom attributes usable on any item

// Function-like macros that look like function calls but operate on the tokens specified as their argument

// We’ll talk about each of these in turn, but first, let’s look at why we even need macros when we already have functions.


// First, we use a set of parentheses to encompass the whole pattern. We use a dollar sign ($) to declare a variable in the macro system that will contain the Rust code matching the pattern. The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable. Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code. Within $() is $x:expr, which matches any Rust expression and gives the expression the name $x.

// The comma following $() indicates that a literal comma separator character could optionally appear after the code that matches the code in $(). The * specifies that the pattern matches zero or more of whatever precedes the *.

// When we call this macro with vec![1, 2, 3];, the $x pattern matches three times with the three expressions 1, 2, and 3.

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )* // repeat for each x
//             temp_vec
//         }
//     };
// }


// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// // }

// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
// struct Pancakes;

// fn main() {
//     Pancakes::hello_macro();
// }

// pub trait HelloMacro {
//     fn hello_macro();
// }

// use hello_macro::HelloMacro;

// struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

// fn main() {
//     Pancakes::hello_macro();
// }