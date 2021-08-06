fn main() {

    let mut s1 = String::from("hello");

    let len = caculate_length(&s1);

    can_write_reference(&mut s1);

    println!("The length of {} is {}.", s1, len);
}

fn caculate_length(s: &String) -> usize {
    return s.len();
}

fn can_write_reference(s: &mut String) -> () {
    s.push_str(", Good")
}