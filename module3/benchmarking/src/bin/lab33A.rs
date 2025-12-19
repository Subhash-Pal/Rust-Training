

/*
What this program demonstrates

Struct memory layout & padding

C-style deterministic layout (repr(C))

Pinning for self-referential structs

Zero-cost abstraction (iterator vs loop)
*/
use std::mem::{size_of, align_of};
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::time::Instant;

/* ============================================================
   PART 1: MEMORY LAYOUT & PADDING
   ============================================================ */

/*
   Rust normally uses `repr(Rust)` which allows the compiler
   to reorder fields for optimal layout.

   To *teach* padding and field-order impact, we MUST freeze
   layout using `#[repr(C)]`.
*/

#[allow(dead_code)]
#[repr(C)]
struct BadLayout {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes
    c: u16,  // 2 bytes
}

/*
   Same fields, better ordering.
   Less padding → smaller size → cache-friendly.
*/

#[allow(dead_code)]
#[repr(C)]
struct GoodLayout {
    b: u64,  // 8 bytes
    c: u16,  // 2 bytes
    a: u8,   // 1 byte
}

/* ============================================================
   PART 2: C-STYLE STRUCT INTEROP (FFI SAFE)
   ============================================================ */

/*
   This struct matches a typical C struct.
   `repr(C)` guarantees:
   - Field order preserved
   - C alignment rules
   - Safe for FFI / binary protocols
*/

#[repr(C)]
struct SensorData {
    id: u32,
    timestamp: u64,
    value: u16,
    status: u8,
}

/* ============================================================
   PART 3: PINNING (SELF-REFERENTIAL STRUCT)
   ============================================================ */

/*
   This struct contains:
   - Owned data (`String`)
   - A pointer to its own field

   Moving this struct would invalidate the pointer.
   Pinning guarantees a stable memory address.
*/

struct SelfRef {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned, // prevents Unpin
}

impl SelfRef {
    fn new(text: &str) -> Pin<Box<SelfRef>> {
        let mut boxed = Box::pin(SelfRef {
            data: text.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        // Get pointer to `data`
        let data_ptr = &boxed.data as *const String;

        // SAFETY:
        // Object is pinned, so its address will not change.
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            let inner = Pin::get_unchecked_mut(mut_ref);
            inner.ptr = data_ptr;
        }

        boxed
    }

    fn print(&self) {
        unsafe {
            println!(
                "Pinned data='{}', ptr points to='{}'",
                self.data,
                &*self.ptr
            );
        }
    }
}

/* ============================================================
   PART 4: ZERO-COST ABSTRACTIONS
   ============================================================ */

/*
   High-level iterator version.
   In RELEASE mode, this compiles to code
   equivalent to the manual loop.
*/

fn sum_with_iterator() -> i64 {
    (0..1_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

/*
   Low-level manual loop.
*/

fn sum_with_loop() -> i64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        if i % 2 == 0 {
            sum += i;
        }
    }
    sum
}

/* ============================================================
   MAIN FUNCTION
   ============================================================ */

fn main() {
    println!("=== Memory Layout Analysis ===");

    println!(
        "BadLayout  -> size: {} bytes, alignment: {} bytes",
        size_of::<BadLayout>(),
        align_of::<BadLayout>()
    );

    println!(
        "GoodLayout -> size: {} bytes, alignment: {} bytes",
        size_of::<GoodLayout>(),
        align_of::<GoodLayout>()
    );

    /*
       Expected output on 64-bit system:
       BadLayout  -> 24 bytes
       GoodLayout -> 16 bytes
    */

    println!("\n=== C Interop Struct ===");

    println!(
        "SensorData -> size: {} bytes, alignment: {} bytes",
        size_of::<SensorData>(),
        align_of::<SensorData>()
    );

    println!("\n=== Pinning Demo ===");

    let pinned = SelfRef::new("hello pinned world");
    pinned.print();

    println!("\n=== Zero-cost Abstraction Demo ===");

    let start = Instant::now();
    let s1 = sum_with_iterator();
    println!("Iterator sum={} time={:?}", s1, start.elapsed());

    let start = Instant::now();
    let s2 = sum_with_loop();
    println!("Loop     sum={} time={:?}", s2, start.elapsed());

    println!("\nNOTE:");
    println!("Run with `cargo run --release` to observe zero-cost behavior.");
}
