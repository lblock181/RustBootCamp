struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {

    // immut ref to self
    fn calc_sales_tax(&self) -> f32 {
        self.price * 0.1
    }

    // mut ref to self
    fn set_price(&mut self, new_price: f32) {
        self.price = new_price;
    }

    // owned version of self
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        123
    }

    // associated function
    // doesn't take self
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // common associated function for contructors
    fn new(name: String, price: f32) -> Product {
        Product {
            name,
            price,
            in_stock: true
        }
    }

}


fn main() {
    let mut book: Product = Product {
        name: String::from("Book"),
        price: 5.50,
        in_stock: true
    };
    let mut book2 = Product::new(
        String::from("Rust Book"),
        25.0
    );
    let sales_tax: f32 = book.calc_sales_tax();
    println!("{sales_tax}");

    book.set_price(1.0);
    book.buy();
    // buy method takes ownership of book & cannot run/compile following line
    // book.set_price(1.5);
    println!("{}", book2.name);
}

