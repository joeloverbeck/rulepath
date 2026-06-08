use crate::state::SecretDraftState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub round_number: u8,
    pub visible_pool_count: usize,
    pub seat_0_committed: bool,
    pub seat_1_committed: bool,
}

pub fn project_view(state: &SecretDraftState) -> PublicView {
    PublicView {
        round_number: state.round_number,
        visible_pool_count: state.visible_pool.len(),
        seat_0_committed: state.seat_committed(crate::ids::SecretDraftSeat::Seat0),
        seat_1_committed: state.seat_committed(crate::ids::SecretDraftSeat::Seat1),
    }
}
