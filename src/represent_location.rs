pub enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Anonymous => write!(f, "Anonymous"),
            Self::Known(lat, lon) => write!(f, "Known => latitude: {} longitude: {}", lat, lon)
        }
    }
}

pub fn main() {
    let address = Location::Unknown;
    println!("{}", address);
    let address = Location::Anonymous;
    println!("{}", address);
    let address = Location::Known(28.608295, -80.604177);
    println!("{}", address);
}