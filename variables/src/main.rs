use std::fmt;

#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.username, self.email, self.sign_in_count, self.active)
    }
}

fn build_user(email: String, username: String) -> User {
    User { username: username, email: email, sign_in_count: 1, active: true }
}

fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The constatn of MAX_POINTS is : {}", MAX_POINTS);
    println!("Build User : {}", build_user(String::from("roach@naver.com"), String::from("username")))
}
