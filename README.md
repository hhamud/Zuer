# Zuer

This library provides implementations of prime fields and elliptic curve operations in Rust. In the future, it will also include lattice-based cryptography, signatures, and symmetric/asymmetric cryptography.

## File Structure

*   `src/lib.rs`: Contains the core traits and structs, including the `PrimeField` trait and the `Fe` struct for representing field elements.
*   `src/curves.rs`: Defines specific prime fields, such as `BN254` and `SmallPrime`, and implements the `PrimeField` trait for them. It also includes unit tests for field element conversions and addition.
*   `src/point.rs`: Defines the `Point` struct for representing points on an elliptic curve and implements basic point operations like addition.

## Usage

To use this library, add the following to your `Cargo.toml` file:

[https://github.com/hhamud/zuer](https://github.com/hhamud/zuer)

```toml
[dependencies]
zuer = { git = "https://github.com/hhamud/zuer" }
```

Here's a basic example of how to use the library:

```rust
use zuer::{Fe, SmallPrime};

fn main() {
    let a: Fe<SmallPrime> = 5u64.into();
    let b: Fe<SmallPrime> = 10u64.into();
    let c = a + b;

    println!("a: {}", a.value());
    println!("b: {}", b.value());
    println!("c: {}", c.value());
}
```

## Testing

The `src/curves.rs` file includes unit tests for basic field operations. To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to suggest improvements or report bugs.

## License

This library is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
