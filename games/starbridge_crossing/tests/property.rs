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
