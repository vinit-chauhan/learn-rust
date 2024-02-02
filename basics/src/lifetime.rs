/*
Lifetime:
    Explicitly define generic lifetime for parameter

Naming Convention:
    - Must begin with '
    - Ideally a single lowercase letter.


*/

fn func_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn func_2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

pub fn exec() {
    let mut res: &str;
    let x = String::from("asdf");
    let y = String::from("asd");
    {
        // let y = String::from("asd");
        /*
           Throw error, because lifetime of x != lifetime of y.
           The function declaration defines that lifetime of x, y and return value is same 'a
        */
        res = func_1(&x, &y);
    }
    println!("res: {res}");

    {
        let y = String::from("asd");
        /*
           Does not throw error because we are not asserting lifetime of x == lifetime of y.
           The function declaration defines that lifetime of x and return value is same 'a
           Which is the case here in func_2

        */
        res = func_2(&x, &y);
    }
    println!("res: {res}");
}
