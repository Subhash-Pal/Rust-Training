use std::io;

// Function that returns a tuple (celsius, fahrenheit)
fn convert_temp(value: f64, unit: char) -> Option<(f64, f64)> {
    match unit {
        'C' | 'c' => {
            let fahrenheit = (value * 9.0 / 5.0) + 32.0;
            Some((value, fahrenheit))
        }
        'F' | 'f' => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            Some((celsius, value))
        }
        _ => None,
    }
}

fn main() {
    println!("🌡️ Temperature Converter (C ↔ F)");
    println!("Enter temperature value:");

    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let temp: f64 = match input_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number!");
            return;
        }
    };

    println!("Enter unit (C or F):");
    let mut input_unit = String::new();
    io::stdin()
        .read_line(&mut input_unit)
        .expect("Failed to read line");

    let unit_char = match input_unit.trim().chars().next() {
        Some(c) => c,
        None => {
            eprintln!("No unit provided!");
            return;
        }
    };

    match convert_temp(temp, unit_char) {
        Some((celsius, fahrenheit)) => {
            println!("==> {:.2}°C = {:.2}°F", celsius, fahrenheit);
        }
        None => {
            eprintln!("Invalid unit! Please use 'C' or 'F'.");
        }
    }
    
    // Bonus: Demonstrate tuple usage explicitly
    let demo = convert_temp(0.0, 'C').unwrap();
    println!("\n💡 Demo: 0°C is ({:.1}°C, {:.1}°F)", demo.0, demo.1);

    // Bonus: Array of sample temps (compound type)
    let samples: [(f64, char); 3] = [(0.0, 'C'), (32.0, 'F'), (100.0, 'C')];
    println!("\n🔍 Sample conversions:");
    for &(val, unit) in samples.iter() {
        if let Some((c, f)) = convert_term_safely(val, unit) {
            println!("  {:.0}°{} → {:.1}°C / {:.1}°F", val, unit, c, f);
        }
    }
}

// Helper for clean sample printing
fn convert_term_safely(value: f64, unit: char) -> Option<(f64, f64)> {
    convert_temp(value, unit)
}


