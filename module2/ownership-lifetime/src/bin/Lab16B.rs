// src/bin/lab16b.rs
// Lab 16B – Real-World Borrowing with &[u8] (fixed & perfect)

// Import the fmt module for formatting functionality
use std::fmt;

// Define a struct to hold large amounts of data
struct BigData {
    label: String,      // Label to identify this data
    data: Vec<u8>,      // Vector to hold the actual binary data
}

// Implement Debug formatting for BigData for display purposes
impl fmt::Debug for BigData {
    // Define how to format BigData when printed with {:?}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BigData({}) @ {:p} len={} bytes",  // Format string with label, pointer, and length
            self.label,                         // Insert the label
            self.data.as_ptr(),                 // Insert the memory address of data
            self.data.len()                     // Insert the length of data
        )
    }
}

// Implement Drop trait to show when BigData is deallocated
impl Drop for BigData {
    // Define what happens when BigData goes out of scope
    fn drop(&mut self) {
        println!(
            "Dropping {} – freeing {} bytes at {:p}",  // Message showing deallocation
            self.label,                                 // Which BigData is being dropped
            self.data.len(),                            // How many bytes are being freed
            self.data.as_ptr()                          // Memory address being freed
        );
    }
}

// Main function - entry point of the program
fn main() {
    // Print lab title with newlines for formatting
    println!("=== Lab 16B: Real-World Borrowing with &[u8] ===\n");

    // Create a BigData instance with a large vector
    let big = BigData {
        label: "My 15MB buffer".to_string(),           // Set descriptive label
        data: create_large_vec(15 * 1024 * 1024),      // Create 15MB vector using helper function
    };

    // Print the created BigData using Debug formatting
    println!("Created: {:?}\n", big);

    // Demonstrate processing with borrowed slice
    println!("--- Processing with borrowed slice (&[u8]) ---");
    // Calculate checksum by borrowing the data as a slice (no copy/move)
    let checksum = crc32_slice(&big.data);
    // Print the calculated checksum in hexadecimal format
    println!("CRC32 checksum = 0x{:08x}\n", checksum);

    // Show that we still own the original data after borrowing
    println!("After processing → still own: {:?}\n", big);

    // Demonstrate that the same function works with different data sources
    let array: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];  // Create a fixed-size array
    let text = b"Rust rocks!";                       // Create a byte string literal

    // Showcase polymorphism through slices
    println!("--- Same function, different sources ---");
    // Calculate checksum for array by borrowing it as a slice
    println!("  array[8]      → CRC32 = 0x{:08x}", crc32_slice(&array));
    // Calculate checksum for byte string (already a slice)
    println!("  byte string   → CRC32 = 0x{:08x}", crc32_slice(text));
    // Calculate checksum for a subslice of the big vector
    println!(
        "  subslice[100] → CRC32 = 0x{:08x}",
        crc32_slice(&big.data[500..600])  // Borrow only 100 bytes from position 500-600
    );

    // End of main - big goes out of scope here and Drop is called automatically
}

// Function to calculate CRC32 checksum from any byte slice
fn crc32_slice(data: &[u8]) -> u32 {  // Accepts a borrowed slice, not owned Vec
    // Print diagnostic info about the received slice
    println!(
        "  → crc32_slice received slice at {:p}, len = {} bytes",
        data.as_ptr(),  // Memory address where slice starts
        data.len()      // Length of slice
    );

    // Initialize CRC value to all 1s (CRC32 standard)
    let mut crc: u32 = 0xffffffff;
    // Iterate through each byte in the slice
    for &byte in data {
        // XOR current byte with CRC
        crc ^= byte as u32;
        // Process each bit in the byte (8 bits)
        for _ in 0..8 {
            // Create mask based on LSB of CRC
            let mask = if (crc & 1) != 0 { 0xffffffff } else { 0 };
            // Right shift and apply polynomial if LSB was 1
            crc = (crc >> 1) ^ (0xedb88320 & mask);  // 0xedb88320 is CRC-32 polynomial
        }
    }
    // Final complement (invert all bits) per CRC32 standard
    !crc
}

// Helper function to create a large vector with patterned data
fn create_large_vec(size: usize) -> Vec<u8> {
    // Create vector by mapping each index to a byte value
    let  v: Vec<u8> = (0..size)                     // Range from 0 to size-1
        .map(|i| (i % 200) as u8 ^ 0x55)            // Pattern: i mod 200 XOR 0x55
        .collect();                                 // Collect iterator into Vec
    v  // Return the created vector
}

/*
Key concepts demonstrated:

Borrowing with &[u8]: Accepting slices instead of owned Vec<u8> avoids unnecessary copies

Polymorphism through slices: Same function works with Vec<u8>, arrays, byte strings, and subslices

Zero-cost abstraction: No runtime overhead for this flexibility

Ownership tracking: Can borrow data while still maintaining ownership

Resource management: Drop trait shows when memory is freed
*/


