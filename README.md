# howlast

`howlast` is a Rust procedural macro designed for timing code execution.
It provides an easy way to measure the duration of specific code blocks, helping developers optimize performance.

## Installation

To use `howlast`, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
howlast = "0.1.1"
```

Then, include the macro in your Rust code:

```rust
use howlast::howlast;
```

## Example Usage

Here's an example of how to use the `howlast` macro:

```rust
use howlast::howlast;

fn main() {
   howlast!(step_duration, result => 
        { 
            let x = 1 + 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
            x 
        }
    );

    print!("Ellapsed time: {:?} with result {}.", step_duration, result);
}
```

This will output the execution time of the code block along with the result of the computation.
