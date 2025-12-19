// Recursive linked list (impossible with plain enums due to infinite size)
/*enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
}
*/

// Trait objects – storing different types in one collection
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle { fn draw(&self) { println!("Circle"); } }
impl Drawable for Square { fn draw(&self) { println!("Square"); } }

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    for shape in shapes { shape.draw(); }
}
    