mod supertraits;
mod trait_obj;
mod trait_trait_bounds;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p1 = Point{x:1,y:2};
    let p2 = Point{x:1,y:2};

    println!("{:?}", p1);
    println!("{}", p1 == p2);
    println!()
}