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
