// src/bin/lab24a_iterators.rs
#![allow(dead_code)]
fn main() {
    println!("=== 5 Custom Iterator Examples ===\n");

    println!("--- 1. Countdown Iterator ---");
    let countdown = Countdown::from(5);
    for num in countdown {
        print!("{}... ", num);
    }
    println!("Liftoff!\n");

    println!("--- 2. Alphabet Character Iterator ---");
    let alphabet = Alphabet::new();
    println!("First 10 letters: {:?}", alphabet.take(10).collect::<Vec<_>>());
    println!("All letters: {:?}", Alphabet::new().collect::<Vec<_>>());
    println!();

    println!("--- 3. Sliding Window Iterator ---");
    let data = vec![1, 2, 3, 4, 5, 6];
    let windows = SlidingWindow::new(&data, 3);
    for window in windows {
        println!("Window: {:?}", window);
    }
    println!();

    println!("--- 4. Cycle Iterator with Limit ---");
    let repeater = CycleLimit::new(vec!["A", "B", "C"], 7);
    for item in repeater {
        print!("{} ", item);
    }
    println!("\n");

    println!("--- 5. Prime Number Iterator ---");
    let primes = PrimeNumbers::new();
    println!("First 15 prime numbers:");
    for (i, prime) in primes.take(15).enumerate() {
        println!("Prime {}: {}", i + 1, prime);
    }
}

/// A countdown iterator that starts from a given number and decrements to 1.
/// 
/// # Examples
/// ```
/// let countdown = Countdown::from(5);
/// for num in countdown {
///     println!("{}...", num);  // Prints: 5... 4... 3... 2... 1...
/// }
/// ```
#[derive(Debug, Clone)]
struct Countdown {
    current: u32,
}
impl Countdown {
    /// Creates a new countdown starting from the given number.
    /// 
    /// # Arguments
    /// * `start` - The number to start counting down from
    /// 
    /// # Panics
    /// Panics if `start` is 0.
    pub fn from(start: u32) -> Self {
        if start == 0 {
            panic!("Countdown cannot start from 0");
        }
        Countdown { current: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            let value = self.current;
            self.current -= 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.current as usize;
        (size, Some(size))
    }
}

impl ExactSizeIterator for Countdown {}

/// An iterator that yields lowercase English alphabet characters from 'a' to 'z'.
/// 
/// # Examples
/// ```
/// let alphabet = Alphabet::new();
/// assert_eq!(alphabet.take(3).collect::<Vec<_>>(), vec!['a', 'b', 'c']);
/// ```
#[derive(Debug, Clone)]
struct Alphabet {
    current: u8,
}

impl Alphabet {
    /// Creates a new alphabet iterator.
    pub fn new() -> Self {
        Alphabet { current: b'a' }
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Alphabet {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= b'z' {
            let ch = self.current as char;
            self.current += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (b'z' - self.current + 1) as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Alphabet {}

/// A sliding window iterator over a slice that yields overlapping windows of fixed size.
/// 
/// # Examples
/// ```
/// let data = vec![1, 2, 3, 4];
/// let windows = SlidingWindow::new(&data, 2);
/// for window in windows {
///     println!("{:?}", window);  // Prints: [1, 2], [2, 3], [3, 4]
/// }
/// ```
#[derive(Debug, Clone)]
struct SlidingWindow<'a, T> {
    data: &'a [T],
    window_size: usize,
    position: usize,
}

impl<'a, T> SlidingWindow<'a, T> {
    /// Creates a new sliding window iterator.
    /// 
    /// # Arguments
    /// * `data` - The slice to iterate over
    /// * `window_size` - The size of each window
    /// 
    /// # Panics
    /// Panics if `window_size` is 0 or greater than the length of `data`.
    pub fn new(data: &'a [T], window_size: usize) -> Self {
        if window_size == 0 {
            panic!("Window size must be greater than 0");
        }
        if window_size > data.len() {
            panic!("Window size cannot be greater than data length");
        }
        SlidingWindow {
            data,
            window_size,
            position: 0,
        }
    }
}

impl<'a, T> Iterator for SlidingWindow<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + self.window_size <= self.data.len() {
            let window = &self.data[self.position..self.position + self.window_size];
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.data.len() >= self.window_size {
            self.data.len() - self.window_size - self.position + 1
        } else {
            0
        };
        (remaining, Some(remaining))
    }
}

/// An iterator that cycles through a collection a limited number of times.
/// 
/// # Examples
/// ```
/// let repeater = CycleLimit::new(vec![1, 2, 3], 5);
/// assert_eq!(repeater.collect::<Vec<_>>(), vec![1, 2, 3, 1, 2]);
/// ```
#[derive(Debug, Clone)]
struct CycleLimit<T> 
where
    T: Clone,
{
    items: Vec<T>,
    position: usize,
    remaining_cycles: usize,
}

impl<T> CycleLimit<T>
where
    T: Clone,
{
    /// Creates a new cycle-limited iterator.
    /// 
    /// # Arguments
    /// * `items` - The items to cycle through
    /// * `limit` - Maximum number of items to yield
    pub fn new(items: Vec<T>, limit: usize) -> Self {
        CycleLimit {
            items,
            position: 0,
            remaining_cycles: limit,
        }
    }
}

impl<T> Iterator for CycleLimit<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_cycles == 0 || self.items.is_empty() {
            return None;
        }

        let item = self.items[self.position].clone();
        self.position = (self.position + 1) % self.items.len();
        self.remaining_cycles -= 1;
        
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining_cycles, Some(self.remaining_cycles))
    }
}

impl<T> ExactSizeIterator for CycleLimit<T> where T: Clone {}

/// An infinite iterator that yields prime numbers in ascending order.
/// 
/// # Examples
/// ```
/// let primes = PrimeNumbers::new();
/// let first_five: Vec<u64> = primes.take(5).collect();
/// assert_eq!(first_five, vec![2, 3, 5, 7, 11]);
/// ```
#[derive(Debug)]
struct PrimeNumbers {
    primes_found: Vec<u64>,
    next_candidate: u64,
}

impl PrimeNumbers {
    /// Creates a new prime number iterator.
    pub fn new() -> Self {
        PrimeNumbers {
            primes_found: Vec::new(),
            next_candidate: 2,
        }
    }

    /// Checks if a number is prime using the primes found so far.
    fn is_prime(&self, n: u64) -> bool {
        // Check divisibility by known primes up to sqrt(n)
        let limit = (n as f64).sqrt() as u64 + 1;
        for &prime in &self.primes_found {
            if prime > limit {
                break;
            }
            if n % prime == 0 {
                return false;
            }
        }
        true
    }
}

impl Default for PrimeNumbers {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PrimeNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Special case for the first prime (2)
        if self.primes_found.is_empty() {
            self.primes_found.push(2);
            return Some(2);
        }

        // Find the next prime number
        while !self.is_prime(self.next_candidate) {
            self.next_candidate += if self.next_candidate == 2 { 1 } else { 2 };
        }

        let prime = self.next_candidate;
        self.primes_found.push(prime);
        self.next_candidate += if prime == 2 { 1 } else { 2 };
        
        Some(prime)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Infinite iterator
        (0, None)
    }
}