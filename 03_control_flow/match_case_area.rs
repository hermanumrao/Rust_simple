enum Shape {
    Circle,
    Square,
    Rectangle,
}

struct Circle {
    radius: f64,
}

struct Square {
    side_length: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

fn process_shape(shape: Shape) {
    match shape {
        Shape::Circle => {
            let circle = Circle { radius: 5.0 };
            println!("Area of Circle: {}", circle.radius * circle.radius * std::f64::consts::PI);
        }
        Shape::Square => {
            let square = Square { side_length: 4.0 };
            println!("Area of Square: {}", square.side_length * square.side_length);
        }
        Shape::Rectangle => {
            let rectangle = Rectangle { width: 3.0, height: 6.0 };
            println!("Area of Rectangle: {}", rectangle.width * rectangle.height);
        }
    }
}

fn main() {
    process_shape(Shape::Circle);
    process_shape(Shape::Square);
    process_shape(Shape::Rectangle);
}

