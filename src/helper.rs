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

#[derive(Debug, Clone)]
pub struct Grid<S> {
    inner: Vec<Vec<S>>,
}

impl<S> Grid<S> {
    pub fn inner(&self) -> &[Vec<S>] {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Vec<S>> {
        &mut self.inner
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = ((usize, usize), &S)> {
        let mut points = vec![];

        if x > 0 {
            points.push(((x - 1, y), self.inner.get(x - 1).and_then(|r| r.get(y))));
        }
        if y > 0 {
            points.push(((x, y - 1), self.inner.get(x).and_then(|r| r.get(y - 1))));
        }

        points.push(((x + 1, y), self.inner.get(x + 1).and_then(|r| r.get(y))));
        points.push(((x, y + 1), self.inner.get(x).and_then(|r| r.get(y + 1))));

        points
            .into_iter()
            .filter_map(|(point, value)| value.map(|value| (point, value)))
    }
}

impl From<String> for Grid<char> {
    fn from(value: String) -> Self {
        Grid {
            inner: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl<S> From<Vec<Vec<S>>> for Grid<S> {
    fn from(value: Vec<Vec<S>>) -> Self {
        Self { inner: value }
    }
}
