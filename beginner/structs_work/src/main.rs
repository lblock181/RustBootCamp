struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

fn main() {
    let book: Product = Product {
        name: String::from("Book"),
        price: 5.50,
        in_stock: true
    };
    let sales_tax: f32 = calc_sales_tax(&book);
    println!("{sales_tax}");
}


fn calc_sales_tax(product: &Product) -> f32 {
    product.price * 0.1
}