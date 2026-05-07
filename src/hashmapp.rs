use std::collections::HashMap;

pub fn hashmapp() {

    //Example 1
    let mut capitals = HashMap::new();

    capitals.insert("India", "Delhi");
    capitals.insert("Japan", "Tokyo");

    println!("{:?}", capitals);


    //Example 2
    let mut scores = HashMap::new();

    // Insert only if key absent
    scores.entry("Tushar").or_insert(0);

    // Update existing value
    *scores.entry("Tushar").or_insert(0) += 1;

    println!("{:?}", scores);


    //Example 3
    let mut capitals = HashMap::new();

    capitals.insert("India", "Delhi");

    // get returns Option
    if let Some(city) = capitals.get("India") {

        println!("{}", city);

    } else {

        println!("Not Found");
    }

}