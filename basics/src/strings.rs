pub fn exec() {
    let mut message: String = String::from("Some String Literal.");

    message.push_str(" new str.");

    println!("msg: {message}");
}
