pub fn find_average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let f = true;
    let v: u32 = f as u32;
    let average = (a as f64 + b + c as f64)/3.0_f64;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}