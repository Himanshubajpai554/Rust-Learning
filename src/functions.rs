pub fn function() {

    say_hello();

    greet("Tushar");

    let result = add(10, 20);

    println!("{}", result);

}

// ------------------------------------------------
// No parameter
// ------------------------------------------------
fn say_hello() {

    println!("Hello");
}

// ------------------------------------------------
// Parameter function
// ------------------------------------------------
fn greet(name: &str) {

    println!("Welcome {}", name);
}

// ------------------------------------------------
// Return value
// -> means returns
// ------------------------------------------------
fn add(a: i32, b: i32) -> i32 {

    // Last line without ;
    // automatically returned

    a + b
}