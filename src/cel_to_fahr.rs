pub fn main() {
    let cel_temp = 23.0;
    let fahr_temp = convert_cel_to_fahr(cel_temp);

    assert_eq!(fahr_temp, 73.4);
    println!("Test passed!");
}

pub fn convert_cel_to_fahr(cel_value: f64) -> f64 {
    (1.8 * cel_value) + 32.0_f64
}