
fn main() {
    let mut str1 = get_new_string();
    println!("Printing through str1: {}", str1);
    let mut str2 = str1;
    println!("Printing through str2: {}", str2);
    str1 = str2;
    println!("Again printing through str1: {}", str1);
    str2 = str1.clone();
    println!("Again printing through str2: {}", str2);
    println!("Printing thourgh both: {}, {}", str1, str2);
}

fn get_new_string() -> String {
    let new_string = String::from("I will master rust 🦀 🦀");
    new_string
}