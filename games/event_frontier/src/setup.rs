//! Deterministic setup scaffolding for Event Frontier.

use crate::variants::{ScenarioVariant, VariantCatalog};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: ScenarioVariant,
}

impl SetupOptions {
    pub fn standard() -> Self {
        let variants =
            VariantCatalog::parse(include_str!("../data/variants.toml")).expect("variants parse");
        Self {
            variant: variants.standard,
        }
    }
}
