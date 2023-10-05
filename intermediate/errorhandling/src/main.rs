use std::fs::{File, read_to_string};
use std::io::{self, Read};

/// propegating different error types
/// one way -> Result<T, Box<dyn error::Error>>
///     use when doesn't matter what error is returned (ie callers don't need to handle errors differently)
/// other way -> create enum CustomError {ERRORTYPE1, ERRORTYPE2}
///     return CustomError enum from fn & ensure map_err(|e| CustomError::)?


fn main() {
    let contents = read_file("something.txt"); 
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    // ? allows for implicit unwrap
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    read_to_string(filename).map(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
    // below works if returning Option<String>
    // read_to_string(filename).ok().and_then(|s| {
    //     s.lines().next().map(|s| s.to_owned())
    // })
}
struct User {
    firstname: String,
    lastname: String
}

fn get_initials(user: User) -> Option<String> {
    let fi = user.firstname.chars().next()?;
    let li = user.lastname.chars().next()?;
    Some(format!("{fi}{li}"))
}

// fn main() {
//     // Recoverable errors
//         // using the Result<T,E> enum

//     let file = File::open("somthign.txt");
//     // let file = File::open("somthign.txt").unwrap();
//     // let file = File::open("somthign.txt").expect("Failed to open file");
//     let file = match file {
//         Ok(file) => file,
//         Err(file) => {
//             panic!("File not found {}", file)
//         }
//     };
// }

// fn get_uid(username: &str) -> Result<i32, String> {
//     if username.is_empty() {
//         Err(String::from("No username provided"))
//     } else {
//         Ok(5)
//     }
// }
