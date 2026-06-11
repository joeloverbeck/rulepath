//! Deterministic setup placeholders for Frontier Control.

use crate::variants::VariantMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: VariantMap,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: VariantMap::standard(),
        }
    }
}
