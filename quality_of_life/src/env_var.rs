pub fn exec() {
    let env_home = env!("HOME");
    dbg!(env_home);

    let user = match option_env!("USERNAME") {
        Some(val) => val,
        None => panic!("Can't find Environment variable."),
    };

    dbg!(user);
}
