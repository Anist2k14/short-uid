# short-uid

**Compact, memory-efficient 64-bit identifiers for Rust.**

- `short-uid` was designed to minimize memory overhead when working with large numbers of identifiers, or performance critical systems.
- It provides compact, human-readable identifiers stored in a single NonZeroU64, without heap allocations or additional memory overhead for optional values.

## Key-features

- **8 bytes per identifier** – compact `u64` representation.
- **Zero-cost `Option`** – `Option<ShortUID>` occupies exactly 8 bytes thanks to Rust's niche optimization.
- **1.188 quintillion possible identifiers** – a large identifier space for devices, users, entities, and other resources.
- **Human-readable format** – identifiers can be represented as a fixed-length string.
- **No per-identifier heap allocation** – identifiers are stored directly as integers.
- **`Copy`, `Clone`, `Hash`, `Eq`** – convenient integration with Rust collections and data structures.
- **Lightweight** – designed for memory-constrained and performance-sensitive applications.


## Identifier Format

```text
NNlll-NNNNll-NNNNN
```

Where:
- `N` represents a decimal digit (`0-9`).
- `l` represents a lowercase ASCII letter (`a-z`).
- `-` is a visual separator.

Each identifier contains:
- 9 decimal digits
- 2 additional decimal digits
- 5 lowercase letters

The format provides:

```text
10^11 × 26^5
= 1,188,137,600,000,000,000
```

possible combinations, minus the single reserved zero representation used by `NonZeroU64`.

## Memory Footprint

| Type | Memory |
|---|---:|
| `ShortUID` | 8 bytes |
| `Option<ShortUID>` | 8 bytes |
| `Uuid` | 16 bytes |
| `Option<Uuid>` | Typically 17 bytes |

The compact representation can significantly reduce memory consumption in applications managing millions of entities.

## Example

```rust
use short_uid::ShortUID;

fn main() {
    let id = ShortUID::generate();

    // Human-readable representation
    println!("ID: {id}");

    // Binary representation
    let bytes: [u8; 8] = id.as_bytes();

    // Parse from a string
    let parsed = ShortUID::new(&id.to_string()).unwrap();

    assert_eq!(id, parsed);

    // Optional identifiers
    let optional: Option<ShortUID> = Some(id);

    assert_eq!(
        std::mem::size_of::<Option<ShortUID>>(),
        8
    );
}
```

## Performance Philosophy

`short-uid` prioritizes:

- Minimal memory footprint.
- Predictable data representation.
- Low allocation overhead.
- Efficient storage and comparison.
- Human-readable serialization when needed.

It is intended for systems where identifier overhead matters, especially when identifiers are stored in large collections or embedded in frequently accessed data structures.

## Security Considerations

`ShortUID` is designed for compact identification, not cryptographic security.

- It must not be used as an authentication token, access credential, session secret, or cryptographic key.
- Generated identifiers are not guaranteed to be unpredictable or globally unique.
- Random generation alone does not provide a security guarantee.
- Applications requiring strict uniqueness must implement collision detection.
- Applications requiring cryptographic security should use dedicated cryptographically secure tokens or identifiers designed for that purpose.

`ShortUID` should be treated as an identifier, not as a security mechanism.

## Collision Considerations

Identifiers are generated using random values. A large identifier space reduces collision probability, but does not guarantee global uniqueness.

Applications requiring strict uniqueness should validate identifiers when registering or storing them.

## License

Licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) for details.


## Disclaimer
This README was generated with the assistance of AI.
The source code and project design are maintained by the author.
