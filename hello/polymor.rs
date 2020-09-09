// Contoh polymorphism
//
const PI: f64 = 3.1416;

struct Circle {
    radius: f64,
}

struct Rectangle {
    height: f64,
    width: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("{}", shape.area());
}

fn print_area_generic<T: Shape>(shape: &T) {
    println!("{}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let rectangle = Rectangle {
        width: 5.0,
        height: 2.0,
    };

    print_area(&circle);
    print_area(&rectangle);

    print_area_generic(&circle);
    print_area_generic(&rectangle);
}
