#[macro_export]
macro_rules! hashmap {
    () => {{
        use std::collections::HashMap;

        HashMap::new();
    }};
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {{
        use std::collections::HashMap;
        use $crate::count_tts;

        let mut map = HashMap::with_capacity(count_tts!($($key)+));

        $(
            map.insert($key, $value);
        )+

        map
    }};
}

// From https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $( $a:tt $b:tt )*) => {
        ( count_tts!($( $a )* ) << 1) | 1
    };
    ($( $a:tt $even:tt )*) => {
        count_tts!( $( $a )* ) << 1
    };
}
