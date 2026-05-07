pub fn matchh() {

    //example 1
    let day = 3;

    match day {

        1 => println!("Monday"),

        2 => println!("Tuesday"),

        3 => println!("Wednesday"),

        _ => println!("Invalid"),
    }


    //example 2

    let number = 2;

    let result = match number {

        1 => "One",

        2 => "Two",

        _ => "Unknown",
    };

    println!("{}", result);


    //example 3

    let day = 6;

    match day {

        1 | 2 | 3 | 4 | 5 => {
            println!("Weekday");
        }

        6 | 7 => {
            println!("Weekend");
        }

        _ => {
            println!("Invalid");
        }
    }


}