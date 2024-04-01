#[macro_export]
macro_rules! write_optional {
    ($f:expr, $k:expr, $v:expr, $empty:expr) => {{
        if !$empty($v) {
            write!($f, "{}: {}\n", $k, $v)?;
        }
    }};
}
