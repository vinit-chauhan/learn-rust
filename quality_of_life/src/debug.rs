// create series, 5, 8, 11, 14, 17...
fn recursive_fn(n: u64) -> u64 {
    match n {
        0 => panic!("Why do you want 0?"),
        1 => 5_u64,
        _ => recursive_fn(n - 1) + 3,
    }
}

pub fn exec() {
    dbg!(
        recursive_fn(1),
        recursive_fn(2),
        recursive_fn(3),
        recursive_fn(4),
    );
}
