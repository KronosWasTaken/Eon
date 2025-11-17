use eon_core::ErrorSummary;
pub fn unwrap_or_panic<T>(result: Result<T, ErrorSummary>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            panic!("{}", e.to_string_ansi());
        }
    }
}