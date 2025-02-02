//! Asa interior mut types are used, calling a ref immut or mut is confusing
//! because things can be mutated through both.

use std::cell::Cell;

/// Cell allows mutations through a shared reference.
/// To avoid undefined behavior, it only allows to copy the value out (if T is Copy),
/// or replace it with another value as a whole.
/// It can be used within a single thread.
///
/// Hence, the if can happen and the compiler cannot remove it in optimization.
/// Both a and b might refer to same value, such that mutating through b can affect a as well.
///
/// However, compiler can assume no other threads are accessing cells concurrently.
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        todo!() // might happen
    }
}