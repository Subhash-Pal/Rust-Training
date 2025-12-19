// src/bin/lab18c.rs
// Lab 18C – Interior Mutability + Split Borrowing (RefCell Done Right)

use std::cell::RefCell;

#[derive(Debug)]
struct GameWorld {
    score: RefCell<u32>,
    lives: RefCell<u32>,
    level: RefCell<String>,
    enemies: RefCell<Vec<String>>,
}

fn main() {
    println!("=== Lab 18C – Interior Mutability + Split Borrowing ===\n");

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
    let score_ref = world.score.borrow();             // immutable
    let mut lives_mut = world.lives.borrow_mut();     // mutable
    let level_ref = world.level.borrow();             // immutable
    let mut enemies_mut = world.enemies.borrow_mut(); // mutable

    *lives_mut -= 1;
    enemies_mut.push("Dragon".to_string());

    // Cannot mutate level while level_ref exists
    drop(level_ref);

    // Now safe
    world.level.borrow_mut().push_str(" - Night");

    println!("During gameplay:");
    println!("  Score   : {}", *score_ref);
    println!("  Lives   : {}", *lives_mut);
    println!("  Enemies : {:?}", enemies_mut);

    // End all borrows before fresh access
    drop(score_ref);
    drop(lives_mut);
    drop(enemies_mut);

    println!("  Level   : {}", world.level.borrow());

    // Final mutation
    *world.score.borrow_mut() += 500;

    println!("\nFinal world state:");
    println!("  Score   : {}", world.score.borrow());
    println!("  Lives   : {}", world.lives.borrow());
    println!("  Level   : {}", world.level.borrow());
    println!("  Enemies : {:?}", world.enemies.borrow());

    // --- OPTIONAL: intentional runtime violation ---
    println!("\nBonus: RefCell rule violation demo (intentional panic):");
    violation_demo(&world);
}

fn violation_demo(world: &GameWorld) {
    let _imm = world.score.borrow();     // immutable borrow
    let _mut = world.score.borrow_mut(); // ❌ PANIC: same RefCell
    println!("Never reached");
}
