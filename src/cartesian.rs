/// Takes N arrays and returns a list of N tuples with the combinations of the elements.
/// 
/// It starts alternating from the start. 
/// 
/// # Example
/// 
/// ```rust
/// use cartesian_array_product::cartesian;
/// 
/// let cartesian_product = cartesian!(["a", "b"], ["x", "y", "z"]);
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
macro_rules! cartesian {
    ($($args:tt)*) => {
        $crate::cartesian_map!($($args)*)
    };
}

#[macro_export]
macro_rules! cartesian_map {
    // General case
    ($([$($rest:tt),*]),*) => {
        cartesian!(@impl
            initial: [];
            acc: [()]; // Commas are removed in implementation
            current: [];
            queue: [$([$($rest)*])*]; // Commas are removed in implementation
        )
    };

    // IMPLEMENTATION

    // If current is empty and queue is empty we're done. Flush `acc`
    (@impl
        initial: $initial:tt;
        acc: [$(($($acc:tt)*))*];
        current: [];
        queue: [];
    ) => {
        [$(($($acc),*)),*] // Add commas back in
    };

    // If current is empty but queue has something, move the first element of queue to current
    (@impl
        initial: $initial:tt;
        acc: $acc:tt;
        current: [];
        queue: [$queue_head:tt $($queue_tail:tt)*];
    ) => {
        cartesian!(@impl
            initial: $acc;
            acc: [];
            current: $queue_head;
            queue: [$($queue_tail)*];
        )
    };

    // If current has something, make a tuple with each element of initial first of current
    (@impl
        initial: [$(($($initial:tt)*))*];
        acc: [$($acc:tt)*];
        current: [$current_head:tt $($current_tail:tt)*];
        queue: $queue:tt;
    ) => {
        cartesian!(@impl
            initial: [$(($($initial)*))*];
            acc: [
                $($acc)* 
                $(( // This is a tuple
                    $($initial)* $current_head
                ))*
            ];
            current: [$($current_tail)*];
            queue: $queue;
        )
    };

    
}
