use engine_core::{SeatId, Viewer};
use frontier_control::{project_view, setup_match, SetupOptions, SiteId, StakeSupplyStatus};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn public_view_is_output_equivalent_for_all_viewers() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let observer = project_view(&state, &Viewer { seat_id: None });
    let seat_0 = project_view(
        &state,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );
    let seat_1 = project_view(
        &state,
        &Viewer {
            seat_id: Some(SeatId("seat_1".to_owned())),
        },
    );

    assert_eq!(observer, seat_0);
    assert_eq!(observer, seat_1);
}

#[test]
fn public_view_carries_rust_computed_supply_flags() {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    state.site_mut(SiteId::Ford).unwrap().stake = true;
    state.last_stake_supply = vec![StakeSupplyStatus {
        site: SiteId::Ford,
        supplied: false,
    }];

    let view = project_view(&state, &Viewer { seat_id: None });
    assert_eq!(
        view.sites
            .iter()
            .find(|site| site.site == SiteId::Ford)
            .unwrap()
            .supplied,
        Some(false)
    );
}
