// Hour 6: Ownership — Identify Move vs Copy
// Stack types (i32, bool, char, tuples of Copy types) → COPY
// Heap types (String, Vec, etc.) → MOVE (unless .clone() is used)

fn main() {
    // === COPY EXAMPLE (stack-allocated, implements Copy trait) ===
    let x: i32 = 42;
    let y = x; // ✅ COPY: `x` is copied to `y`. Both are usable.
    println!("x = {}, y = {}", x, y); // No error — Copy types allow this.

    // === MOVE EXAMPLE (heap-allocated, does NOT implement Copy) ===
    let s1 = String::from("hello"); // `s1` owns heap data
    let s2 = s1; // ❌ MOVE: ownership of heap data transferred from `s1` to `s2`
    
    // println!("s1 = {}", s1); // 💥 COMPILE ERROR if uncommented! `s1` no longer valid.
    println!("s2 = {}", s2); // ✅ OK — `s2` now owns the data.

    // === Demonstrating .clone() to force deep copy (heap duplication) ===
    let s3 = String::from("world");
    let s4 = s3.clone(); // ✅ COPY (deep): heap data duplicated → both `s3` and `s4` valid
    println!("s3 = {}, s4 = {}", s3, s4); // Both work!

    // === Function calls: move into function ===
    let s5 = String::from("move me");
    takes_ownership(s5); // ❌ MOVE: `s5` moved into function
    // println!("{}", s5); // 💥 Error: `s5` no longer valid after move

    // === Function calls: copy into function (for Copy types) ===
    let num = 100;
    makes_copy(num); // ✅ COPY: `num` copied into function
    println!("num is still usable: {}", num); // ✅ OK

    // === Tuple mixing Copy and Move types ===
    let t1 = (42, String::from("tuple")); // (i32, String)
    let t2 = t1; // ❌ MOVE: because `String` doesn’t implement Copy, the whole tuple is MOVED
    // println!("{}", t1.0); // 💥 Error: `t1` moved
    println!("t2 = ({}, {})", t2.0, t2.1); // ✅ OK

    // === Summary of key rules ===
    // • Types that implement `Copy` → copied on assignment/use.
    // • All others (e.g., `String`, `Vec`) → moved (ownership transferred).
    // • After a move, the original variable **cannot be used**.
    // • `.clone()` forces a deep copy (expensive, but explicit).
}

fn takes_ownership(s: String) {
    println!("Inside function: {}", s);
    // `s` goes out of scope here and is dropped (heap memory freed)
}

fn makes_copy(x: i32) {
    println!("Inside function (copy): {}", x);
    // `x` copied, so original still valid
}