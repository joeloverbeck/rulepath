//! State scaffolding for Event Frontier.

use crate::variants::ScenarioVariant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventFrontierState {
    pub variant: ScenarioVariant,
}
