# cartesian-array-product

This crate provides a macro to generate an array with the cartesian product of multiple arrays.

It is meant to be usable in `const` contexts.

## Example

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
