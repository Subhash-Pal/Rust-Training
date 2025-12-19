// Lab 16A: Passing Large Structures Without Cloning
// Focus: Ownership transfer vs borrowing, &T vs T, avoiding heap copies

fn main() {
    println!("=== Lab 16A: Passing Large Structures Without Cloning ===\n");

    // Create a large Vec<u8> simulating 10MB of data
    let large_data = create_large_vec(10 * 1024 * 1024); // 10 MB
    println!("large_data created at {:p} (size: {} bytes)", 
             large_data.as_ptr(), large_data.len());

    println!("\n--- 1. Taking ownership (moves the Vec) ---");
    process_by_ownership(large_data);

    // This would cause a compile error!
    // println!("After ownership transfer, large_data len: {}", large_data.len());

    // Recreate the data for the next examples
    let large_data = create_large_vec(10 * 1024 * 1024);
    println!("\n--- 2. Borrowing immutably (&T) - no copy, no move ---");
    process_by_borrowing(&large_data);
    // large_data is still usable!
    println!("After borrowing, we can still use large_data (len = {})", large_data.len());

    println!("\n--- 3. Borrowing mutably (&mut T) ---");
    let mut large_data = create_large_vec(10 * 1024 * 1024);
    process_by_mut_borrowing(&mut large_data);
    println!("After mutable borrow, first 8 bytes became: {:?}", 
             &large_data[..8]);

    println!("\n--- 4. What if we accidentally clone? (Expensive!) ---");
    let large_data = create_large_vec(10 * 1024 * 1024);
    process_by_cloning(large_data.clone()); // Explicit deep copy!
    println!("Original large_data still available after clone (len = {})", large_data.len());
    println!("But we wasted ~10MB of memory with an unnecessary copy!");

    println!("\nKey Takeaway:");
    println!("   • Use &T or &mut T → zero-cost borrowing, no heap allocation");
    println!("   • Use T         → moves ownership (or clones if you call .clone())");
    println!("   • .clone() on large Vec → full heap copy → slow + high memory");
}

// Simulates creating a large buffer (e.g. file contents, image, etc.)
fn create_large_vec(size: usize) -> Vec<u8> {
    println!("Allocating new Vec<u8> of {} bytes at {:p}", size, 
             Vec::<u8>::with_capacity(size).as_ptr());
    let mut vec = Vec::with_capacity(size);
    // Fill with some pattern so it's not all zeros (prevents some optimizations)
    for i in 0..size {
        vec.push((i % 251) as u8); // 251 is prime, good for visible patterns
    }
    vec // return the filled Vec 
}

// BAD: Takes ownership → original Vec is moved and dropped
fn process_by_ownership(data: Vec<u8>) {
    println!("  → process_by_ownership received data at {:p}", data.as_ptr());
    println!("  → Processing {} bytes (ownership taken)", data.len());

    // Do some fake work
    let sum: u64 = data.iter().map(|&b| b as u64).sum();
    println!("  → Fake computation result: {}", sum);

    // data is dropped here → heap memory freed
    println!("  → data dropped at end of function");
}

// GOOD: Borrows immutably → no move, no copy
fn process_by_borrowing(data: &Vec<u8>) {
    println!("  → process_by_borrowing received reference to data at {:p}", data.as_ptr());
    println!("  → Processing {} bytes (borrowed, no copy)", data.len());

    let sum: u64 = data.iter().map(|&b| b as u64).sum();
    println!("  → Fake computation result: {}", sum);

    // No drop of the Vec here — caller still owns it
}

// GOOD: Mutable borrow when we need to modify
fn process_by_mut_borrowing(data: &mut Vec<u8>) {
    println!("  → process_by_mut_borrowing received &mut at {:p}", data.as_ptr());
    // Modify first few bytes to prove we really have mutable access
    for i in 0..8 {
        data[i] = 0xFF;
    }
    println!("  → Modified first 8 bytes to 0xFF");
}

// EXPENSIVE: Explicit clone creates full copy on heap
fn process_by_cloning(data: Vec<u8>) {
    println!("  → process_by_cloning received CLONED data at {:p}", data.as_ptr());
    println!("  → This copy cost ~{} bytes of heap allocation!", data.len());

    let sum: u64 = data.iter().map(|&b| b as u64).sum();
    println!("  → Fake computation result: {}", sum);
    // The clone is dropped here — memory freed
}

/*
Learning Outcomes Achieved:

Clear visual difference between moving vs borrowing (pointer addresses stay the same when borrowing)
Proof that borrowing does not copy heap data
Demonstration of the high cost of .clone() on large structures
Understanding when ownership transfer is appropriate vs when borrowing is better

Perfect for teaching or self-study on Rust's ownership and borrowing system!
*/