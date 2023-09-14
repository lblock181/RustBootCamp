
/// Using option enum
fn main() {
    let username: Option<String> = get_username(1);


    if let Some(name) = username {
        println!("{name}")
    }
}


// using option enum
fn get_username(user_id: i32) -> Option<String> {
    // gets un from db
    let query = format!("get username from users where user_id == {user_id};")
    let db_result: Result<String> = query_db(query);
    if db_result.ok() {
        println!("Connected");
    } else {
        println!("Fail");
    }
}

fn query_db(query: String) -> Result<String> {
    if query.is_empty() {
        Err(String::from("Not found"))
    } else {
        Ok(String::from("Ferris"))
    }
}