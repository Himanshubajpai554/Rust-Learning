pub fn tuple() {

    // ------------------------------------------------
    // Tuple = fixed size collection
    // Can store DIFFERENT TYPES
    // ------------------------------------------------

    let person = ("Tushar", 24, true);

    // Access using index
    println!("{}", person.0);
    println!("{}", person.1);
    println!("{}", person.2);

    // ------------------------------------------------
    // Destructuring
    // Extract values into variables
    // ------------------------------------------------

    let (name, age, active) = person;

    println!("{}", name);
    println!("{}", age);
    println!("{}", active);

}