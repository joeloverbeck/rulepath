//! Viewer-facing labels and presentation metadata for Starbridge Crossing.

use crate::{
    ids::{StarSpaceId, StarZone},
    topology::{space_for_id, StarCoord, StarUiAnchor},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaceUiMetadata {
    pub space: StarSpaceId,
    pub coordinate_label: String,
    pub zone_label: String,
    pub anchor: StarUiAnchor,
}

pub fn space_ui_metadata(space: StarSpaceId, zone: StarZone) -> SpaceUiMetadata {
    let topology = space_for_id(space);
    SpaceUiMetadata {
        space,
        coordinate_label: coordinate_label(topology.coord),
        zone_label: zone_label(zone),
        anchor: topology.ui_anchor,
    }
}

pub fn space_label(space: StarSpaceId) -> String {
    let topology = space_for_id(space);
    format!("{} {}", space, coordinate_label(topology.coord))
}

pub fn coordinate_label(coord: StarCoord) -> String {
    format!("q{} r{} s{}", coord.q, coord.r, coord.s)
}

pub fn zone_label(zone: StarZone) -> String {
    match zone {
        StarZone::Home(point) => format!("{} home", point.label()),
        StarZone::Target(point) => format!("{} target", point.label()),
        StarZone::Neutral => "neutral".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StarPoint;

    #[test]
    fn ui_labels_are_seat_neutral_metadata_only() {
        assert_eq!(coordinate_label(StarCoord::new(1, -1, 0)), "q1 r-1 s0");
        assert_eq!(zone_label(StarZone::Home(StarPoint::North)), "north home");
        assert_eq!(zone_label(StarZone::Neutral), "neutral");
    }
}
