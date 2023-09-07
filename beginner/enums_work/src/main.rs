
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
    let category = ProductCategory::Electronics;
    let mut comp = Product{
        name: String.from("Computer"),
        category: category,
        price: 50.0,
        in_stock: true
    }
    println!("Hello, world!");

    let cmd = Command::Undo;
    let cmd = Command::AddText("Hello".to_string());
    let cmd = Command::MoveCursor(2,3);

}

/// More complex version
/// think of command patterns
/// enums can carry data as well
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32,i32),
    Replace(from: String, to: String)
}

impl Command {
    fn serialize_cmd(&self) -> String {
        "dummy string".to_string()
    }
}
