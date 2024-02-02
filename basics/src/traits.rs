struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    velocity: f64,
    crew_size: u8,
}

impl Satellite {
    fn new(name: String) -> Satellite {
        Satellite {
            name,
            velocity: 1001_f64,
        }
    }
}

impl SpaceStation {
    fn new(name: String) -> SpaceStation {
        SpaceStation {
            name,
            crew_size: 10,
            velocity: 100_f64,
        }
    }
}

trait Description {
    fn desc(&self) -> String;
}

impl Description for Satellite {
    fn desc(&self) -> String {
        format!("the {} flying at {} km/s.", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn desc(&self) -> String {
        format!(
            "the {} flying at {} km/s with {} crew members.",
            self.name, self.velocity, self.crew_size
        )
    }
}

pub fn exec() {
    let s1 = Satellite::new(String::from("Hubble Telescope"));

    let ss1 = SpaceStation::new(String::from("International Space Station"));

    println!("s1: {}", s1.desc());
    println!("s1: {}", ss1.desc());
}
