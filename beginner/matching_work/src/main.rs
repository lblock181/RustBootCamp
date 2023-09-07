
enum ProductCategory {
    Books,
    Clothing,
    Electronics
}

struct Product {
    name: String,
    // category: String, Don't use this instead use enum
    category: ProductCategory,
    price: f32,
    in_stock: bool
}

fn main() {
    // let age: i32 = 30;

    // match age {
    //     1 => println!("one year old"),
    //     30 => println!("Hello 30"),
    //     13..=19 => println!("teen"),
    //     x => println!("{}", x)
    // }

    let cmdr = Command::Redo;
    let cmdu = Command::Undo;
    let cmda = Command::AddText("Hello".to_string());
    let cmdm = Command::MoveCursor(2,3);
    
}

/// More complex version
/// think of command patterns
/// enums can carry data as well
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32,i32),
    Replace{from: String, to: String}
}

impl Command {
    fn serialize_cmd(&self) -> String {
        match self {
            Command:Undo => String::from("Undo"),
            Command::Redo => String::from("Redo"),
            Command::AddText(x) => String::from("Adding {x}"),
            Command::MoveCursor(x,y) => String::from("Moving {x} to {y}"),
            Command::Replace{f,t} => String::from("Replace {f} to {t}")
        }
    }
}
