//! Setup skeleton and typed options for Flood Watch.

use crate::variants::ScenarioVariant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: ScenarioVariant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: ScenarioVariant::standard(),
        }
    }
}
