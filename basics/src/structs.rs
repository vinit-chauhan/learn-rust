/*
    Struct stays in Stack but it can use properties which lives in Heap.

    - The following struct Shuttle when instantiated,
    it will be stored in stack, with `crew_size`, `propellent`, and
    properties of String(ptr, len, cap) being in stack.
    However, the actual string represented by ptr is stored in the heap.

*/

/*
Memory Map:

Stack: 40 bytes
    Here, align = 8, which means memory will grow my 8 bytes,
        - 8-bit types   e.g., u8, i8             :: align -> 1-byte
        - 16-bit types  e.g., u16, i16           :: align -> 2-byte
        - 32-bit types  e.g., u32, i32, f32      :: align -> 4-byte
        - 64-bit types  e.g., u64, i64, f64      :: align -> 8-byte
    Therefore, to allocate 33 bytes, struct will reserve 40 bytes.

    -----------------------------------------
    | name:  : 24 bytes                     |
    |   ---------------                     |
    |   | ptr: 0x1200 | : 8 bytes           |
    |   | len: 4      | : 8 bytes           |
    |   | cap: 4      | : 8 bytes           |
    |   ---------------                     |
    |                                       |
    | crew_size: u8     : 1 byte            |
    | propellent: f64   : 8 bytes           |
    -----------------------------------------
Heap:
    0x1200-0x1204 (String):
    -------------------------
    | 'H' | 'i' | 'i' | '!' |
    -------------------------

*/

// derive(Debug) : Add ability to print in debug mode.
// derive(Clone) : Add ability to clone properties.
#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellent: f64,
}

/*
Struct methods:
    These can be identified by &self reference in the method parameters
    These are used when we need to perform action based on the state of the instance

Use:
    To use the methods of structure,
    use . operator on the instance of the struct

Example:
    string_instance.trim()

 */
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, litres: f64) {
        self.propellent += litres;
    }
}

/*
Struct Functions:
    These functions are not relied on instances of the struct,
    but these can be considered as the functional properties of a struct
    Struct Functions work similar as the static methods in OOP.

Usage:
    To use the functions of the struct,
    use :: operator on the struct type.

Example:
    String::new()

*/
impl Shuttle {
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 0,
            propellent: 0.0,
        }
    }
}

/*
Tuple Struct:
    This type of struct is required when the order of values in struct
    is either a convention and is known by all.
    However, It's a good practice to add comment stating the format of tuple Struct.

Example:
    Colors in pixel are defined by u8 values in (R,G,B) order.
    IP Address is represented by 4 octets: IP(o1, o2, o3, o4)

*/

struct Color(u8, u8, u8); // Color( Red value, Green value, Blue value)

pub fn exec() {
    let mut vehicle: Shuttle = Shuttle {
        name: String::from("A"),
        crew_size: 8,
        propellent: 100.00,
    };

    let _vehicle2: Shuttle = Shuttle {
        name: String::from("B"),
        // struct update copies remaining values (in Stack) of vehicle here.
        ..vehicle
    };

    let vehicle3: Shuttle = Shuttle {
        crew_size: 20,
        /*
           The following line of code will result in an Error.
            because, Ownership of string will be transferred to vehicle3
            after use of struct update operation.

            Therefore, We can,t access vehicle.name later on in the code.
        */
        // ..vehicle
        ..vehicle.clone()
    };

    let _vehicle4 = Shuttle::new("D");

    println!("Vehicle 1: {vehicle:?}");
    println!("Vehicle 3: {vehicle3:?}");

    vehicle.name = String::from("A'");

    vehicle.add_fuel(10000.00);

    println!(
        "Vehicle 1 is now {} with {} crew members and it has {:.2} L of fuel left.\n",
        vehicle.get_name(),
        vehicle.crew_size,
        vehicle.propellent
    );

    let pixel: Color = Color(255, 0, 0);
    println!("red: {}, green: {}, blue: {}", pixel.0, pixel.1, pixel.2);
}
