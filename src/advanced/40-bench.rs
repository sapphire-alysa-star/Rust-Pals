macro_rules! bench {
    ($arg:expr) => {{
        let start = ::std::time::Instant::now();
        let x = $arg;
        let duration = start.elapsed(); // use .as_millis() if u128 wanted
        (x, duration)
    }};
}