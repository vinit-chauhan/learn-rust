pub fn exec() {
    let mut people: Vec<String> = Vec::new();

    people.push(String::from("Vinit"));
    people.push(String::from("Sam"));
    people.push(String::from("Matt"));
    people.push(String::from("Hanna"));

    println!("vec: {people:?}");

    /*
       If the 2nd index doesn't exist,
       It will panic in runtime
    */
    let third: &String = &people[2];
    println!("3rd person: {third}",);

    // therefore, It's a best practice to use .get method.
    let third: &String = people.get(2).unwrap();
    println!("3rd person: {third}",);

    // removes the last element from the vector
    let last: String = match people.pop() {
        Some(person) => person,
        None => String::from("None found"),
    };
    println!("last person: {last}",);

    println!("vec: {people:?}");

    // pre-initialized vector.
    let people: Vec<String> = vec![
        String::from("Vinit"),
        String::from("Hanna"),
        String::from("Matt"),
    ];
    println!("vec: {people:?}");
}
