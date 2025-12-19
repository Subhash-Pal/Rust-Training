/*
How Rust structs differ from C++ classes
A Point2D example with methods
An exercise: Rectangle area calculator

*/

// ===== 2D Point Struct with Methods =====
#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    // Constructor-like associated function (not a "new" by convention, but idiomatic)
    fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    // Method to compute distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Method to compute distance to another point
    //p1, p2
    //p1.distance_to(&p2);
    fn distance_to(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Method to move the point (returns new point — immutable by default)
    fn translate(&self, dx: f64, dy: f64) -> Point2D {
        Point2D::new(self.x + dx, self.y + dy)
    }
}

// ===== Rectangle Area Calculator (Exercise) =====
//#[derive(Debug)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point2D,
    bottom_right: Point2D,
}

impl Rectangle {
    // Constructor
    fn new(top_left: Point2D, bottom_right: Point2D) -> Self {
        // Optional: normalize points so that top_left is actually top-left
        let x1 = top_left.x.min(bottom_right.x);
        let x2 = top_left.x.max(bottom_right.x);
        let y1 = top_left.y.max(bottom_right.y); // higher y = top in typical screen coords
        let y2 = top_left.y.min(bottom_right.y);

        Rectangle {
            top_left: Point2D::new(x1, y1),
            bottom_right: Point2D::new(x2, y2),
        }
    }

    // Calculate width
    fn width(&self) -> f64 {
        (self.bottom_right.x - self.top_left.x).abs()
    }

    // Calculate height
    fn height(&self) -> f64 {
        (self.top_left.y - self.bottom_right.y).abs()
    }

    // Calculate area
    fn area(&self) -> f64 {
        self.width() * self.height()
    }

    // Optional: center point
    fn center(&self) -> Point2D {
        Point2D::new(
            (self.top_left.x + self.bottom_right.x) / 2.0,
            (self.top_left.y + self.bottom_right.y) / 2.0,
        )
    }
}

// ===== Main Function: Demo =====
fn main() {
    println!("=== Point2D Demo ===");
    let p1 = Point2D::new(3.0, 4.0);
    let p2 = Point2D::new(0.0, 0.0);

    println!("Point p1: {:?}", p1);
    println!("Distance from origin: {:.2}", p1.distance_from_origin());
    println!("Distance from p1 to origin: {:.2}", p1.distance_to(&p2));

    let p3 = p1.translate(1.0, -2.0);
    println!("p1 translated by (1, -2): {:?}", p3);

    println!("\n=== Rectangle Area Calculator ===");
    // Define two corners (order doesn't matter thanks to normalization)
    let rect = Rectangle::new(Point2D::new(1.0, 5.0), Point2D::new(4.0, 2.0));

    println!("Rectangle: {:?}", rect);
    println!("Width: {:.2}", rect.width());
    println!("Height: {:.2}", rect.height());
    println!("Area: {:.2}", rect.area());
    println!("Center: {:?}", rect.center());

    // Edge case: same point → zero area
    let degenerate = Rectangle::new(Point2D::new(2.0, 2.0), Point2D::new(2.0, 2.0));
    println!("\nDegenerate rectangle area: {:.2}", degenerate.area());
}

/*
Feature         |   Rust struct                     |      C++ class
Data only       |  Yes (no methods in definition)   | Can include data + methods
Methods         | Defined separately in impl blocks | Defined inside class
No inheritance  |          ✅                      |Supports inheritance
No hidden state | All fields explicit               |Can have private/mutable state
No constructors | Use fn new() pattern              |Built-in constructors
Memory layout   |  Controllable (repr(C))           | Complex (vtables, etc.)

*/