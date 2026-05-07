pub fn mut_shadowing() {

    // ================================================
    // MUT
    // Same variable
    // Same memory conceptually
    // Same type only
    // ================================================

    let mut age = 24;
    println!("{}",age);

    age = 25;

    println!("{}", age);

    // ================================================
    // SHADOWING
    // New variable
    // Can change type
    // ================================================

    let data: i32 = 100;

    println!("{}",data);

    let data = "Rust";

    println!("{}", data);

}