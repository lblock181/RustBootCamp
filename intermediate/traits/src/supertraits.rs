
// traits can "inherit" from other traits
// can add additional super traits by adding a + and the trait name
// helpful when defining traits that rely on other traits

pub trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "black".to_owned()
    }
}

pub trait Paint {
    fn paint(&self, color: String) {
        println!("painting {}", color);
    }
}


fn main() {
    
}