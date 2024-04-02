use area::Area;
use circle::Circle;
use square::Square;
use triangle::Triangle;

mod area;
mod circle;
mod triangle;
mod square;

fn main() {
    let circle = Circle::new(1.0);
    let square = Square::new(3.0);
    let triangle = Triangle::new(3.0, 5.0);

    print_area(&circle);
    print_area(&square);
    print_area(&triangle);
}

/// 打印图形面积
fn print_area<T: Area>(shape: &T) {
    println!("The area is {}", shape.area());
}
