#![cfg(test)]

use const_anonymous_functions::caf;

use crate::{cartesian_array, cartesian_array_map};

#[test]
fn prod_0() {
    assert_eq!(cartesian_array!(), [()])
}

#[test]
fn one_dimension() {
    assert_eq!(
        cartesian_array!([1, 2, 3, 4, 5, 8, 13, 69420]),
        [1, 2, 3, 4, 5, 8, 13, 69420]
    )
}

#[test]
fn prod_2x2() {
    let expected = [(1, 3), (2, 3), (1, 4), (2, 4)];

    assert_eq!(cartesian_array!([1, 2], [3, 4]), expected)
}

#[test]
fn prod_3x4() {
    let expected = [
        (1, 1),
        (2, 1),
        (3, 1),
        (1, 2),
        (2, 2),
        (3, 2),
        (1, 3),
        (2, 3),
        (3, 3),
        (1, 4),
        (2, 4),
        (3, 4),
    ];

    assert_eq!(cartesian_array!([1, 2, 3], [1, 2, 3, 4]), expected)
}

#[test]
fn prod_2x2x2() {
    let expected = [
        (1, 3, 5),
        (2, 3, 5),
        (1, 4, 5),
        (2, 4, 5),
        (1, 3, 6),
        (2, 3, 6),
        (1, 4, 6),
        (2, 4, 6),
    ];

    assert_eq!(cartesian_array!([1, 2], [3, 4], [5, 6]), expected)
}

#[test]
fn prod_2x3x4() {
    let expected = [
        (1, 1, 1),
        (2, 1, 1),
        (1, 2, 1),
        (2, 2, 1),
        (1, 3, 1),
        (2, 3, 1),
        (1, 1, 2),
        (2, 1, 2),
        (1, 2, 2),
        (2, 2, 2),
        (1, 3, 2),
        (2, 3, 2),
        (1, 1, 3),
        (2, 1, 3),
        (1, 2, 3),
        (2, 2, 3),
        (1, 3, 3),
        (2, 3, 3),
        (1, 1, 4),
        (2, 1, 4),
        (1, 2, 4),
        (2, 2, 4),
        (1, 3, 4),
        (2, 3, 4),
    ];

    assert_eq!(cartesian_array!([1, 2], [1, 2, 3], [1, 2, 3, 4]), expected)
}

#[test]
fn prod_2x2_map() {
    let expected = [1 + 3, 2 + 3, 1 + 4, 2 + 4];

    assert_eq!(cartesian_array_map!([1, 2], [3, 4]; |a, b| a + b), expected)
}

#[test]
fn prod_2x2_const_map() {
    let expected = [1 + 3, 2 + 3, 1 + 4, 2 + 4];

    const fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    const ARRAY: [i32; 4] = cartesian_array_map!([1, 2], [3, 4]; sum);

    assert_eq!(ARRAY, expected)
}

#[test]
fn prod_2x2_const_map_caf() {
    let expected = [1 + 3, 2 + 3, 1 + 4, 2 + 4];

    const ARRAY: [i32; 4] = cartesian_array_map!(
        [1, 2], [3, 4]; 
        caf!(|a: i32, b: i32| -> i32 { a + b })
    );

    assert_eq!(ARRAY, expected)
}

#[test]
fn more_complicated_exprs() {
    let expected = [1 + 3, 2 + 3, 1 + 4, 2 + 4];

    let array = cartesian_array_map!(
        [{ 1 }, {let i = { 2 }; i}], [3, 4]; 
        caf!(|a: i32, b: i32| -> i32 { a + b })
    );

    assert_eq!(array, expected)
}

#[test]
fn recursive_macro() {
    let expected = [
        (vec![1, 2], vec![5, 6]),
        (vec![3, 4], vec![5, 6]),
        (vec![1, 2], vec![7, 8]),
        (vec![3, 4], vec![7, 8]),
    ];

    let array = cartesian_array_map!(
        [vec![1, 2], vec![3, 4]], [vec![5, 6], vec![7, 8]]
    );

    let array_2 = cartesian_array!(
        [vec![1, 2], vec![3, 4]], [vec![5, 6], vec![7, 8]]
    );

    assert_eq!(array, expected);
    assert_eq!(array_2, expected);
}

