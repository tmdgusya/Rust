struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let integer = Point {x: 5, y: 10};
    println!("X is {}", integer.x); 
    let float = Point {x:1.0, y: 4.0};
}

fn largest<T> (list: &[T]) -> T {
    let mut largest = list[0];

    // borrow reference
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }       
    }

    largest
}
