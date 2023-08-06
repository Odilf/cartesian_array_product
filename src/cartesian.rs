/// Takes N arrays and returns a list of N tuples with the combinations of the elements.
///
/// It starts alternating from the start.
///
/// Oftentimes, you want to do a `.map` after this macro. If that's the case, you might
/// want to use [`cartesian_array_map!`](crate::cartesian_array_map) instead.
///
/// # Example
///
/// ```rust
/// use cartesian_array_product::cartesian_array;
///
/// let cartesian_product = cartesian_array!(["a", "b"], ["x", "y", "z"]);
/// let expected = [
///     ("a", "x"),
///     ("b", "x"),
///     ("a", "y"),
///     ("b", "y"),
///     ("a", "z"),
///     ("b", "z"),
/// ];
///
/// assert_eq!(cartesian_product, expected);
/// ```
#[macro_export]
macro_rules! cartesian_array {
    ($([$($queue:expr),*]),*) => {
        $crate::cartesian_array_map!($([$($queue),*]),*)
    };
}

/// Same as [`cartesian_array!`] but mapping each element.
///
/// It is useful to use this macro instead of [`cartesian_array!`] followed by a `.map` because it avoids creating an intermediate array.
/// Also, `.map` doesn't work in `const` scenarious, while [`cartesian_array_map!`](crate::cartesian_array_map) does.
///
/// # Example
///
/// ## Regular use
///
/// ```rust
/// use cartesian_array_product::cartesian_array_map;
///
/// let mapped_product = cartesian_array_map!([1, 2], [3, 4]; |a, b| a + b);
/// let expected = [
///     1 + 3,
///     2 + 3,
///     1 + 4,
///     2 + 4,
/// ];
///
/// assert_eq!(mapped_product, expected);
/// ```
///
/// ## Const use
///
/// ```rust
/// use cartesian_array_product::cartesian_array_map;
///
/// const fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// const mapped_product: [i32; 4] = cartesian_array_map!([1, 2], [3, 4]; sum);
/// const expected: [i32; 4] = [
///     1 + 3,
///     2 + 3,
///     1 + 4,
///     2 + 4,
/// ];
///
/// assert_eq!(mapped_product, expected);
/// ```
/// 
/// Tip: You can use [`caf!`](crate::caf) to create a const anonymous function
/// 
/// ```rust
/// use cartesian_array_product::{cartesian_array_map, caf};
///
/// const mapped_product: [i32; 4] = cartesian_array_map!(
///     [1, 2], [3, 4]; 
///     caf!(|a: i32, b: i32| -> i32 { a + b })
/// );
/// 
/// const expected: [i32; 4] = [
///     1 + 3,
///     2 + 3,
///     1 + 4,
///     2 + 4,
/// ];
///
/// assert_eq!(mapped_product, expected);
/// ```
#[macro_export]
macro_rules! cartesian_array_map {
    ($([$($queue:expr),*]),*) => {
        $crate::cartesian_array_map!(@impl
            initial: [];
            acc: [()];
            current: [];
            queue: [$([$($queue)*])*]; // Commas are removed in implementation
            wrapper: ;
        )
    };

    ($([$($queue:tt),*]),*; $wrapper:expr) => {
        $crate::cartesian_array_map!(@impl
            initial: [];
            acc: [()];
            current: [];
            queue: [$([$($queue)*])*]; // Commas are removed in implementation
            wrapper: $wrapper;
        )
    };

    // IMPLEMENTATION

    // If current is empty and queue is empty we're done. Flush `acc`
    (@impl
        initial: $initial:tt;
        acc: [$(($($acc:tt)*))*];
        current: [];
        queue: [];
        wrapper: ;
    ) => {
        [$(($($acc),*)),*] // Add commas back in
    };

    // Same as before, but with a wrapper
    // (would be nice to just do `$($wrapper)?)`, but that doesn't work because we're already inside a repetition :/)
    (@impl
        initial: $initial:tt;
        acc: [$(($($acc:tt)*))*];
        current: [];
        queue: [];
        wrapper: $wrapper:expr;
    ) => {
        [$($wrapper ($($acc),*)),*] // Add commas back in
    };

    // If current is empty but queue has something, move the first element of queue to current
    (@impl
        initial: $initial:tt;
        acc: $acc:tt;
        current: [];
        queue: [$queue_head:tt $($queue_tail:tt)*];
        wrapper: $($wrapper:expr)?;
    ) => {
        $crate::cartesian_array_map!(@impl
            initial: $acc;
            acc: [];
            current: $queue_head;
            queue: [$($queue_tail)*];
            wrapper: $($wrapper)?;
        )
    };

    // If current has something, make a tuple with each element of initial first of current
    (@impl
        initial: [$(($($initial:tt)*))*];
        acc: [$($acc:tt)*];
        current: [$current_head:tt $($current_tail:tt)*];
        queue: $queue:tt;
        wrapper: $($wrapper:expr)?;
    ) => {
        $crate::cartesian_array_map!(@impl
            initial: [$(($($initial)*))*];
            acc: [
                $($acc)*
                $(( // This is a tuple
                    $($initial)* $current_head
                ))*
            ];
            current: [$($current_tail)*];
            queue: $queue;
            wrapper: $($wrapper)?;
        )
    };
}
