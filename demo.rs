// Demo of Rust with explanations
//
// This demo is suitable for newcomers to Rust who know
// other programming langauges such as C, Java, Python, etc.

// Compiler directives
//
// The Rust compiler will normally warn about issues such
// as dead code, unused variables, misformatted names, etc.
//
// For this demo, we don't want the compiler to warn us,
// because this demo code is meant to be easy, not perfect.
//
// To allow dead code, unused variables, etc. we use a compiler
// directive at the start of a file (or start of a module).
//
#![allow(unused)] // Allow all unused issues, such as dead code.

// External crates
//
// Link an external crate library, and import all its 
// items under module named the same as the library.
// See https://rustbyexample.com/crates/link.html
//
// A "crate" is a package of Rust code. 
//
// A "library crate" contains code intended to be used in other 
// programs, such as by using `Cargo.toml`,  `extern crate`, etc.
//
// The project we're been building is a binary crate, 
// which is an executable, to be run on the command line.
//
extern crate rand; // randomization functions
extern crate html5ever; // HTML5 parser

// Import modules
//
use std::io; // Use the standard library input-output module
use std::net; // Use the standard library networking module
use rand::Rng; // Use the rand crate random number generator

// Enumerations
//
// An enumeration is a type that can have a fixed set of values;
// the set of values are called the enumumeration's variants.
//
// Enumerations are often referred to as enums. 
//
// The `Result` enum and its variants:
//
//   * The `Ok` variant indicates the operation succeeded, 
//     and the variant contains the successful value.
//
//   * The `Err` variant indicates the operation failed, 
//     and the variant contains information about the failure.
//
// The `Ordering` enum and its variants:
//
//   * `Less`
//
//.  * `Greater`
//
//   * `Equal` 
//
use std::result; // Enums for error handling
use std::cmp::Ordering; // Enums for `cmp` compare function

// Testing
// 
// Rust and Cargo have good testing capabilities,
// by defining test functions in their own module.
//
// To test all the code, run `cargo test`.
//
#[cfg(test)] // Annotation: use code for `cargo test`, not for `cargo build`
mod tests { // Define a module named "tests"

    use super::*;  // Bring in code from the outer module

    #[test] // Annotation: the next function is a test
    fn blank() { // Define a function as usual; an empty test always succeeds
    }

    #[test]            
    fn foo_test() {  
        assert!(foo()); // Assert is a test macro that will succeed or panic!
    }

    #[test]            
    fn echo_test() {
        let s = "hello"; // This text is type `&str`, not type `String`
        assert_eq!(echo(s), s); // Assert the `echo` function returns the same text
    }
}

 // Define a public function named "foo" that returns a boolean 
pub fn foo() -> bool {
    true // Return true
}

// Define a public function named "echo" that takes one argument `s`
// that is a string fragment type `&str`, and returns the same text.
pub fn echo(s: &str) -> &str {
    s
}

// Demo println
//
// This function shows the `println!` macro.
// Note that a macro is not the same as a function.
//
fn demo_println() {

    // Print to standard output
    println!("Hello"); 
    
    // String interpolation placeholders
    println!("Hello {} and {}", "Alice", "Bob");

}

// Demo variables
//
// This shows examples of variables and a constant.
//
fn demo_variables() {

    // Constant
    const a = 1; // A constant is always immutable

    // Variable
    let b = 1; // A variable is immutable by default

    // Variable that is mutable
    let mut c = 1; // A variable with `mut` is mutable.

    // Types
    let c: i32 = 1; // Variable is type signed integer 32 bit
    let d: f64 = 1; // Variable is type floating point 64 bit

    // Char
    let letter: char = 'x'; // Any Unicode Scalar Value, not just one byte 

    // Array
    let arr: i32 = [1, 2, 3]; // Each element must be the same type

    // Tuple
    let tup: (i32, f64, char) = (1, 2, 'x'); // Each element can be its own type

    // Object with initialization
    let mut f = String::new(); // Create a new empty string
    let mut g = String::from("hello"); // Create a new string that says "hello"

    // Integer literals
    let h = 98_222; // Decimal  
    let i = 0xff; // Hex        
    let j = 0o77; // Octal      
    let k = 0b1111_0000; // Binary      
    let l = b'A'; // Byte (u8 only)     

}

fn demo_array_access() {
    let arr = [1, 2, 3];
    let x = a[0];
    let y = a[1];
    let z = a[2];
}

fn demo_tuple_destructure() {
    let tup = (1, 2, 3);
    let (x, y, z) = tup;
}

fn demo_control_flow() {
    let x = true;

    if x {
        //...
    }

    if x {
        //...
    } else {
        //...
    }

    if x {
        //...
    } else if y {
        //...
    } else if z {
        //...
    } else {
        //...
    }

    loop {
        //...
        break;
    }

    while b {
        //...
    }

    let arr = [1, 2, 3];
    for a in arr.iter() {
        //...
    }

}

