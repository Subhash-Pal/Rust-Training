// ==============================================
// Unsafe Rust + libc Example
// Manual Memory Management
// ==============================================

use libc::{malloc, free, c_void};

fn main() {
    unsafe {
        // Allocate memory for 5 i32 values
        let count = 5;
        let size = count * std::mem::size_of::<i32>();

        // malloc returns a raw void pointer
        let ptr = malloc(size) as *mut i32;

        if ptr.is_null() {
            panic!("malloc failed");
        }

        // Write values into allocated memory
        for i in 0..count {
            *ptr.add(i) = (i as i32) * 10;
        }

        // Read values back
        for i in 0..count {
            println!("Value[{}] = {}", i, *ptr.add(i));
        }

        // Free allocated memory
        free(ptr as *mut c_void);
    }
}
