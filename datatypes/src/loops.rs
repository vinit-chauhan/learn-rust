pub fn exec() {
    // loop_with_return_value();
    // while_loop();
    for_loop()
}

pub fn infinite_loop() {
    let mut count: i32 = 0;
    loop {
        if count == 10 {
            break;
        }
        count += 1;
        println!("count: {count}");
    }

    println!("Exited loop!!")
}

// break in loop can return a value.
pub fn loop_with_return_value() {
    let mut count: i32 = 0;
    let res: i32 = loop {
        if count == 10 {
            break count + 12;
        }
        count += 1;
        println!("count: {count}");
    };

    println!("Exited loop with res: {res}!!")
}

// break in while loop can't return a value.
pub fn while_loop() {
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("count : {count}");
    }
}

pub fn for_loop() {
    let arr = [1, 2, 3, 4, 5, 6];

    for num in arr {
        println!("num: {num}")
    }

    // enumerate returns reference of the element.
    // therefore, we will have to use &num in declaration
    // or use num in declaration and use *num while using value
    for (index, &num) in arr.iter().enumerate() {
        if num == 4 {
            continue;
        }
        println!("num[{index}]: {num}")
    }
}
