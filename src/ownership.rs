pub fn ownership() {

    let a = String::from("Hello");

    // Ownership moved
    let b = a;

    println!("{}", b);

    // ERROR
    // println!("{}", a);

    //Example 2

    let a = String::from("Hello");

    // Deep copy
    let b = a.clone();

    println!("{}", a);
    println!("{}", b);

}