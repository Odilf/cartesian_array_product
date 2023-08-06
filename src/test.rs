use crate::cartesian;

#[test]
fn prod_0() {
	assert_eq!(cartesian!(), [()])
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

	assert_eq!(cartesian!([1, 2, 3], [1, 2, 3, 4]), expected)
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

	assert_eq!(cartesian!([1, 2], [1, 2, 3], [1, 2, 3, 4]), expected)
}
