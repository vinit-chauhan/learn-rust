pub fn exec() {
    // Ownership is used for variables in Heap.
    // it does not apply on Stack. ( _use_case_3 () )

    /*

    - Rule of Thumb:
        When we pass a reference of a variable,
        from that moment of time the owner ship of the object in memory is transferred.
        and the variable is useless.

    */

    _use_case_6();
}

fn _use_case_1() {
    let outer_var: String;
    {
        let mut inner_var = String::from("Hello");
        println!("inner_var: {inner_var}");

        inner_var.insert(2, 'L');
        inner_var.remove(3);

        outer_var = inner_var;
        // inner_var does not refer to "Hello" after its been owned by outer_var.
        // println!("inner_var: {inner_var}"); // this will throw an error.

        /*
           before:
               "Hello" <- inner_var
           after
               "Hello" <- outer_var

               If It was C/C++, There would be two pointers pointing to
               String "Hello".

               However, Rust compilers has 2 Rules for ownership,
               1. Every value can have only 1 variable referencing it at a time. ( Only 1 Owner )
               2. When owning variable goes out of scope, value is dropped.

        */
    }

    println!("outer_var: {outer_var}");
}

fn _use_case_2() {
    let outer_var: String;
    {
        let inner_var = String::from("Hello");
        println!("inner_var: {inner_var}");
        outer_var = inner_var.clone(); // creates a new copy of string.
        println!("inner_var: {inner_var}");
    }

    println!("outer_var: {outer_var}");
}

fn _use_case_3() {
    let outer_var: i32;
    {
        let mut inner_var = 32;
        println!("inner_var: {inner_var}");
        // this will not give reference as
        // the data lives on stack.
        outer_var = inner_var;

        inner_var += 10; // this will work.
        println!("inner_var: {inner_var}");
    }

    println!("outer_var: {outer_var}");
}

fn _use_case_4() {
    fn func_i(mut var: i32) {
        var += 1;
        println!("inside var: i32 = {var}");
    }
    fn func_s(mut var: String) {
        let suffix = String::from("value");
        var.insert_str(var.len(), &suffix);
        println!("inside var: String = {var}");
    }

    let var_i = 123;
    let var_s = String::from("Hello ");
    let var_s2 = String::from("Hello2 ");

    func_i(var_i);
    println!("outside: var: i32 = {var_i}"); // this will work

    println!("outside: var: String = {var_s}"); // This will work
    func_s(var_s); // passed as ref. Hence, Ownership is gone.

    // println!("outside: var: String = {var_s}"); // This won't work

    println!("outside: var2: String = {var_s2}"); // This will work
    func_s(var_s2.clone()); // passed as value. Hence, Ownership is with var_s2.

    // This will work, However, changes made by function are gone,
    println!("outside: var2: String = {var_s2}");
}

fn _use_case_5() {
    fn func(mut str: String) -> String {
        let suffix = String::from("value");
        str.insert_str(str.len(), &suffix);
        println!("inside str: String = {str}");

        str
    }

    let var_s = String::from("Hello ");
    println!("outside: var_s: String = {var_s}"); // This will work

    /*
    Ownership:
        1. var_s : Has the ownership when initialized.

        2. func : Ownership transferred when the var_s is passed to the function.

        3. var_s : Used Variable shadowing to get ownership back again.
        ( because we are returning reference to the same string at the end of the function)
     */
    let var_s = func(var_s); // passed as value. Hence, Ownership is with var_s.

    // This will work, var_s will have the latest value.
    println!("outside: var_s: String = {var_s}");
}

fn _use_case_6() {
    let mut x: i8 = 1;
    x += 1;

    {
        x += 3; // uses the value from parent scope.
    }

    x += {
        let mut x = 0; // create a new local value.
        x += 3;
        x // returns local x, which is added to parent x;
    } as i8;

    x += 1;

    println!("x: {x}");
}
