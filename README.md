# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector after obtaining a raw pointer to its elements.  The `bug.rs` file contains the problematic code, while `bugSolution.rs` shows a safer alternative.

## Bug Description
The bug arises from modifying a vector's contents using a raw pointer (`as_mut_ptr()`) and subsequently calling a method that might reallocate the vector's internal memory (such as `push()`).  The raw pointer becomes invalidated, leading to unpredictable behavior, memory corruption, or crashes.

## Solution
The solution avoids using raw pointers directly when dealing with dynamically sized collections like vectors.  Instead, safe methods provided by the standard library should be used.

## How to Run
1. Clone this repository.
2. Navigate to the repository's directory.
3. Run `rustc bug.rs && ./bug` to see the undefined behavior.
4. Run `rustc bugSolution.rs && ./bugSolution` to see the correct behavior.