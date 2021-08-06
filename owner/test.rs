fn main() {

    let s = String::from("hello");

    take_ownership(s);

    println!("Is Playes : {} ?" , s);

    let x = 5;

    makes_copy(5);

}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}