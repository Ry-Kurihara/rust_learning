trait Summable {
    type Sum;
    fn sum(&self) -> Self::Sum;
}

struct Point<T> {
    x: T,
    y: T,
}

// impl<T> Summable for Point<T> {
// }

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.5 };

    println!("Sum of integers: {}", p1.sum());
    println!("Sum of floats: {}", p2.sum());
}