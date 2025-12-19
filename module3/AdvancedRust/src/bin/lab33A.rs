// =======================================================
// Hour 33: Memory Layout & Zero-Cost Abstractions
// Lab 33A – Struct Layout Analysis (WITH EXPLANATION)
// =======================================================
//
// GOAL:
// 1. Understand how Rust lays out structs in memory
// 2. Compare `repr(Rust)` vs `repr(C)`
// 3. See how field ordering affects size & padding
// 4. Learn why this is a zero-cost abstraction
//
// IMPORTANT THEORY (READ THIS):
// -----------------------------------------
// • `repr(Rust)`
//   - Compiler is FREE to reorder fields
//   - Layout is NOT stable across versions
//   - Optimized for memory efficiency
//
// • `repr(C)`
//   - Layout follows C ABI rules
//   - Field order is preserved
//   - Required for FFI (C/C++)
//
// • Padding
//   - Added to satisfy alignment requirements
//   - Can waste memory if fields are poorly ordered
//
// • Zero-cost abstraction
//   - Layout optimizations happen at COMPILE TIME
//   - No runtime overhead
//

use std::mem::{size_of, align_of};

#[allow(dead_code)]
#[repr(Rust)]
struct RustLayout {
    // Rust compiler MAY reorder these fields
    a: u8,   // 1 byte
    b: u32,  // 4 bytes (alignment = 4)
    c: u16,  // 2 bytes
}

#[allow(dead_code)]
#[repr(C)]
struct CLayout {
    // C ABI: fields are laid out EXACTLY in this order
    a: u8,   // 1 byte
    b: u32,  // 4 bytes
    c: u16,  // 2 bytes
}

#[allow(dead_code)]
#[repr(C)]
struct OptimizedCLayout {
    // Same fields, reordered MANUALLY
    // This minimizes padding while keeping C compatibility
    b: u32,  // 4 bytes
    c: u16,  // 2 bytes
    a: u8,   // 1 byte
}

fn main() {
    println!("===== Struct Memory Layout Analysis =====\n");

    // Rust layout (compiler optimized, NOT ABI-stable)
    println!(
        "RustLayout  -> size = {} bytes, align = {} bytes",
        size_of::<RustLayout>(),
        align_of::<RustLayout>()
    );

    // C layout (ABI-stable, predictable)
    println!(
        "CLayout     -> size = {} bytes, align = {} bytes",
        size_of::<CLayout>(),
        align_of::<CLayout>()
    );

    // Optimized C layout (best of both worlds)
    println!(
        "OptimizedC  -> size = {} bytes, align = {} bytes",
        size_of::<OptimizedCLayout>(),
        align_of::<OptimizedCLayout>()
    );

    println!("\n===== Key Observations =====");
    println!("• RustLayout may be smaller due to field reordering");
    println!("• CLayout preserves order but may waste memory");
    println!("• OptimizedCLayout reduces padding WITHOUT runtime cost");
}
