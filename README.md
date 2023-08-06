# cartesian-array-product

This crate provides a macro to generate an array with the cartesian product of multiple arrays.

It is meant to be usable in `const` contexts.

## Usage

Add this crate as a dependency:

```bash
cargo add cartesian_array_product
```

### Example

```rust
use cartesian_array_product::cartesian_array;

let product = cartesian_array!([1, 2], [3, 4]);
let expected = [
    (1, 3),
    (2, 3),
    (1, 4),
    (2, 4),
];

assert_eq!(product, expected);
```

See <https://docs.rs/cartesian_array_product> for further info. 

### Caveat

This macro doesn't allow trailing commas in the input arrays. This is because `$($var:whatever),*` doesn't match trailing commas, and adding manually `$($var:whatever),* $(,)?` is too much repetation for a thing that, honestly, isn't that important. (maybe i'm just writing this to convience myself because i hate not having trailing commas 😭😭😭)
