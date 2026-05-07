enum Direction {

    Up,
    Down,
    Left,
    Right,
}

enum LoginResult {

    Success(String),

    Error(String),
}


pub fn enumss() {

    //Example 1
    let move_to = Direction::Left;

    match move_to {

        Direction::Up => println!("Going Up"),

        Direction::Down => println!("Going Down"),

        Direction::Left => println!("Going Left"),

        Direction::Right => println!("Going Right"),
    }


    //Example 2
    let result = LoginResult::Success(
        String::from("JWT Token")
    );
    
    let result1 = LoginResult::Error(
        String::from("JWT Token error")
    );

    match result {

        LoginResult::Success(token) => {
            println!("Token: {}", token);
        }

        LoginResult::Error(message) => {
            println!("Error: {}", message);
        }
    }
    
    match result1 {

        LoginResult::Success(token) => {
            println!("Token: {}", token);
        }

        LoginResult::Error(message) => {
            println!("Error: {}", message);
        }
    }

}