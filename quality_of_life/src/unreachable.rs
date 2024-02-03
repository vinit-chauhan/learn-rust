fn odd_or_even(num: i32) -> String {
    /*
    if we do `num % 2 == 0` it becomes bool value,
    and has only 2 possibilities [ true/false ].

    However, we are using `num % 2`` is i32 value,
    match statement expects all the possible values to be handled.
    Therefore, we need to use unreachable macro
    */

    match num % 2 {
        0 => "even".to_string(),
        1 | -1 => "odd".to_string(),
        _ => unreachable!(),
    }
}

pub fn exec() {
    for i in -10..10 {
        println!("{}: {}", i, odd_or_even(i));
    }
}
