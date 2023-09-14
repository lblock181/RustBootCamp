/// Hypothetical
/// main fn calls fn to get username from db

// fn main() {
//     let username: String = get_username(1);
//     println!("{username}");
// }

/// Using option enum
fn main() {
    let username: Option<String> = get_username(1);

    // match username {
    //     Some(u) => println!("{u}")
    //     None => {}
    // }

    if let Some(name) = username {
        println!("{name}")
    }
}

/// returns nothing
// fn get_username(user_id: i32) -> String {
//     // gets un from db
//     let db_result = String::from("lblock");
//     if user_id == 1 {
//         db_result
//     } else {

//     }
// }

// using option enum
fn get_username(user_id: i32) -> Option<String> {
    // gets un from db
    let db_result = String::from("lblock");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}