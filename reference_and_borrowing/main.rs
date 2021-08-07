fn main() {

    let s1 = String::from("hello");

    let len = caculate_length(s1);

    println!("The length of {} is {}.", s1, len);
}

fn caculate_length(s: String) -> usize {
    return s.len();
}