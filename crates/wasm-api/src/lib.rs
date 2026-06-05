//! Placeholder browser API crate.

const PLACEHOLDER_VERSION: &str = "rulepath-wasm-api/0.1.0";

pub fn placeholder_version() -> &'static str {
    PLACEHOLDER_VERSION
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_ptr() -> *const u8 {
    PLACEHOLDER_VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_len() -> usize {
    PLACEHOLDER_VERSION.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_version_is_stable() {
        assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
    }
}
