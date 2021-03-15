trace
[![Build Status](https://img.shields.io/travis/gsingh93/trace/master)](https://travis-ci.org/gsingh93/trace)
[![Latest Version](https://img.shields.io/crates/v/trace.svg)](https://crates.io/crates/trace)
[![Documentation](https://docs.rs/trace/badge.svg)](https://docs.rs/trace)
-----

A procedural macro for tracing the execution of functions.

Adding `#[trace]` to the top of functions, `mod`s, or `impl`s will insert `println!` statements at the beginning and the end of the affected functions, notifying you of when that function was entered and exited and printing the argument and return values. Useful for quickly debugging whether functions that are supposed to be called are actually called without manually inserting print statements.

See the [`examples`](examples/) directory and the [documentation](https://docs.rs/trace) for more detail on how to use and configure this library.

## Installation

Add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
func_trace = "^1.0.3"
```

## Example

```rust
use func_trace::trace;

func_trace::init_depth_var!();

#[trace]
fn fib(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    fib(4);
}
```

Output:
```
[+] Entering fib(n = 4)
  [+] Entering fib(n = 3)
    [+] Entering fib(n = 2)
    [-] Exiting fib = 1
    [+] Entering fib(n = 1)
    [-] Exiting fib = 1
  [-] Exiting fib = 2
  [+] Entering fib(n = 2)
  [-] Exiting fib = 1
[-] Exiting fib = 3
```
