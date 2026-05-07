pub fn borrowing() {

    //Example 1
    let name = String::from("Tushar");

    print_name(&name);

    // Still usable
    println!("{}", name);


    //Example 2
    let mut text = String::from("Rust");

    change(&mut text);

    println!("{}", text);

}

fn print_name(data: &String) {

    println!("{}", data);

}


fn change(value: &mut String) {

    value.push_str(" Language");

}