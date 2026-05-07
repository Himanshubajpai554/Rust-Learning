struct User {

    name: String,
    age: u32,
    active: bool,
}

struct User1 {

    name: String,
    age: u32,
}

pub fn structt() {

    //Example 1
    // --------------------------------------------
    // Create struct object
    // --------------------------------------------

    let user = User {

        name: String::from("Tushar"),
        age: 24,
        active: true,
    };

    println!("{}", user.name);
    println!("{}", user.age);
    println!("{}", user.active);


    //Example 2

    let mut user1 = User1 {

        name: String::from("Himanshu"),
        age: 24,
    };

    // Update value
    user1.name = String::from("Tushar");

    println!("{}", user1.name);
    println!("{}", user1.age);
}