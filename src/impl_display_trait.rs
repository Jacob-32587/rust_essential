use core::fmt;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n\tname: {}\n\tvelocity: {}\n}}", self.name, self.velocity)
    }
}

pub fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
