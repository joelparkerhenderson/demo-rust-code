// Demo of Rust with explanations
//
// This demo is suitable for newcomers to Rust who know
// other programming langauges such as C, Java, Python, etc.

////
//
// Introduction
//
////

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

// `error_chain!` can recurse deeply, sp we limit it.
#![recursion_limit = "1024"]

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
#[macro_use] extern crate error_chain; // error handling everywhere
extern crate getopts; // command line option parser
extern crate rand; // randomization functions

// Import modules
//
use std::env; // Use the standard library environment module
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
// The `std::result Result` enum and its variants:
//
//   * The `Ok` variant indicates the operation succeeded, 
//     and the variant contains the successful value.
//
//   * The `Err` variant indicates the operation failed, 
//     and the variant contains information about the failure.
//
//   * We prefer to use the `error-chain` crate for Result.
//
// The `std::cmp::Ordering` enum and its variants:
//
//   * `Less`
//
//.  * `Greater`
//
//   * `Equal` 
//
use std::result; // Enums for error handling
use std::cmp::Ordering; // Enums for `cmp` compare function

////
//
// Errors
//
////

// This code is specificalliy for the `error-chain` crate.
// It sets up typical error handling types that we use everywhere.
// Notably, it sets up a `Result` type that we prefer to `std::result`.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;

////
//
// Test
//
////

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

// Define a public function named `foo`.
// The function has no arguments.
// The function returns a boolean. 
//
pub fn foo() -> bool {
    true // Return true
}

// Define a public function named `echo`.
// The function has one argument `s` of type `&str` string fragment.
// The function returns one value of type `&str` string fragment.
// 
 and returns the same text.
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

// Demo constants
//
// A constant name uses the convention of all uppercase.
//
// Constants can be declared in any scope including global. 
//
// Constants require explicit type annotation.
//
// Rust has two different types of constants. 
//
//   * `const`: An immutable value; this is the common case.
//   * `static`: A possibly-mutable variable with `'static `lifetime.
//
fn demo_constants() {

    // Immutable
    const FOO: i32 = 1;

    // Static lifetime
    static GOO: &'static str = "Hello World";

}

// Demo variables
//
// This shows examples of variables and a constant.
//
fn demo_variables() {

    // Variable
    let b = 1; // A variable is immutable by default

    // Variable that is mutable
    let mut c = 1; // A variable with `mut` is mutable.

    // Types
    let c: i32 = 1; // Variable is type signed integer 32 bit
    let d: f64 = 1.1; // Variable is type floating point 64 bit

    // Char
    let letter: char = 'x'; // Any Unicode Scalar Value, not just one byte 

    // Array
    let arr = [1, 2, 3]; // Each element must be the same type 

    // Tuple
    let tup: (i32, f64, char) = (1, 1.1, 'x'); // Each element can be its own type

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
    let a = arr[0]; // a=10
    let b = arr[1]; // b=20
    let c = arr[2]; // c=30
}

fn demo_tuple_destructure() {
    let tup = (1, 1.1, 'x');
    let (a, b, c) = tup; // This sets a=1, b=1.1, c='x'
}

fn demo_control_flow() {
    let a = true;
    let b = true;
    let c = true;

    if a {
        //...
    }

    if a {
        //...
    } else {
        //...
    }

    if a {
        //...
    } else if b {
        //...
    } else if c {
        //...
    } else {
        //...
    }

    loop {
        //...
        break;
    }

    while a {
        //...
        break;
    }

    let arr = [1, 2, 3];
    for item in arr.iter() {
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
fn demo_shadow() {
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
    let x = "123";
    // Try to parse the text into an unsigned integer number,
    // then try to match the Result to either Ok or Err.
    let x: u32 = match x.parse() { 
        Ok(num) => num,
        Err(_) => 0
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

// Demo file name to string.
//
// This includes a utf8-encoded file as a string, 
// i.e. this looks for a file named "text.txt",
// then opens the file and returns the file's text.
// 
// The file is located relative to the current file,
// similarly to how modules are found.
//
// This macro will yield an expression of type `&'static str` 
// which is the contents of the file.
//
fn demo_file_name_to_string() {
    let s = include_str!("text.txt");
}

use std::io::Read;
fn demo_file_path_to_string() -> String {
    let name = "text.txt";
    let path = std::path::Path::new(name);
    let mut file = std::fs::File::open(path)
    .expect("file open failed");
    let mut s = String::new();
    file.read_to_string(&mut s)
    .expect("file read failed");
    s
}

////
//
// main
//
////

// The `main` function is a special name, 
// much like in C/C++, because it runs first.
//
// It's fine if you write typical code in the main function.
// We suggest this workflow: start with error handling code, 
// then get any command line options, then run typical code. 
// 
// We like to use these steps for clarity:
//
//   * `main` doesn't do anything except call the next function;
//     we do it this way to make the workflow easier to follow.
//
//   * `main_error_chain` sets up the `error-chain` crate info;
//     we use the error-chain documentation code boilerplate.
//
//   * `main_getopts` sets up the `getopts` crate info;
//     we set up all the command-line options, such as flags.
//
//   * `main_help` prints some help text and usage information;
//     much of this is automatic thanks to the `getopts` crate.
//
//   * `main_version` prints the version number;
//     we like to use semantic versioning, such as "x.y.z".
//
//   * `main_run` is where your typical code goes and does work.
//
fn main() {
    main_error_chain();
}

// We wrap everything in error handling code,
// which is provided by the crate `error-chain`.
//
// This code is from the error-chain documentation,
// and provides a simple way to print all errors.
// All our typical programs use this exact code.
//
#[allow(dead_code)]
fn main_error_chain() {
    if let Err(ref e) = main_getopts() {
        use std::io::Write;
        use error_chain::ChainedError; 
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Get the command line options.
// Customize this for your program's options.
//
// Return `Result<()>` which is an `error-chain` standard
// that means any result or any error-chain compatible error.
//
fn main_getopts() -> Result<()> {

    // Get all the args into a vector, for easier processing.
    // Note that std::env::args will panic if any argument contains
    // invalid Unicode; see the Rust docs for how to handle this.
    let args: Vec<String> = std::env::args().collect();
    
    // Unix convention puts the program name in the first argument.
    // However, this is not secure nor reliable, so use at your own risk.
    let program = args[0].clone(); 

    // Create the options parser, then add option flags and option values.
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print the help information");
    opts.optflag("v", "version", "print the version number");

    // Parse the args
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        main_help(&program, opts);
        return Ok(());
    }

    if matches.opt_present("v") {
        main_version();
        return Ok(());
    }

    main_run(&program, opts)
}

fn main_run(program: &str, opts: getopts::Options) -> Result<()> {
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
    Ok(())
}

fn main_help(program: &str, opts: getopts::Options) {
    let brief = format!("Help: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main_version() {
    println!("x.y.z");
}
