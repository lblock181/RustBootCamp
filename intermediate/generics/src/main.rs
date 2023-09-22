#[warn(dead_code, unused_variables)]

struct BrowserCommand<T> {
    name: String,
    payload: T
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self{
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1: BrowserCommand<String> = BrowserCommand ::new(
        "navigate".to_owned(), 
        "https://rubberduckysolutions.io".to_owned()
    );
    let cmd2: BrowserCommand<i32> = BrowserCommand::new("zoom".to_owned(), 200);
    
    // let cmd1: BrowserCommand<String> = BrowserCommand { 
    //     name: "navigate".to_owned(), 
    //     payload: "https://rubberduckysolutions.io".to_owned()
    // };
    // let cmd2: BrowserCommand<i32> = BrowserCommand { 
    //     name: "zoom".to_owned(), 
    //     payload: 200
    // };

}

// Doesn't affect runtime performace as Rust builds fn for each T that is found to be passed in
fn serialize_payload<T>(payload: T) -> String {
    // jsonify payload
    "placeholder".to_owned()
}

