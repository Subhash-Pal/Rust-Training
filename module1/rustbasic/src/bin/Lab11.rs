/*
Rust implementation for Hour 11: Generics + Trait Bounds, covering:

Type parameters and trait bounds
A generic min/max function
An exercise: Vector Aggregator that works with any numeric type
*/

/// The lines `use std::cmp::PartialOrd;`, `use std::ops::Add;`, and `use std::iter::Sum;` are bringing
/// specific traits into scope from the standard library in Rust.
/// The lines `use std::cmp::PartialOrd;`, `use std::ops::Add;`, and `use std::iter::Sum;` are bringing
/// specific traits into scope from the standard library in Rust.
use std::cmp::PartialOrd;
use std::ops::Add;
use std::iter::Sum;

// ===== Helper trait to convert to f64 =====
/// The `pub trait ToF64` defines a trait named `ToF64` in Rust. This trait specifies a method `to_f64`
/// that converts a value of a type implementing the trait to a `f64` type. This trait allows different
/// types (like `i32`, `f64`, `u32`, `f32`, etc.) to define their own implementations for converting
/// themselves to a `f64` value. This can be useful for cases where you need to work with numeric types
/// in a generic way and need to convert them to a common type for certain operations.



/// The `pub trait ToF64` in Rust defines a trait named `ToF64` that specifies a method `to_f64` which
/// converts a value of a type implementing the trait to a `f64` type. This trait allows different types
/// (like `i32`, `f64`, `u32`, `f32`, etc.) to define their own implementations for converting
/// themselves to a `f64` value. This can be useful for cases where you need to work with numeric types
/// in a generic way and need to convert them to a common type for certain operations.
/// The `pub trait ToF64` in Rust defines a trait named `ToF64` that specifies a method `to_f64` which
/// takes a reference to `self` and returns a `f64` value. This trait allows types implementing it to
/// define their own implementations for converting themselves to a `f64` value. In the provided code,
/// implementations for `i32`, `f64`, `u32`, and `f32` are provided to convert these types to `f64`.
/// This trait is useful for cases where you need to work with different numeric types in a generic way
/// and need to convert them to a common type for certain operations.
pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}
impl ToF64 for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

// ===== Generic min/max =====
/// The function `min` returns the smaller of two values of any type that implements the `PartialOrd`
/// trait in Rust.
/// 
/// Arguments:
/// 
/// * `a`: The `min` function takes two parameters `a` and `b`, both of type `T`, where `T` is any type
/// that implements the `PartialOrd` trait. This trait allows values of type `T` to be compared for
/// ordering.
/// * `b`: The parameter `b` in the `min` function represents the second value that you want to compare
/// with the first value `a` to determine the minimum value between the two.
/// 
/// Returns:
/// 
/// The `min` function returns the minimum of the two input values `a` and `b`.
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

// ✅ Fixed: Struct with named field
#[derive(Debug)]
/// The `VectorAggregator` struct in Rust contains a field named `data` that holds a vector of elements
/// of type `T`.
/// 
/// Properties:
/// 
/// * `data`: The `data` field in the `VectorAggregator` struct is a vector that stores elements of type
/// `T`.
pub struct VectorAggregator<T> {
    data: Vec<T>, // ← field name `data` added!
}

// ✅ Fixed: `new` with named parameter
impl<T> VectorAggregator<T> {
    pub fn new(data: Vec<T>) -> Self { // ← `data: Vec<T>`, not just `Vec<T>`
        Self { data } // now `data` is in scope
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

// Aggregation methods with bounds
/// The `impl<T> VectorAggregator<T> where T: Copy + PartialOrd + Add<Output = T> + Sum + ToF64` block
/// in Rust is specifying trait bounds for the generic type `T` used in the `VectorAggregator` struct.
/// Here's what each trait bound means:
/// The `impl<T> VectorAggregator<T> where T: Copy + PartialOrd + Add<Output = T> + Sum + ToF64` block
/// in Rust is specifying trait bounds for the generic type `T` used in the `VectorAggregator` struct.
/// Here's what each bound means:
impl<T> VectorAggregator<T>
where
    T: Copy + PartialOrd + Add<Output = T> + Sum + ToF64,
{
    pub fn sum(&self) -> T {
        self.data.iter().copied().sum()
    }

    /// The `min` function in the `VectorAggregator` struct is defined as follows:
    pub fn min(&self) -> Option<T> {
        self.data.iter().copied().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(if x <= y { x } else { y }),
        })
    }

    pub fn max(&self) -> Option<T> {
        self.data.iter().copied().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(if x >= y { x } else { y }),
        })
    }

    pub fn average(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        let total: f64 = self.data.iter().map(|x| x.to_f64()).sum();
        Some(total / self.data.len() as f64)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }
}

impl<T> From<Vec<T>> for VectorAggregator<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

// ===== Main =====
fn main() {
    println!("=== Generic Min/Max ===");
    println!("min(5, 10) = {}", min(5i32, 10));
    println!("max(3.2, 7.1) = {}", max(3.2f32, 7.1));
    println!("min('x', 'a') = '{}'", min('x', 'a'));
////////////////////////////////////////////////////////////
    println!("\n=== Vector Aggregator (Integers) ===");
    let int_data = vec![10, 20, 5, 30, 15];
    let int_agg = VectorAggregator::new(int_data);
    println!("Data: {:?}", int_agg.data());
    println!("Sum: {}", int_agg.sum());
    println!("Min: {:?}", int_agg.min());
    println!("Max: {:?}", int_agg.max());
    println!("Average: {:.2}", int_agg.average().unwrap());
    println!("Count: {}", int_agg.count());

    println!("\n=== Vector Aggregator (Floats) ===");
    let float_data = vec![1.5, 2.5, 3.0, 4.5];
    let float_agg = VectorAggregator::from(float_data);
    println!("Sum: {}", float_agg.sum());
    println!("Average: {:.2}", float_agg.average().unwrap());

    println!("\n=== Empty Vector Handling ===");
    let empty_agg: VectorAggregator<i32> = VectorAggregator::new(vec![]);
    println!("Min of empty: {:?}", empty_agg.min());
    println!("Average of empty: {:?}", empty_agg.average());
}