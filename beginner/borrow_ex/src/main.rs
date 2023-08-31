fn main() {
    let s1 = String::from("Rust");
    let mut s2 = String::from("Rust");
    let r1 = &s1;
    let r2 = &mut s2;
    print_str(&r1);
    mod_str(r2);
    println!("{r2}");
}

// fn print_string(p1:String) {
//     println!("{p1}");
// }

fn print_str(p1: &str) {
    println!("printstr {p1}");
}

fn mod_str(p1: &mut String) {
    p1.push_str(" string");
}
