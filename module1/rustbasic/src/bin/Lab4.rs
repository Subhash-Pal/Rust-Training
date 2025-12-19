use std::io;

// Classifies a numeric score into a letter grade
fn classify_grade(score: f64) -> Option<char> {
    if score < 0.0 || score > 100.0 {
        None // Invalid input
    } else if score >= 90.0 {
        Some('A')
    } else if score >= 80.0 {
        Some('B')
    } else if score >= 70.0 {
        Some('C')
    } else if score >= 60.0 {
        Some('D')
    } else {
        Some('F')
    }
}

// Alternative version using match (though ranges require if guards or external crates)
// For clarity and simplicity, if/else is idiomatic here.

fn main() {
    println!("🎓 Grade Classifier");
    println!("Enter a score (0–100):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let score: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("❌ Error: Please enter a valid number.");
            return;
        }
    };

    match classify_grade(score) {
        Some(grade) => println!("✅ Your grade is: {}", grade),
        None => eprintln!("❌ Score must be between 0 and 100."),
    }
}