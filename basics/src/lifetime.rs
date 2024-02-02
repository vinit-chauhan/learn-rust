/*
Lifetime:
    Explicitly define generic lifetime for parameter

Naming Convention:
    - Must begin with '
    - Ideally a single lowercase letter.

Lifetime Elision Rule:
    1. Each input parameter that is reference (Only referenced parameter) is assigned its own lifetime
        - fn func<'a> (x: i32, y: &'a str) -> &str
        - fn func<'a, 'b> (x: &'a str, y: &'b str) -> &str
    2. If there's only one input lifetime, assign it to all output lifetime.
        - fn func<'a> (x: i32, y: &'a str) -> &'a str <- will be defined implicitly
        - fn func<'a, 'b> (x: &'a str, y: &'b str) -> &str <- Can't be determined implicitly need to define lifetime for output parameter,
    3. if there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes.
        - fn func<'a, 'b> (&'a self, y: &'b str) -> &'a str

Static:
    - Indicates that the reference will eb available through out the life cycle of the program.
    ex. let x:&'static str = "Hey!!";

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

#[allow(dead_code)]
struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting msg: {msg}");
        // implicitly: (&'a self, msg: &'b str) -> &'a str
        // self.name // so this line works without annotating lifetime in method.

        // we need to explicitly define (&'a self, msg: &'b str) -> &'b str
        // in order of this to work
        msg
    }
}

pub fn exec() {
    let mut res: &str;
    let s: &'static str;
    let x = String::from("asdf");
    let y = String::from("asd");
    {
        // let y = String::from("asd");
        /*
           Throw error, because lifetime of x != lifetime of y.
           The function declaration defines that lifetime of x, y and return value is same 'a
        */
        res = func_1(&x, &y);

        s = "Hello.";
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

    let shuttle_1 = Shuttle { name: "Endeavour" };

    let sender = shuttle_1.send("Hello...");
    println!("sender: {sender}");

    println!("s: {}", s);
}
