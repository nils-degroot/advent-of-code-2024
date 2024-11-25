#[macro_export]
macro_rules! hashmap {
    () => {{
        use std::collections::HashMap;
        HashMap::new();
    }};
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};
}
