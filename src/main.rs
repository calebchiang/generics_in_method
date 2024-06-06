struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 { // Method is only available for instances of Point that are type f32
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let point = Point {
        x: 3.0,
        y: 4.0,
    };

    println!("x: {:.1}", point.x());

}
