fn main() {

    let guess: u32 = 1;

    println!("{}", guess);

    println!("Add numbers {}", add_numbers(guess));

}

fn add_numbers(x: u32) -> u32 {
    x + 1
}