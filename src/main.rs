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
    let point = Point {
        x: 3.0,
        y: 4.0,
    };

    println!("x: {:.1}", point.x());

}
