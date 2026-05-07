pub fn loopp() {

    //Example 1
    let mut count = 0;

    loop {

        count += 1;

        if count == 5 {

            break;
        }

        println!("{}", count);
    }


    // Example 2
    let mut x = 0;

    let result = loop {

        x += 1;

        if x == 10 {

            break x * 2;
        }
    };

    println!("{}", result);


    //Example 3
    for i in 1..6 {

        println!("{}", i);
    }

    
    //Example 4
    let mut num = 1;

    while num <= 5 {

        println!("{}", num);

        num += 1;
    }

}