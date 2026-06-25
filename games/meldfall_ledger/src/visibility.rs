//! Public-observer and seat-private projection for Meldfall Ledger.
//!
//! Later tickets add redaction for private hands, stock order, actions,
//! previews, diagnostics, semantic effects, and replay exports.

use engine_core::Viewer;

use crate::state::{MeldGroup, MeldTableau, TableCard};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicTableauView {
    pub groups: Vec<PublicMeldGroupView>,
}

impl PublicTableauView {
    pub fn stable_string(&self) -> String {
        self.groups
            .iter()
            .map(PublicMeldGroupView::stable_string)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicMeldGroupView {
    pub id: String,
    pub kind: String,
    pub origin_seat: usize,
    pub cards: Vec<PublicTableCardView>,
}

impl PublicMeldGroupView {
    pub fn stable_string(&self) -> String {
        let cards = self
            .cards
            .iter()
            .map(PublicTableCardView::stable_string)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}:{}:origin={}:cards=[{}]",
            self.id, self.kind, self.origin_seat, cards
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicTableCardView {
    pub card: String,
    pub played_by: usize,
    pub score_credit_owner: usize,
    pub play_turn: u32,
}

impl PublicTableCardView {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:played_by={}:credit={}:turn={}",
            self.card, self.played_by, self.score_credit_owner, self.play_turn
        )
    }
}

pub fn project_public_tableau(tableau: &MeldTableau) -> PublicTableauView {
    PublicTableauView {
        groups: tableau.groups.iter().map(project_meld_group).collect(),
    }
}

pub fn project_tableau_for_viewer(tableau: &MeldTableau, _viewer: &Viewer) -> PublicTableauView {
    project_public_tableau(tableau)
}

fn project_meld_group(group: &MeldGroup) -> PublicMeldGroupView {
    PublicMeldGroupView {
        id: group.id.as_string(),
        kind: group.kind.stable_string(),
        origin_seat: group.origin_seat,
        cards: group.cards.iter().map(project_table_card).collect(),
    }
}

fn project_table_card(card: &TableCard) -> PublicTableCardView {
    PublicTableCardView {
        card: card.card.as_str(),
        played_by: card.played_by,
        score_credit_owner: card.score_credit_owner,
        play_turn: card.play_turn.0,
    }
}
