struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U>Point<T,U> {
    fn mixup<V,W>(self, another: Point<V, W>) -> Point<T,W> {
        return Point {
            x: self.x,
            y: another.y
        }
    }
}


fn main() {
    let p1 = Point{x:1, y:2};
    let p2 = Point{x:"a", y:"b"};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y={}", p3.x, p3.y);
}