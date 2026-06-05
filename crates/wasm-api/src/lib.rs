//! Placeholder browser API crate.

pub fn placeholder_version() -> &'static str {
    "rulepath-wasm-api/0.1.0"
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_version_is_stable() {
        assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
    }
}
