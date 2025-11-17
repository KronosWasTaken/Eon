#[macro_export]
macro_rules! swrite {
    ($s:expr, $($arg:tt)*) => {{
        ::std::fmt::Write::write_fmt($s, format_args!($($arg)*)).unwrap()
    }};
}
#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let s = &mut String::new();
        swrite!(s, "{}, {}, {}", "text", 123, 4.56);
        assert_eq!(s, "text, 123, 4.56");
    }
}