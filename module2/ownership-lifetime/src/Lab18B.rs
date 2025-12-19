// src/bin/lab18b.rs
// Lab 18B – Advanced Split Borrowing (Real-World Patterns)

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    mana: u32,
    position: (f32, f32),
}

fn main() {
    println!("=== Lab 18B – Advanced Split Borrowing ===\n");

    let mut player = Player {
        name: "Shubh the Wise".to_string(),
        health: 100,
        mana: 80,
        position: (0.0, 0.0),
    };

    println!("Initial player: {:?}", player);

    // Step 1: Split borrow – immutable name, mutable everything else
    let name_ref = &player.name;                     // immutable borrow of name
    let player_mut = &mut player;                    // mutable borrow of whole player

    // We can read name while mutating other fields!
    player_mut.health -= 20;
    player_mut.mana += 30;
    player_mut.position = (5.5, -2.3);

    println!("After combat (name still borrowed):");
    println!("  Name   : {}", name_ref);              // OK – still valid!
    println!("  Player : {:?}", player);
    // name_ref borrow ends here

    // Step 2: Borrow individual fields independently
    let health_ref = &player.health;                 // borrow health
    let mana_mut   = &mut player.mana;               // mutable borrow mana
    let pos_ref    = &player.position;               // borrow position

    *mana_mut = (*mana_mut).saturating_sub(50);      // safe subtract

    println!("\nAfter spell cast (multiple field borrows):");
    println!("  Health : {} (borrowed immutably)", health_ref);
    println!("  Mana   : {} (mutated)", player.mana);
    println!("  Position: {:?}", pos_ref);

    // Step 3: Reborrowing – get &mut from &mut T
    reborrow_demo(&mut player);

    println!("\nFinal player state: {:?}", player);
}

fn reborrow_demo(p: &mut Player) {
    // p is &mut Player
    let name: &str = &p.name;                        // reborrow immutably
    let health: &mut u32 = &mut p.health;            // reborrow mutably

    println!("\nInside reborrow_demo:");
    println!("  Name borrowed as &str   : {}", name);
    *health = health.saturating_sub(10);
    println!("  Health decreased by 10 → {}", health);

    // We can even split further inside the function!
    let pos: &mut (f32, f32) = &mut p.position;
    pos.0 += 10.0;
    pos.1 += 5.0;
}