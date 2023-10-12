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

fn main() {
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