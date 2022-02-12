#[macro_export]
macro_rules! duration {
    ($func:expr) => {{
        let start = std::time::Instant::now();
        let res = $func;
        let duration = start.elapsed();
        (res, duration)
    }};
}
