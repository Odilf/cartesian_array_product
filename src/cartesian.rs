/// Takes N arrays and returns a list of N tuples with the combinations of the elements.
/// 
/// It starts alternating from the start. 
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
    ($([$($queue:tt),*]),*) => {
        $crate::cartesian_array_map!($([$($queue),*]),*)
    };
}


#[macro_export]
macro_rules! cartesian_array_map {
    ($([$($queue:tt),*]),*) => {
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
