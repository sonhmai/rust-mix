# Heap Allocation Performance

Refs
- https://nnethercote.github.io/perf-book/heap-allocations.html
- https://blog.logrocket.com/using-bufread-faster-rust-io-speed/

## Iterator allocation
### Problem
BufRead::lines makes it easy to read a file one line at a time:
```rust
#![allow(unused)]
fn main() {
    fn process(_: &str) {}
    use std::io::{self, BufRead};
    let mut lock = io::stdin().lock();
    for line in lock.lines() {
        process( & line ? );
    }
}
```
But the iterator it produces returns io::Result<String>, which means it allocates for every line in the file.

### Solution
Use a workhorse `String` in a loop over `BufRead::read_line`.
This reduces the number of allocations to at most a handful, and possibly just one. (The exact number depends on how many times line needs to be reallocated, which depends on the distribution of line lengths in the file.)

```rust 
use std::io::{self, BufRead};
let mut lock = io::stdin().lock();
let mut line = String::new();
while lock.read_line(&mut line)? != 0 {
    process(&line);
    line.clear();
}
```

Note: loop body operates on a `&str`, not a `String`. 
As moving String to the function does not allow it to be used again in the loop scope (not owner anymore).

[Sample PR](https://github.com/nnethercote/counts/commit/7d39bbb1867720ef3b9799fee739cd717ad1539a)

### Explanation
Why producing io::Result<String> means it allocates for every line in the file?
- because of lifetime constraints and iterator semantics.
