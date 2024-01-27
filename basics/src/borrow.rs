pub fn exec() {
    _use_case_2();
}
// only borrow, no changes to the value.
fn _func_borrow(str: &String) -> usize {
    // The ownership is not with `_func_borrow`. var_s has the ownership of string "Hello "
    println!("inside str: String = {str}");

    let length: usize = str.len();
    length
}

// mutable borrow, we can change value of the variable.
fn _func_borrow_mut(str: &mut String) -> usize {
    // The ownership is not with `_func_borrow_mut`. var_s has the ownership of string "Hello "
    // `_func_borrow_mut` has mutable reference for var_s
    str.push_str("World...");

    let new_ref = &str;

    _func_borrow(new_ref);

    let length: usize = str.len();
    length
}

fn _use_case_1() {
    let mut var_s: String = String::from("Hello ");
    println!("var_s: String = {var_s}");

    /*
     * We are just borrowing the reference,
     *
     */
    let len = _func_borrow(&var_s);
    println!("var_s: String = {var_s} with size {len} chars");

    /*
     * We are just borrowing the reference,
     * With intent to make change to the value
     *
     */
    let ref1 = &mut var_s;
    // You can't create more than 1 mutable reference of a variable in a given scope.
    // let ref2 = &mut var_s;
    let len = _func_borrow_mut(ref1);
    println!("var_s: String = {var_s} with size {len} chars");
}

fn _borrow_ref_return() -> String {
    // -> &String won't work, as the value will be discarded once the function is over
    // Rust will give you a compiler error.
    let new_str = String::from("new string");

    new_str
}

fn _use_case_2() {
    let _str = String::from("Hello World.");
    println!("new str: {}", _borrow_ref_return());
}
