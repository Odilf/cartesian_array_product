use crate::cartesian;

#[test]
fn prod_0() {
	let lol: [u32; 0] = cartesian!();
	assert_eq!(lol, [])
}

#[test]
fn one_dimension() {
	assert_eq!(cartesian!([1, 2, 3, 4, 5, 8, 13, 69420]), [1, 2, 3, 4, 5, 8, 13, 69420])
}

#[test]
fn prod_2x2() {
	let expected = [
		(1, 3), 
		(2, 3), 
		(1, 4), 
		(2, 4)
	];

	assert_eq!(cartesian!([1, 2], [3, 4]), expected)
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

	assert_eq!(cartesian!([1, 2], [3, 4], [5, 6]), expected)
}
