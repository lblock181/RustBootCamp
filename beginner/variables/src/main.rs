fn main() {
    // creation
    let a: i32 = 5;

    // mutability
    let mut b: i32 = 6;
    b = 10;

    // shadowing

    let c: i32 = 10;
    let c = 20;

    println!("{}", c);

    // scope 
    let d = 40;
    {
        let d = 50;
        println!("inner scope {}",d);
    }
    println!("outer scope {}",d);
}
