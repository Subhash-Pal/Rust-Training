// Returns (area, circumference) of a circle given its radius
fn circle_metrics(radius: f64) -> (f64, f64) {
    let area = std::f64::consts::PI * radius * radius;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    (area, circumference) // Tuple return
}

fn main() {
    let radius = 5.0;
    let (area, circumference) = circle_metrics(radius); // Destructuring the tuple

    println!("For a circle with radius {}:", radius);
    println!("  Area = {:.2}", area);
    println!("  Circumference = {:.2}", circumference);

    // You can also access tuple fields by index if you don't destructure:
    let metrics = circle_metrics(3.0);
    println!("\nUsing index access:");
    println!("  Area = {:.2}", metrics.0);
    println!("  Circumference = {:.2}", metrics.1);
}