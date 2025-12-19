/*
🛠️ Hands-On Exercise: Safe Wrapper Around Vec
✅ Task

Implement a wrapper struct that ensures Vec access is always safe.

Starter Code
*/
struct SafeVec {
    data: Vec<i32>,
}

impl SafeVec {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}

fn main() {
    let mut s = SafeVec::new();
    s.push(10);
    s.push(20);
    s.push(30);
    s.push(40);

    match s.get(5) {
        Some(val) => println!("Value: {}", val),
        None => println!("Index out of range"),
    }
}
