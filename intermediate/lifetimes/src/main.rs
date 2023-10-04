// Rc & RefCell
use std::rc::Rc;
use std::cell::RefCell;
struct Database {
    max_conn: u32,
}

struct Auth {
    db: Rc<RefCell<Database>>,
}

struct Content {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database{max_conn:100}));
    let a = Auth{db:Rc::clone(&db)};
    let c = Content{db:Rc::clone(&db)};
    db.borrow_mut().max_conn = 200;
}


// Box smart pointers
// trait UIComponent {
//     fn render(&self) {
//         println!("rendering");
//     }
// }

// struct Button { 
//     text: String,
// }

// impl UIComponent for Button {}

// fn main() {

//     let btn = Button{
//         text: "button".to_owned(),
//     };
    
//     // creates smart pointer to heap allocated resource
//     // avoids needing to copy large amounts of data
//         // this happens when reassigning ownership to new var (see lines below)
//         // all of btn copied to btn_c on stack whereas just pointer copied to btn_d
//     let btn_a: Box<Button> = Box::new(Button{
//         text: "button".to_owned(),
//     });

//     let btn_c = btn;
//     let btn_d = btn_a;


//     let components: Vec<Box<dyn UIComponent>> = vec![
//         Box::new(btn_c),
//         btn_d,
//     ];
// }





// fn main() {
//     let s = String::from("Hello");
//     // let r;
//     {
//         let s1 = String::from("Hello");
//         // r = &s1;
//     }
//     println!("s {s}");
//     // println!("r {r}");
//     // println!("s {s1}"); this doesn't work as s1 goes out of scope
//     // this is the same if s was assigned to s1 as s1 takes ownership of s
    
//     // cannot do on line 6 either since s1 doesn't live long enough (dropped after brackets end)

//     // below works because r1 scope doesn't go beyond the println 
    
//     let mut s1 = String::from("world");
//     let r1 = &s1;
//     println!("{r1}");
//     let r2 = &mut s1;
//     r2.push_str("!");

//     let p1 = String::from("Player1");
//     let p2 = String::from("Player2");
//     let res = first_turn(p1.as_str(), p2.as_str());
//     println!("Player going first {}", res);

// }

// // generic lifetime annotation
// // done by adding <'a> after fn name & then add it to each var
// fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
//     // below works because string slice has default static lifetime & is compiled into the binary
//     // let s: &'static str = "blah";
    
//     if rand::random() {
//         p1
//     } else {
//         p2
//     }
// }