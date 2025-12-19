// src/bin/lab18b.rs
// Lab 18B – Advanced Split Borrowing (CORRECT & REAL-WORLD)

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    mana: u32,
    position: (f32, f32),
}

fn main() {
    println!("=== Lab 18B – Advanced Split Borrowing (Fixed & Perfect) ===\n");

    let mut player = Player {
        name: "Who the Wise".to_string(),
        health: 100,
        mana: 80,
        position: (0.0, 0.0),
    };

    println!("Initial player: {:?}\n", player);

    // CORRECT WAY: Borrow individual fields independently
    let name_ref = &player.name;              // immutable borrow of name
    let health_mut = &mut player.health;      // mutable borrow of health
    let mana_mut = &mut player.mana;          // mutable borrow of mana
    let pos_mut = &mut player.position;       // mutable borrow of position

    // We can safely mutate the mutable fields while reading name!
    *health_mut -= 30;
    *mana_mut += 50;
    pos_mut.0 += 10.5;
    pos_mut.1 -= 3.2;

    println!("After combat (split field borrowing):");
    println!("  Name     : {} (immutably borrowed)", name_ref);
    println!("  Health   : {} (mutated)", player.health);
    println!("  Mana     : {} (mutated)", player.mana);
    println!("  Position : {:?} (mutated)", player.position);

    // All borrows end here — we can borrow the whole player again
    println!("\nFull player state: {:?}", player);

    // Bonus: Reborrowing in a function
    reborrow_demo(&mut player);
    println!("\nAfter reborrow_demo: {:?}", player);
}

fn reborrow_demo(p: &mut Player) {
    println!("\nInside reborrow_demo:");

    // Reborrow fields from &mut Player
    let name: &str = &p.name;                     // immutable reborrow
    let health: &mut u32 = &mut p.health;         // mutable reborrow

    println!("  Reading name   : {}", name);
    *health = health.saturating_sub(20);
    println!("  Health reduced : {}", health);

    // We can even reborrow position and mutate it
    let pos: &mut (f32, f32) = &mut p.position;
    pos.0 += 5.0;
    pos.1 += 10.0;
    println!("  Position moved : {:?}", pos);
}