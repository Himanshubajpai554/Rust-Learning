pub fn stringg() {

    // String literal
    let a = "Hello";
    println!("{}",a);

    // Heap allocated String
    let mut b = String::from("Rust");

    b.push('!');
    b.push_str(" Backend");

    println!("{}", b);


    //Concatinate 

    let first = String::from("Hello ");
    let second = String::from("World");

    let result = first + &second;

    println!("{}", result);

}