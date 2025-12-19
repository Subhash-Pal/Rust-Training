// src/bin/lab18c.rs
// Lab 18C – Interior Mutability + Split Borrowing (SAFE & CORRECT)

use std::cell::RefCell;

#[derive(Debug)]
struct GameWorld {
    score: RefCell<u32>,
    lives: RefCell<u32>,
    level: RefCell<String>,
    enemies: RefCell<Vec<String>>,
}

fn main() {
    println!("=== Lab 18C – Interior Mutability + Split Borrowing (Safe) ===\n");

    let world = GameWorld {
        score: RefCell::new(0),
        lives: RefCell::new(3),
        level: RefCell::new("Forest".to_string()),
        enemies: RefCell::new(vec!["Goblin".to_string(), "Orc".to_string()]),
    };

    println!(
        "Initial world: score={}, lives={}, level={}\n",
        world.score.borrow(),
        world.lives.borrow(),
        world.level.borrow()
    );

    // --- Multiple borrows from DIFFERENT RefCells ---
    let score_val = *world.score.borrow();          // COPY value (no long borrow)
    let mut lives_mut = world.lives.borrow_mut();   // mutable
    let mut enemies_mut = world.enemies.borrow_mut(); // mutable

    *lives_mut -= 1;
    enemies_mut.push("Dragon".to_string());

    // Safe mutation of another RefCell
    world.level.borrow_mut().push_str(" - Night");

    println!("During gameplay:");
    println!("  Score   : {}", score_val);
    println!("  Lives   : {}", *lives_mut);
    println!("  Enemies : {:?}", enemies_mut);
    println!("  Level   : {}", world.level.borrow());

    // End borrows explicitly (good practice in teaching code)
    drop(lives_mut);
    drop(enemies_mut);

    // Final safe mutation
    *world.score.borrow_mut() += 500;

    println!("\nFinal world state:");
    println!("  Score   : {}", world.score.borrow());
    println!("  Lives   : {}", world.lives.borrow());
    println!("  Level   : {}", world.level.borrow());
    println!("  Enemies : {:?}", world.enemies.borrow());
}
