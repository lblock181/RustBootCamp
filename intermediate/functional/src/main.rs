struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn validate_crendentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

// Function that takes closure as arg
// see the extra steps for generics
fn get_default_creds<T>(f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials { username: "user".to_owned(), password: "pass".to_owned(), validator: f  }
}

// function that returns closure
// static dispatch trait
fn get_password_validator(min_len: usize) -> impl Fn(&str, &str) -> bool {
    move |_: &str, password: &str | !password.len() >= min_len
}

// function that returns closure
// dynamic dispatch of trait object
fn get_password_validator_special(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str | {
            !password.len() >= min_len && 
            password.contains(['*','!'])
        })
    } else {
        Box::new(move |_: &str, password: &str | !password.len() >= min_len)
    }
}


// fn that takes two closures
// ex: two validators to validate item of type V
// see main with gte & lte closures
fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}

fn are_both_true_fn<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    f1(item) && f2(item)
}

fn less_than(x: &i32) -> bool {
    *x < 20
}


// ITERATORS //////////////////

// default Rust Iterator trait

// trait Iterator {
//     // associated type (like generic but more specified)
//     type Item;

//     // this is the required impl for item/struct/etc that impls the iterator trait
//     fn next(&mut self) -> Option<Self::Item>;
// }

// trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

struct MyStruct {}

// use below if you want to restrict the usage of next on a struct
// otherwise use generics you want next to return differenly based on type
impl Iterator for MyStruct {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> { None }
}

struct Person {
    first: String,
    last: String,
    occupation: String
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator {
            vals: vec![
                self.first,
                self.last,
                self.occupation
            ]
        }
    }
}

struct PersonIterator {
    vals: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vals.is_empty() {
            return None;
        }
        Some(self.vals.remove(0))
    }
}

////////// Combinators ///////////////////

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32
}

fn main() {
    #![allow(unused)]

    let students = vec![
        "L 3.1",
        "T 2.0",
        "G 0.1",
        "H 4.0",
        "E 3.0",
    ];

    
    // Clunky way to parse out the students based on the criteria
    // let mut gs: Vec<Student> = vec![];
    // for s in students {
    //     let mut s = s.split(" ");
    //     let n = s.next();
    //     let g = s.next();

    //     if let (Some(n), Some(g)) = (n, g) {
    //         let n = n.to_owned();
    //         let g = g.parse::<f32>();
    //         if let Ok(g) = g {
    //             if g >= 3.5 {
    //                 gs.push(Student { name: n, gpa: g });
    //             }
    //         }
    //     }
    // }

    // using iterators & combinators

    // let gs: Vec<Student> = students.iter()
    //     .map(|s| {
    //         let mut s = s.split(' ');
    //         let n = s.next()?.to_owned();
    //         let g = s.next()?.parse::<f32>().ok()?;
    //         Some(Student {name:n, gpa:g});
    //     })
    //     .flatten()
    //     .filter(|s| s.gpa >= 3.5)
    //     .collect();

    let good_students: Vec<Student> = students.iter()
        .filter_map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;
            if gpa < 3.5 {return None}
            Some(Student{name, gpa})
        })
        .collect();


    let p = Person {
        first: "L".to_owned(),
        last: "B".to_owned(),
        occupation: "Dev".to_owned()
    };

    // let mut i = p.into_iter();
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());

    for item in p {
        println!("{item}");
    }

    // store validator as closure
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    };
    let creds = Credentials {
        username: "".to_owned(),
        password: "".to_owned(),
        validator
    };
    println!("{}", validate_crendentials(&creds.username, &creds.password));
    println!("{}", validator(&creds.username, &creds.password));
    println!("{}", creds.is_valid());
    let defaults = get_default_creds(validator);

    let ten = 10;
    // below closure only works with are_both_true because var ten is being captured by closure
    // let gte = |x:&i32| ten > 10;
    let gte = |x: &i32| *x > 10;
    let lte = |x: &i32 | *x < 20;
    are_both_true(gte, lte, &15);

    // below is passing fn pointer to fn instead of closure
    are_both_true_fn(gte, less_than, &15);
}