///////////////////
//CRC32 Algorithm Explained Line by Line

/*

```rust
// Initialize CRC value to all 1s (CRC32 standard)
let mut crc: u32 = 0xffffffff;
```
Why 0xffffffff?

In CRC32, the initial value (called "seed" or "initial remainder") is typically all 1s (0xffffffff)

This initialization helps detect leading zeros in the data

Some implementations use 0x00000000, but 0xffffffff is standard for many protocols (Ethernet, PNG, etc.)

The value 0xffffffff in binary is: 11111111111111111111111111111111 (32 bits all set to 1)

```rust
// XOR current byte with CRC
crc ^= byte as u32;
```
What this does:

crc ^= byte as u32 means crc = crc XOR byte

XOR (exclusive OR) operation:

If bits are different → result is 1

If bits are same → result is 0

Example: If crc is 0xffffffff and byte is 0x55:

text
11111111111111111111111111111111  (0xffffffff)
XOR
00000000000000000000000001010101  (0x00000055)
=
11111111111111111111111110101010  (0xffffffaa)
This mixes the current byte into the CRC value

```rust
// Create mask based on LSB of CRC
let mask = if (crc & 1) != 0 { 0xffffffff } else { 0 };
```
Understanding the mask:

crc & 1 checks the Least Significant Bit (LSB) of the CRC

& is bitwise AND: crc & 1 isolates just the last bit

If LSB is 1 → mask becomes 0xffffffff (all 32 bits = 1)

If LSB is 0 → mask becomes 0x00000000 (all 32 bits = 0)

This mask will control whether we apply the polynomial

rust
// Right shift and apply polynomial if LSB was 1
crc = (crc >> 1) ^ (0xedb88320 & mask);
Breaking this down:

Part 1: crc >> 1
Right shift the entire CRC by 1 bit

Example: 0b1011 >> 1 becomes 0b0101

This shifts all bits right, filling left with 0:

text
Before: 11111111111111111111111110101010
After:  01111111111111111111111111010101
Part 2: 0xedb88320 & mask
0xedb88320 is the CRC-32 polynomial in reversed representation

Polynomial in normal form: x³² + x²⁶ + x²³ + x²² + x¹⁶ + x¹² + x¹¹ + x¹⁰ + x⁸ + x⁷ + x⁵ + x⁴ + x² + x + 1

& mask means:

If mask is 0xffffffff → use the full polynomial

If mask is 0x00000000 → result is 0 (don't apply polynomial)

Part 3: XOR the results
The shifted CRC is XORed with (possibly) the polynomial

If LSB was 1: crc = (shifted_crc) XOR (polynomial)

If LSB was 0: crc = (shifted_crc) XOR 0 = just shifted CRC

Visual Example for One Byte
Let's trace through processing byte 0x41 ('A'):

Initialization: crc = 0xffffffff

text
11111111111111111111111111111111
XOR with byte: crc ^= 0x41

text
11111111111111111111111111111111  (0xffffffff)
XOR
00000000000000000000000001000001  (0x00000041)
=
11111111111111111111111110111110  (0xffffffbe)
First bit processing (LSB = 0):

LSB = 0 → mask = 0

crc >> 1 = 01111111111111111111111111011111

0xedb88320 & 0 = 0

XOR: shifted_crc XOR 0 = shifted_crc
Result: 01111111111111111111111111011111

Second bit processing (LSB now = 1):

LSB = 1 → mask = 0xffffffff

Right shift

XOR with polynomial
... continues for 6 more bits

After all 8 bits processed, move to next byte

Why This Works: CRC Mathematics
CRC is essentially polynomial division in GF(2):

Data is treated as a polynomial

Example: Byte 0x41 (binary 01000001) = polynomial: x⁶ + 1

We divide by the CRC polynomial

Remainder is the CRC checksum

The bit-by-bit algorithm above implements this division efficiently:

XOR with byte: Align current data with CRC

Check LSB: Determine if we need to subtract (XOR) the polynomial

Right shift: Move to next bit position

XOR with polynomial: Subtract polynomial if needed

Final Step: Complement
rust
!crc  // Invert all bits
After all data processed, we invert all bits

This is the CRC-32 standard final transformation

Example: If final CRC is 0x12345678, output is !0x12345678 = 0xedcba987

Real-World Example
For the string "123456789", the CRC32 should be 0xcbf43926:

text
Data: "123456789" (bytes: 0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39)
CRC32: 0xcbf43926
This algorithm produces that result, confirming it's a correct CRC-32 implementation following the IEEE 802.3 standard used in Ethernet, PNG, ZIP, and many other formats.
```
*/