// #[macro_export]
// macro_rules! cartesian {
//     // Cross two arrays
//     ([$($a:tt)*], [$($b:tt)*]) => {
//         cartesian!(@impl 
//             init_b: [$($b)*,];
//             out: []; 
//             queue: [];
//             current: [$($a)*,], [$($b)*,]; 
//         )
//     };

//     // Cross three arrays
//     ([$($a:tt)*], [$($b:tt)*], [$($c:tt)*]) => {
//         cartesian!(@impl 
//             init_b: [$($b)*,];
//             out: []; 
//             queue: [[$($c)*]];
//             current: [$($a)*,], [$($b)*,]; 
//         )
//     };

//     // If `a` is empty and the queue is empty, we're done. Flush `out`.
//     (@impl 
//         init_b: $init_b:tt;
//         out: $out:tt;
//         queue: [];
//         current: [], $b:tt;
//     ) => {
//         $out
//     };

//     // If `a` is empty and the queue isn't, pass the queue to the next iteration.
//     (@impl 
//         init_b: $init_b:tt;
//         out: $out:tt; 
//         queue: [[$($queue_first:tt)*]];
//         current: [], $b:tt;
//     ) => {
//         cartesian!(@impl
//             init_b: $init_b;
//             out: [];
//             queue: [];
//             current: [$($queue_first)*], $out;
//         )
//     };

//     // If `b` is empty and `a` isn't, reset `b` to its initial value and move `a` along. 
//     (@impl 
//         init_b: $init_b:tt;
//         out: $out:tt; 
//         queue: $queue:tt;
//         current: [$a:expr, $($at:tt)*], [];
//     ) => {
//         cartesian!(@impl 
//             init_b: $init_b;
//             out: $out; 
//             queue: $queue;
//             current: [$($at)*], $init_b;
//         )
//     };

//     // If none are empty, move `b` along
//     (@impl 
//         init_b: $init_b:tt;
//         out: [$($out:tt)*]; 
//         queue: $queue:tt;
//         current: [$a:expr, $($at:tt)*], [$b:expr, $($bt:tt)*];
//     ) => {
//         cartesian!(@impl 
//             init_b: $init_b;
//             out: [$($out)* ($a, $b),]; 
//             queue: $queue;
//             current: [$a, $($at)*], [$($bt)*];
//         )
//     };
// }

#[macro_export]
macro_rules! cartesian {
    () => {
        []
    };

    ([$($a:tt),*]) => {
        [$($a),*]
    };

    ([$($a:tt),*], [$($b:tt),*]) => {
        cartesian!(@impl
            initial: [$(($a))*];
            acc: [];
            current: [$($b)*];
            queue: [];
        )
    };

    ([$($a:tt),*], [$($b:tt),*], $([$($rest:tt),*]),*) => {
        cartesian!(@impl
            initial: [$(($a))*];
            acc: [];
            current: [$($b)*];
            queue: [$([$($rest)*])*];
        )
    };

    // IMPLEMENTATION

    // If current is empty and queue is empty
    (@impl
        initial: [$($initial:tt)*];
        acc: [$($acc:tt)*];
        current: [];
        queue: [];
    ) => {
        [$($acc),*]
    };

    // If current is empty but queue has something
    (@impl
        initial: [$($initial:tt)*];
        acc: [$($acc:tt)*];
        current: [];
        queue: [$queue_head:tt $([$queue_tail:tt])*];
    ) => {
        cartesian!(@impl
            initial: [$($acc)*];
            acc: [];
            current: $queue_head;
            queue: [$($queue_tail)*];
        )
    };

    // If current has something
    (@impl
        initial: [$(($($initial:tt),*))*];
        acc: [$($acc:tt)*];
        current: [$current_head:tt $($current_tail:tt)*];
        queue: $queue:tt;
    ) => {
        cartesian!(@impl
            initial: [$(($($initial),*))*];
            acc: [$($acc)* $(($($initial),*, $current_head))*];
            current: [$($current_tail)*];
            queue: $queue;
        )
    };

    
}
