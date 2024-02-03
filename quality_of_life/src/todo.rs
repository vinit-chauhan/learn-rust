struct Person {}

#[derive(Debug)]
#[allow(dead_code)]
struct MyMoves {
    name: String,
    energy_used_pr: u8,
}

trait Dance {
    fn pick_the_song(&self, name: String) -> bool;
    fn dance(&self, name: String, energy_used_pr: u8) -> MyMoves;
}

impl Dance for Person {
    fn dance(&self, name: String, energy_used_pr: u8) -> MyMoves {
        MyMoves {
            name,
            energy_used_pr,
        }
    }
    fn pick_the_song(&self, _name: String) -> bool {
        // removes compile time error for un-implemented code.
        // panics if someone uses that code.
        todo!();
    }
}

pub fn exec() {
    let person = Person {};

    println!(
        "dance move: {:?}",
        person.dance(String::from("Jump"), 10_u8)
    );
    println!(
        "dance move: {:?}",
        person.pick_the_song(String::from("some song"))
    );
}
