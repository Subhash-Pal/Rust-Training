// Supertrait: any iterator must be resettable
trait Resettable {
    fn reset(&mut self);
}

// Custom iterator trait with associated type
trait MyIterator: Resettable {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Concrete iterator struct
struct Counter {
    start: i32,
    end: i32,
    current: i32,
}

// Implement the supertrait
impl Resettable for Counter {
    fn reset(&mut self) {
        self.current = self.start;
    }
}

// Implement the custom iterator trait
impl MyIterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter {
        start: 1,
        end: 5,
        current: 1,
    };

    println!("--- First Run ---");
    while let Some(v) = counter.next() {
        println!("{}", v);
    }

    counter.reset();

    println!("--- After Reset ---");
    while let Some(v) = counter.next() {
        println!("{}", v);
    }
}
