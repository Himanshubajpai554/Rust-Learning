// ==========================================================
// RUST FUNDAMENTALS
// HOME + INTRO + GET STARTED + SYNTAX + OUTPUT + COMMENTS
// ==========================================================

// --------------------------------------------
// SINGLE LINE COMMENT
// Rust ignores anything after //
// Useful for explanation and debugging notes
// --------------------------------------------

/*
    MULTI LINE COMMENT

    Used for:
    - documentation
    - temporary disabling code
    - explanations

    Rust compiler ignores this block
*/


// --------------------------------------------------
// fn main() is the ENTRY POINT of every Rust program
// Program execution starts from here
// --------------------------------------------------

pub fn shadowing() {

    // --------------------------------------------
    // println! is a MACRO
    // ! means macro, not normal function
    // It prints text to console with newline
    // --------------------------------------------

    println!("Hello, Rust!");

    // --------------------------------------------
    // print! prints WITHOUT newline
    // --------------------------------------------

    print!("This ");
    print!("is ");
    print!("same line\n");

    // --------------------------------------------
    // \n => newline character
    // --------------------------------------------

    println!("Line 1\nLine 2");

    // --------------------------------------------
    // {} is placeholder formatting
    // Similar to template placeholders
    // --------------------------------------------

    println!("My age is {}", 24);

    // --------------------------------------------
    // Multiple placeholders
    // --------------------------------------------

    println!("{} is learning {}", "Himanshu", "Rust");

    // --------------------------------------------
    // Named formatting
    // --------------------------------------------

    println!(
        "{name} is building {project}",
        name = "Tushar",
        project = "Authentication Engine"
    );

    // --------------------------------------------
    // Debug formatting
    // {:?} prints debug representation
    // --------------------------------------------

    println!("{:?}", (10, 20, 30));

    // --------------------------------------------
    // Basic syntax rules in Rust
    // --------------------------------------------

    // 1. Statements usually end with ;
    let x = 5;

    // 2. Variables are immutable by default
    println!("{}", x);

    // 3. Curly braces define blocks
    {
        let inside_block = 100;
        println!("{}", inside_block);
    }

    // --------------------------------------------
    // Rust is strongly typed
    // Compiler knows exact types
    // --------------------------------------------

    let language: &str = "Rust";

    println!("Language: {}", language);

    // --------------------------------------------
    // Rust compiles BEFORE running
    // rustc file.rs
    // ./file
    //
    // Cargo is Rust package manager
    // cargo new project_name
    // cargo run
    // cargo build
    // --------------------------------------------

}