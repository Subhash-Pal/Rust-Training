/*
Pinned, FFI-Safe, Zero-Copy Buffer Descriptor

📌 Scenario
You are building a high-performance networking / storage engine where:

A buffer descriptor is passed to C code / OS / DMA

Memory address must never change

Rust must still provide safety guarantees
*/

// =======================================================
// Hour 33: Advanced Memory Layout & Pinning
// Lab 33B – FFI-Safe, Pinned, Zero-Cost Buffer
// =======================================================
//
// REAL-WORLD USE CASE:
// • Network stacks
// • DMA buffers
// • Async runtimes
// • OS / kernel interfaces
//
// CORE CONCEPTS USED:
// • repr(C) for ABI stability
// • Pin<T> to prevent movement in memory
// • PhantomPinned to opt-out of Unpin
// • Safe public API over unsafe internals
//

use std::marker::PhantomPinned;
use std::mem::{align_of, size_of};
use std::pin::Pin;
use std::ptr::NonNull;

// -------------------------------
// FFI-safe buffer descriptor
// -------------------------------
#[repr(C)]
struct RawBuffer {
    ptr: *mut u8,   // Raw pointer for C / OS / DMA
    len: usize,     // Buffer length
}

// -------------------------------
// Rust-side safe wrapper
// -------------------------------
struct PinnedBuffer {
    raw: RawBuffer,

    // Prevents this struct from being moved
    _pin: PhantomPinned,
}

impl PinnedBuffer {
    /// Allocate a pinned buffer
    fn new(size: usize) -> Pin<Box<Self>> {
        let mut data = Vec::with_capacity(size);
        let ptr = data.as_mut_ptr();

        // Prevent Vec from freeing memory
        std::mem::forget(data);

        let buffer = PinnedBuffer {
            raw: RawBuffer {
                ptr,
                len: size,
            },
            _pin: PhantomPinned,
        };

        Box::pin(buffer)
    }

    /// Safe accessor to raw pointer (for FFI)
    fn as_raw(self: Pin<&Self>) -> *mut u8 {
        self.get_ref().raw.ptr
    }

    /// Length accessor
    fn len(self: Pin<&Self>) -> usize {
        self.get_ref().raw.len
    }
}

// -------------------------------
// Simulated C API (FFI boundary)
// -------------------------------
unsafe fn c_consume_buffer(ptr: *mut u8, len: usize) {
    for i in 0..len {
        *ptr.add(i) = (i % 255) as u8;
    }
}

// -------------------------------
// Demo
// -------------------------------
fn main() {
    let buffer = PinnedBuffer::new(16);

    println!("===== Advanced Memory Layout =====");
    println!("RawBuffer size  = {}", size_of::<RawBuffer>());
    println!("RawBuffer align = {}", align_of::<RawBuffer>());

    unsafe {
        // SAFETY:
        // • Memory is pinned
        // • Pointer is valid
        // • Length is controlled
        c_consume_buffer(buffer.as_ref().as_raw(), buffer.as_ref().len());
    }

    println!("Pinned buffer safely passed to C-style API");
}

/*

🧠 Why This Is Advanced Rust
🔹 1. repr(C)

Guarantees layout for C / OS / hardware

Required for DMA, syscalls, kernel APIs

🔹 2. Pin<Box<T>>

Prevents memory relocation

Critical for self-referential or FFI structs

🔹 3. PhantomPinned

Opts out of Unpin

Compiler enforces immovability

🔹 4. Unsafe Contained, Not Spread
unsafe fn c_consume_buffer(...)


Unsafe is localized

Public API remains safe

🔥 Zero-Cost Abstraction Proof
Feature	Runtime Cost
Pin<T>	0
repr(C)	0
PhantomPinned	0
Safety checks	Compile-time
🧩 Where This Pattern Is Used

Tokio internals

Linux io_uring

RDMA buffers

Game engines

Database storage layers

🏆 Interview-Ready Explanation

“We use Pin<Box<T>> with repr(C) to ensure ABI-safe,
immovable memory when interfacing with C or hardware,
while keeping unsafe code strictly encapsulated.”


*/