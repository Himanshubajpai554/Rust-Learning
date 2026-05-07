pub fn array() {

    // ------------------------------------------------
    // Array
    // Same type only
    // Fixed size
    // Stored in stack
    // Fast access
    // ------------------------------------------------

    let numbers = [10, 20, 30, 40, 50];

    println!("{:?}", numbers);

    // Access element
    println!("{}", numbers[0]);

    // Length
    println!("{}", numbers.len());

    // ------------------------------------------------
    // Iterate array
    // ------------------------------------------------

    for value in numbers {

        println!("{}", value);
    }

    //mutable data in array 

    let mut scores = [1, 2, 3, 4, 5];

    scores[0] = 100;

    println!("{:?}", scores);

}