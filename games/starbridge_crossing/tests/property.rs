use starbridge_crossing::{spaces_by_stable_order, SPACE_COUNT};

#[test]
fn topology_order_is_deterministic() {
    let first: Vec<_> = spaces_by_stable_order()
        .map(|space| {
            (
                space.id.index(),
                space.coord.q,
                space.coord.r,
                space.coord.s,
            )
        })
        .collect();
    let second: Vec<_> = spaces_by_stable_order()
        .map(|space| {
            (
                space.id.index(),
                space.coord.q,
                space.coord.r,
                space.coord.s,
            )
        })
        .collect();

    assert_eq!(first.len(), usize::from(SPACE_COUNT));
    assert_eq!(first, second);
}

#[test]
fn setup_has_at_most_one_occupant_per_space_for_supported_counts() {
    for count in [2, 3, 4, 6] {
        let seats = (0..count)
            .map(|index| engine_core::SeatId::from_zero_based_index(index as u32))
            .collect::<Vec<_>>();
        let state = starbridge_crossing::setup_match(
            engine_core::Seed(31),
            &seats,
            &starbridge_crossing::SetupOptions::default(),
        )
        .unwrap();
        let occupied = state
            .occupancy
            .iter()
            .filter(|occupant| occupant.is_some())
            .count();

        assert_eq!(occupied, state.pegs.len());
    }
}

#[test]
fn legal_step_destinations_are_empty_in_setup_position() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = starbridge_crossing::setup_match(
        engine_core::Seed(31),
        &seats,
        &starbridge_crossing::SetupOptions::default(),
    )
    .unwrap();

    for step in starbridge_crossing::legal_step_moves(&state) {
        assert_eq!(state.occupancy(step.to), None);
    }
}
