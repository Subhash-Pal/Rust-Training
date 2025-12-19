fn main() {
    println!("=== Lab 18A – Split Borrowing & NLL (Correct) ===\n");

    let mut data = vec![10, 20, 30, 40, 50];

    // First split: [0,1] | [2,3,4]
    let (left, right) = data.split_at_mut(2);

    let first = &left[0]; // data[0]

    // Second split: [2,3] | [4]
    let (middle_slice, last_slice) = right.split_at_mut(2);

    let middle = &mut middle_slice[..]; // data[2..4]
    let last = &last_slice[0];          // data[4]

    println!("Before mutation:");
    println!("  first  = {}", first);
    println!("  middle = {:?}", middle);
    println!("  last   = {}", last);

    middle[0] *= 100;
    middle[1] += 999;

    println!("\nAfter mutation:");
    println!("  first  = {} (unchanged)", first);
    println!("  middle = {:?}", middle);
    println!("  last   = {} (unchanged)", last);
    println!("  full data = {:?}", data);
}
