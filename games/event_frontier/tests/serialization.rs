use engine_core::{SeatId, Seed, StableSerialize};
use event_frontier::{
    setup_match, CardCatalog, CardPresentationCatalog, EventFrontierSnapshot, SetupOptions,
    SitePresentationCatalog,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn setup_snapshot_round_trips_and_serializes_stably() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::land_rush()).expect("setup");
    let snapshot = EventFrontierSnapshot::from_state(&state);

    assert_eq!(snapshot.clone().into_state(), state);
    assert_eq!(
        snapshot.stable_bytes(),
        snapshot.stable_summary().into_bytes()
    );
    assert_eq!(snapshot.stable_hash(), state.stable_hash());
    assert!(snapshot
        .stable_summary()
        .contains("sites=site_charterhouse:agents1:settlers0:depot1:caches0"));
}

#[test]
fn card_data_rejects_behavior_looking_fields() {
    for key in ["when", "condition", "trigger", "effect", "script"] {
        let input = format!("card_ids = \"ef_border_survey\"\n{key} = \"bad\"\n");
        assert!(CardCatalog::parse(&input).is_err(), "{key} was accepted");
        let presentation_input = format!(
            "card_ids = \"ef_border_survey\"\nlabels = \"Border Survey\"\nsummaries = \"x\"\nfamilies = \"ordinary\"\naccessibility_labels = \"x\"\n{key} = \"bad\"\n"
        );
        assert!(
            CardPresentationCatalog::parse(&presentation_input).is_err(),
            "{key} was accepted by presentation metadata"
        );
        let site_presentation_input = format!(
            "site_ids = \"site_charterhouse\"\nlabels = \"Charterhouse\"\naccessibility_labels = \"Charterhouse site\"\n{key} = \"bad\"\n"
        );
        assert!(
            SitePresentationCatalog::parse(&site_presentation_input).is_err(),
            "{key} was accepted by site presentation metadata"
        );
    }
}
