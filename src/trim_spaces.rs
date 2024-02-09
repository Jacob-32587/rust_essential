pub fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

pub fn trim_spaces(val: &str) -> &str {
    let mut start_index = 0;
    let mut end_index = val.len();
    for c in val.chars() {
        if c != ' ' {
            break;
        }
        start_index += 1;
    }
    if start_index == end_index {
        return "";
    }
    for c in val.chars().rev() {
        if c != ' ' {
            break;
        }
        end_index -= 1;
    }

    &val[start_index..end_index]
}