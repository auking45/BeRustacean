#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ($($k: expr => $v: expr,)+) => { hashmap!($($k => $v),+) };
    ($($k: expr => $v: expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                _map.insert($k, $v);
            )*

            _map
        }
    }
}
