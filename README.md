# 🃏 Rust Card Deck: Memory Management Edition

A high-performance Rust implementation of a playing card deck, focused on mastering memory ownership, borrowing, and heap allocation.

## 🚧 Status: Work in Progress
Currently following the **Stephen Grider Rust Course**. This project serves as a "Proof of Work" for low-level systems programming concepts.

## 🧠 Engineering Highlights
- **Heap Allocation**: Uses `Vec<String>` to manage dynamic card data in the Heap.
- **In-Place Mutation**: Shuffling logic uses `&mut self` to rearrange memory pointers without triggering new allocations.
- **Ownership Transfer (Zero-Copy)**: The `deal` method utilizes `split_off` to efficiently move blocks of memory from the main Deck to a Hand variable.
- **Automated Generation**: Nested `for` loops combined with the `format!` macro for O(n) deck construction.

## 🛠️ Current Features
- [x] **Custom Deck Struct**: Encapsulated data structure for card management.
- [x] **Shuffle Logic**: Integrated `rand` crate for cryptographically secure-ish randomization.
- [x] **Dealing System**: Precise memory splitting for card distribution.
- [x] **Debug Optimized**: Full implementation of the `Debug` trait for transparent memory inspection.

## 💻 Development Environment
Built and tested in a professional-grade systems environment:
- **Host OS**: Windows 10
- **Kernel**: WSL2 (Windows Subsystem for Linux)
- **Environment**: Ubuntu 24.04.3 LTS (Noble Numbat)
- **Toolchain**: Rustc 1.94.0 / Cargo 1.94.0
- **IDE**: VS Code (Remote-WSL Extension)

## 🚀 How to Run
Ensure you have the Rust toolchain installed.

## 🚀 How to Run
Ensure you have the Rust toolchain installed.

```bash
# Clone the project
git clone https://github.com/iHuGi/rust_deck.git
cd <project-folder>

# Install dependencies (Automated via Cargo)
cargo add rand@0.8.5

# Run the project
cargo run -q