use super::*;
use crate::export;

macro_rules! arrays {
    ( $($len:literal $fmt:literal,)+ ) => { $(
        impl<T> Format for [T; $len]
        where
            T: Format
        {
            default_format!();

            #[inline]
            fn _format_tag() -> Str {
                internp!($fmt)
            }

            #[inline]
            fn _format_data(&self) {
                export::fmt_array(self);
            }
        }
    )+ };
}

arrays! {
    0 "{=[?;0]}",
    1 "{=[?;1]}",
    2 "{=[?;2]}",
    3 "{=[?;3]}",
    4 "{=[?;4]}",
    5 "{=[?;5]}",
    6 "{=[?;6]}",
    7 "{=[?;7]}",
    8 "{=[?;8]}",
    9 "{=[?;9]}",
    10 "{=[?;10]}",
    11 "{=[?;11]}",
    12 "{=[?;12]}",
    13 "{=[?;13]}",
    14 "{=[?;14]}",
    15 "{=[?;15]}",
    16 "{=[?;16]}",
    17 "{=[?;17]}",
    18 "{=[?;18]}",
    19 "{=[?;19]}",
    20 "{=[?;20]}",
    21 "{=[?;21]}",
    22 "{=[?;22]}",
    23 "{=[?;23]}",
    24 "{=[?;24]}",
    25 "{=[?;25]}",
    26 "{=[?;26]}",
    27 "{=[?;27]}",
    28 "{=[?;28]}",
    29 "{=[?;29]}",
    30 "{=[?;30]}",
    31 "{=[?;31]}",
    32 "{=[?;32]}",
    64 "{=[?;64]}",
    128 "{=[?;128]}",
    256 "{=[?;256]}",
    512 "{=[?;512]}",
    1024 "{=[?;1024]}",
    2048 "{=[?;2048]}",
    4096 "{=[?;4096]}",
    8192 "{=[?;8192]}",
    16384 "{=[?;16384]}",
    32768 "{=[?;32768]}",
    65536 "{=[?;65536]}",
    131072 "{=[?;131072]}",
    262144 "{=[?;262144]}",
    524288 "{=[?;524288]}",
    1048576 "{=[?;1048576]}",
    2097152 "{=[?;2097152]}",
    4194304 "{=[?;4194304]}",
    8388608 "{=[?;8388608]}",
    16777216 "{=[?;16777216]}",
    33554432 "{=[?;33554432]}",
    67108864 "{=[?;67108864]}",
    134217728 "{=[?;134217728]}",
    268435456 "{=[?;268435456]}",
    536870912 "{=[?;536870912]}",
    1073741824 "{=[?;1073741824]}",
    100 "{=[?;100]}",
    1000 "{=[?;1000]}",
    10000 "{=[?;10000]}",
    100000 "{=[?;100000]}",
    1000000 "{=[?;1000000]}",
    10000000 "{=[?;10000000]}",
    100000000 "{=[?;100000000]}",
    1000000000 "{=[?;1000000000]}",
}