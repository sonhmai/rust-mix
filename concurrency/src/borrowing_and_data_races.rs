//! Rust Atomics and Locks.
//!
//! One rule of Rust to prevent data races and undefined behavior is there can be never
//! more than one mut ref to an object.
fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        // never happens. Because this fn has immutable ref to "a", nothing in the entire program
        // can mutably borrow this integer. So the compiler can conclude that *a will not change
        // and the condition of if will never be true -> can remove call to x as optimization.
        // Impossible to write Rust program what breaks compiler assumptions except using unsafe.
        todo!()
    }
}
