// Hour 7: Borrowing & References — Fix Borrow Checker Errors

fn main() {
    // ============= ✅ FIXED VERSION: Immutable then mutable borrows =============
    let mut s = String::from("hello");

    // Immutable borrows first
    let r1 = &s;
    let r2 = &s;
    println!("Immutable refs: {}, {}", r1, r2);

    // Now mutable borrow (after immutable refs go out of scope)
    let r3 = &mut s;
    r3.push_str(", world!");
    println!("After mutation: {}", r3);

    // New immutable borrow after mutable one ends
    let r4 = &s;
    println!("Final value: {}", r4);

    // ============= Fix: Use reference instead of move =============
    let s2 = String::from("original");
    let s3 = &s2; // borrow, don't move
    println!("s2: {}, s3: {}", s2, s3);

    // ============= Fix: Scoped mutable borrow for Vec =============
    let mut data = vec![1, 2, 3];
    {   //let _x =& mut data 
        let r = &mut data;
        println!("Before push: {:?}", r);
        r.push(4);
        println!("After push: {:?}", r);
    } // `r` dropped here
    data.push(5);
    println!("Final vec: {:?}", data);

    // ============= Demonstrate safe function borrowing =============
    safe_borrowing_demo(); // 👈 Now actually used!
}

// These functions are now called — no dead code!
fn modify_string(s: &mut String) {
    s.push_str(" — modified!");
}

fn print_string(s: &String) {
    println!("Read-only: {}", s);
}

fn safe_borrowing_demo() {
    let mut text = String::from("Start");
    print_string(&text);           // immutable borrow
    modify_string(&mut text);      // mutable borrow — OK (no overlap)
    print_string(&text);           // new immutable borrow
    println!("Final: {}", text);
}