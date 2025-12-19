use std::io::{self};

fn main() {
    let mut name = String::new();//immutable variable
    let _x : i32 = 5; //mutable variable

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Enter your age: ");
    let mut age_input = String::new();
    io::stdin()
      .read_line(&mut age_input)
     .expect("Failed to read line");

    let age: i32 = age_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Hello, {}! You are {} years old.", name, age);

    let next_year_age = age + 1;
    println!("Next year, you will be {}.", next_year_age);
}
