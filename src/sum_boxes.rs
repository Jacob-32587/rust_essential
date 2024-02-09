use std::ops::Add;

pub fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);
    
    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
    
    println!("Tests passed!");
}

pub fn sum_boxes<T: Add<Output = T>>(l: Box<T>, r: Box<T>) -> Box<T> {
    Box::new(*l + *r)
}