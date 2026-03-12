// Checking RUST speed

fn main() {
    // We use a 64-bit unsigned integer (u64) to hold a massive number.
    // It's 'mut' (mutable) so we can update it inside the loop.
    let mut val: u64 = 0;

    // Captures the exact system time right before the loops start.
    let now = std::time::Instant::now();

    // Outer loop: Runs 100,000 times.
    for i in 0..100_000 {
        // Inner loop: Runs 10,000 times for every single outer loop iteration.
        // Total operations: 100,000 * 10,000 = 1,000,000,000 (1 Billion).
        for j in 0..10_000 {
            // '^' is the bitwise XOR operator.
            // .wrapping_add() tells Rust: "If this number gets too big for 
            // a u64, just wrap back to zero instead of crashing the program."
            val = val.wrapping_add(i ^ j);
        }
    }

    // .elapsed() calculates the difference between 'now' and the current time.
    // {:?} is a special formatter used to print timing data clearly.
    println!("Finished in: {:?}", now.elapsed());
    
    // We print the result so the compiler doesn't "cheat" and skip 
    // the loop (if you don't use the result, the optimizer might delete the code!).
    println!("Result: {}", val);
}