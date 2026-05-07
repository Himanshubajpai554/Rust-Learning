pub fn vector() {

    // ------------------------------------------------
    // Vec = dynamic array
    // Stored in heap
    // Growable
    // ------------------------------------------------

    let mut fruits = Vec::new();

    fruits.push("Apple");
    fruits.push("Banana");
    fruits.push("Mango");

    println!("{:?}", fruits);


    //Vector with Initial Values

    let mut numbers = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // Add element
    numbers.push(5);

    // Remove last
    numbers.pop();

    println!("{:?}", numbers);


    //Insert and Remove

    let mut data = vec![10, 20, 30];

    // Insert at index
    data.insert(1, 99);

    println!("{:?}", data);

    // Remove from index
    data.remove(2);

    println!("{:?}", data);


    //Iterate Vector

    let nums = vec![1, 2, 3, 4];

    for value in &nums {

        println!("{}", value);
    }


    //


}