// Demo compare
//
// This function depends on an earlier line:
//
//     use std::cmp::Ordering;
//
// The enumeration variants:
//
//   * Less
//   * Greater
//   * Equal
//
fn demo_compare() {
    let x = 1;
    let y = 2;
    match x.cmp(&y) {
        Ordering::Less    => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal   => println!("equal"),
    }
}    

// Demo shadow variable
//
// Notice that we create two different variables named `x`.
//
// Rust enables us to shadow the previous value of `x` with a new one.
//
// Shadowing lets us reuse the guess variable name rather 
// than forcing us to create two unique variables.
//
// This feature is often used when coverting a value to a similar value,
// such as converting a value from one type to another type. 
//
fn demo_shadow_variable() {
    let x = "    hello     "; // a string with some whitespace padding
    println!("x is {} before trimming", x);
    let x = x.trim();
    println!("x is {} after trimming", x);
}

// Demo random variable
//
// This function depends on an earlier line:
//
//     extern crate rand;
//
// This function generates a random number 
// between 1 inclusive and 101 exclusive.
//
fn demo_random_variable() {
    let r = rand::thread_rng().gen_range(1, 101);
}

fn demo_string_append() {
    let mut s = String::from("hello");
    s.push(' '); // append one character
    s.push_str("world"); // append a `&str` string fragment
}

//TODO
// fn demo_output_synchronization() ->  Result<(), &'static str> {
//     use std::io::{self, Write};
//
//      // Implicit synchronization
//      io::stdout().write(b"hello")?;

//      // Explicit synchronization
//      let stdout = io::stdout();
//      let mut handle = stdout.lock();
//      handle.write(b"hello")?;
// }
// Demo a function to return a Result enumeration variant
//
// This function depends on an earlier line:
//
//     use std::result;
//
// A function like this would typically return:
//
//   * `Ok` with a valid value, whatever that means contextually
//   * `Err` with an error message
//
// This function always returns Ok without any particular value;
// this is useful if the caller wants to know the call succeeded.
// 
//fn demo_result() -> Result<(), &'static str> {
//      Ok(())
//}

// Demo result with error handing
//
// This example tries to parse a string, and shows a typical
// way to do error handling by using a match and result.
//
// The `parse` function returns a Result that is either:
//
//   * Ok, which contains the result number.
//   * Err, which contains more error information.
//
// The `match` function below first tries to match both:
//
//   * Ok(num). We can use the number.
//   * Err(_). The underscore is a catchall meaning any value.
//
//
fn demo_result_with_error_handling() {
    // Create some text
    let x = "123"
    // Try to parse the text into an unsigned integer number,
    // then try to match the Result to either Ok or Err.
    let x: u32 = match x.parse() { 
        Ok(num) => println!("parse num is {}", num);
        Err(_) => println!("parse failed");
    }; // Note semicolon which ends the statement
}

// Demo input
//
// This function depends on an earlier line:
//
//     use std::io;
//
fn demo_input() {
    let mut s = String::new(); // Create a mutable string

    println!("Please enter some text:");
    io::stdin().read_line(&mut s); // IO with no error handling

    println!("Please enter some more text:");
    io::stdin().read_line(&mut s) // IO with minimal error handing on the next line
    .expect("oops"); // If there's an error, this line crashes and prints an error
}

// Demo output
//
// This function depends on an earlier line:
//
//     use std::io;
//
fn demo_output() {
    //TODO
    //io::stdout().write(b"hello")?; // TODO: what is the `b` doing?

    //TODO
    //io::stdout().write(b s);
    //.expect("oops");
}

// Demo convert a string to a number
//
// Notice that we create two different variables named `x`.
//
// Rust enables us to shadow the previous value of `x` with a new one.
// This feature is often used in similar situations in which you want
// to convert a value from one type to another type. 
//
// Shadowing lets us reuse the guess variable name rather 
// than forcing us to create two unique variables.
//
fn demo_convert_a_string_to_a_number() {
    let x = "  123  "; // a string with some whitespace padding
    let x = x.trim();  // trim the whitespace and use a shadow variable
    let x: u32 = x.parse() // parse to a number and use another shadow variable
    .expect("must be a number"); // if there's an error, then crash
}

// The `main` function is a special name, 
// much like in C/C++, because it runs first.
fn main() {
    demo_println();
    demo_compare();
    demo_shadow();
    demo_array_access();
    demo_tuple_destructure();
    demo_control_flow();
    demo_random_variable();
    demo_result_with_error_handling();
    demo_string_append();
    demo_input();
    demo_output();
    demo_convert_a_string_to_a_number();
}
