// src/bin/lab19a.rs
// Lab 19A – Safe Division with Result<T, E>

fn main() {
    println!("=== Lab 19A – Safe Division with Result<T, E> ===\n");

    let numbers = [10.0, 5.0, 2.0, 0.0, -4.0, 8.0];

    for &divisor in &numbers {
        let dividend = 100.0;
        let result = safe_divide(dividend, divisor);

        println!("100.0 / {:>5} = {:?}", divisor, result);

        // Step-by-step handling of the Result
        match result {
            Ok(value) => println!("   → Success! Result = {}", value),
            Err(error) => println!("   → Error: {}", error),
        }
        println!();
    }

    // Bonus: Using ? operator in another function
    println!("Bonus: Using ? operator for cleaner code");
    let final_result = compute_average(&[100.0, 50.0, 25.0]);
    match final_result {
        Ok(avg) => println!("Average = {}", avg),
        Err(e) => println!("Failed to compute average: {}", e),
    }
}

// Core function: Returns Result instead of panicking or returning magic values
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Division by zero is not allowed!".to_string())
    } else if divisor.is_nan() || dividend.is_nan() {
        Err("Cannot divide with NaN values".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

// Example of propagating errors with ?
fn compute_average(values: &[f64]) -> Result<f64, String> {
    if values.is_empty() {
        return Err("Cannot compute average of empty list".to_string());
    }

    let mut sum = 0.0;
    for &v in values {
        sum += v;
    }

    let avg = sum / values.len() as f64;
    // Simulate calling safe_divide inside
    safe_divide(sum, values.len() as f64).map(|_| avg)
}