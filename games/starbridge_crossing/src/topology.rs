use crate::ids::{StarPoint, StarSpaceId, StarZone, SPACE_COUNT};
use std::{collections::BTreeMap, sync::LazyLock};

pub const TOPOLOGY_GENERATOR: &str = "cube_star_order_4_v1";
const ARM: i8 = 4;
const OUTER: i8 = ARM * 2;

const BEHAVIOR_KEYS: &[&str] = &[
    "when",
    "if",
    "then",
    "else",
    "selector",
    "condition",
    "trigger",
    "script",
    "loop",
    "foreach",
    "priority_expression",
    "ai_condition",
    "effect_script",
    "rule",
    "requires",
    "valid_if",
    "on_play",
    "on_reveal",
    "formula",
    "path_formula",
    "jump_formula",
    "adjacency_rule",
    "legal_if",
    "bot_policy",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StarCoord {
    pub q: i8,
    pub r: i8,
    pub s: i8,
}

impl StarCoord {
    pub const fn new(q: i8, r: i8, s: i8) -> Self {
        Self { q, r, s }
    }

    pub const fn is_cube_coord(self) -> bool {
        self.q + self.r + self.s == 0
    }

    pub const fn neighbor(self, direction: StarDirection) -> Self {
        let (dq, dr, ds) = direction.delta();
        Self::new(self.q + dq, self.r + dr, self.s + ds)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StarDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl StarDirection {
    pub const ALL: [Self; 6] = [
        Self::East,
        Self::NorthEast,
        Self::NorthWest,
        Self::West,
        Self::SouthWest,
        Self::SouthEast,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::East => 0,
            Self::NorthEast => 1,
            Self::NorthWest => 2,
            Self::West => 3,
            Self::SouthWest => 4,
            Self::SouthEast => 5,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::NorthEast => Self::SouthWest,
            Self::NorthWest => Self::SouthEast,
            Self::West => Self::East,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
        }
    }

    pub const fn delta(self) -> (i8, i8, i8) {
        match self {
            Self::East => (1, -1, 0),
            Self::NorthEast => (1, 0, -1),
            Self::NorthWest => (0, 1, -1),
            Self::West => (-1, 1, 0),
            Self::SouthWest => (-1, 0, 1),
            Self::SouthEast => (0, -1, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StarUiAnchor {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StarSpace {
    pub id: StarSpaceId,
    pub coord: StarCoord,
    pub zone: StarZone,
    pub ui_anchor: StarUiAnchor,
    pub neighbors: [Option<StarSpaceId>; 6],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Manifest {
    pub game_id: String,
    pub topology_generator: String,
    pub space_count: u16,
    pub coordinate_system: String,
    pub point_order: String,
    pub rules_version_label: String,
    pub data_version_label: String,
}

impl Manifest {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "game_id",
                "topology_generator",
                "space_count",
                "coordinate_system",
                "point_order",
                "rules_version_label",
                "data_version_label",
            ],
        )?;

        Ok(Self {
            game_id: required_string(&values, "game_id")?,
            topology_generator: required_string(&values, "topology_generator")?,
            space_count: required_u16(&values, "space_count")?,
            coordinate_system: required_string(&values, "coordinate_system")?,
            point_order: required_string(&values, "point_order")?,
            rules_version_label: required_string(&values, "rules_version_label")?,
            data_version_label: required_string(&values, "data_version_label")?,
        })
    }
}

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn spaces() -> &'static [StarSpace] {
    &SPACES
}

pub fn spaces_by_stable_order() -> impl Iterator<Item = &'static StarSpace> {
    spaces().iter()
}

pub fn space_for_id(id: StarSpaceId) -> &'static StarSpace {
    &SPACES[usize::from(id.index())]
}

pub fn coordinate_for_id(id: StarSpaceId) -> StarCoord {
    space_for_id(id).coord
}

pub fn neighbor_in_direction(id: StarSpaceId, direction: StarDirection) -> Option<StarSpaceId> {
    space_for_id(id).neighbors[direction.index()]
}

pub fn home_spaces(point: StarPoint) -> impl Iterator<Item = &'static StarSpace> {
    spaces()
        .iter()
        .filter(move |space| space.zone == StarZone::Home(point))
}

static SPACES: LazyLock<Vec<StarSpace>> = LazyLock::new(generate_spaces);

fn generate_spaces() -> Vec<StarSpace> {
    let coords = star_coords();
    let coord_to_id: BTreeMap<StarCoord, StarSpaceId> = coords
        .iter()
        .enumerate()
        .map(|(index, coord)| {
            (
                *coord,
                StarSpaceId::new(index as u16).expect("generated topology has 121 spaces"),
            )
        })
        .collect();

    coords
        .into_iter()
        .enumerate()
        .map(|(index, coord)| {
            let mut neighbors = [None; 6];
            for direction in StarDirection::ALL {
                neighbors[direction.index()] = coord_to_id.get(&coord.neighbor(direction)).copied();
            }
            StarSpace {
                id: StarSpaceId::new(index as u16).expect("generated topology has 121 spaces"),
                coord,
                zone: zone_for_coord(coord),
                ui_anchor: ui_anchor_for_coord(coord),
                neighbors,
            }
        })
        .collect()
}

fn star_coords() -> Vec<StarCoord> {
    let mut coords = Vec::with_capacity(usize::from(SPACE_COUNT));
    for r in -OUTER..=OUTER {
        for q in -OUTER..=OUTER {
            let s = -q - r;
            let coord = StarCoord::new(q, r, s);
            if is_star_coord(coord) {
                coords.push(coord);
            }
        }
    }
    coords
}

fn is_star_coord(coord: StarCoord) -> bool {
    coord.is_cube_coord()
        && (-OUTER..=OUTER).contains(&coord.q)
        && (-OUTER..=OUTER).contains(&coord.r)
        && (-OUTER..=OUTER).contains(&coord.s)
        && [coord.q, coord.r, coord.s]
            .into_iter()
            .filter(|value| (-ARM..=ARM).contains(value))
            .count()
            >= 2
}

fn zone_for_coord(coord: StarCoord) -> StarZone {
    if coord.r < -ARM {
        StarZone::Home(StarPoint::North)
    } else if coord.q > ARM {
        StarZone::Home(StarPoint::NorthEast)
    } else if coord.s > ARM {
        StarZone::Home(StarPoint::SouthEast)
    } else if coord.r > ARM {
        StarZone::Home(StarPoint::South)
    } else if coord.q < -ARM {
        StarZone::Home(StarPoint::SouthWest)
    } else if coord.s < -ARM {
        StarZone::Home(StarPoint::NorthWest)
    } else {
        StarZone::Neutral
    }
}

fn ui_anchor_for_coord(coord: StarCoord) -> StarUiAnchor {
    StarUiAnchor {
        x: i16::from((coord.q * 2) + coord.r),
        y: i16::from(coord.r * 3),
    }
}

fn parse_flat_toml(input: &str) -> Result<BTreeMap<String, String>, String> {
    let mut values = BTreeMap::new();
    for (line_index, raw_line) in input.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(format!("line {} is not key = value", line_index + 1));
        };
        let key = raw_key.trim().to_owned();
        reject_behavior_key(&key)?;
        if values.contains_key(&key) {
            return Err(format!("duplicate key `{key}`"));
        }
        values.insert(key, parse_value(raw_value.trim(), line_index + 1)?);
    }
    Ok(values)
}

fn parse_value(raw: &str, line: usize) -> Result<String, String> {
    if raw.starts_with('"') {
        if !raw.ends_with('"') || raw.len() == 1 {
            return Err(format!("line {line} has malformed quoted value"));
        }
        Ok(raw[1..raw.len() - 1].to_owned())
    } else {
        Ok(raw.to_owned())
    }
}

fn reject_behavior_key(key: &str) -> Result<(), String> {
    if key == "rules_version_label" {
        return Ok(());
    }
    if BEHAVIOR_KEYS.iter().any(|token| key.contains(token)) {
        return Err(format!("behavior-looking field `{key}` is not allowed"));
    }
    Ok(())
}

fn reject_unknown_keys(values: &BTreeMap<String, String>, allowed: &[&str]) -> Result<(), String> {
    for key in values.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(())
}

fn required_string(values: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    values
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing `{key}`"))
}

fn required_u16(values: &BTreeMap<String, String>, key: &str) -> Result<u16, String> {
    required_string(values, key)?
        .parse()
        .map_err(|_| format!("`{key}` must be a u16"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL};
    use std::collections::BTreeSet;

    #[test]
    fn topology_has_exactly_121_stable_spaces() {
        let spaces = spaces();
        assert_eq!(spaces.len(), usize::from(SPACE_COUNT));

        let ids: BTreeSet<_> = spaces.iter().map(|space| space.id.index()).collect();
        assert_eq!(ids.len(), usize::from(SPACE_COUNT));
        assert_eq!(ids.first().copied(), Some(0));
        assert_eq!(ids.last().copied(), Some(SPACE_COUNT - 1));

        for (expected, space) in spaces.iter().enumerate() {
            assert_eq!(usize::from(space.id.index()), expected);
            assert_eq!(space_for_id(space.id), space);
        }
    }

    #[test]
    fn coordinates_are_cube_coords_and_unique() {
        let coords: BTreeSet<_> = spaces().iter().map(|space| space.coord).collect();
        assert_eq!(coords.len(), usize::from(SPACE_COUNT));
        assert!(coords.iter().all(|coord| coord.is_cube_coord()));
    }

    #[test]
    fn neighbor_relations_are_symmetric_with_expected_degree_range() {
        let mut min_degree = usize::MAX;
        let mut max_degree = 0;

        for space in spaces() {
            let degree = space
                .neighbors
                .iter()
                .filter(|neighbor| neighbor.is_some())
                .count();
            min_degree = min_degree.min(degree);
            max_degree = max_degree.max(degree);

            for direction in StarDirection::ALL {
                if let Some(neighbor_id) = neighbor_in_direction(space.id, direction) {
                    assert_eq!(
                        neighbor_in_direction(neighbor_id, direction.opposite()),
                        Some(space.id)
                    );
                }
            }
        }

        assert_eq!(min_degree, 2);
        assert_eq!(max_degree, 6);
    }

    #[test]
    fn home_zones_are_ten_spaces_each_and_opposites_are_involutive() {
        let neutral_count = spaces()
            .iter()
            .filter(|space| space.zone == StarZone::Neutral)
            .count();
        assert_eq!(neutral_count, 61);

        for point in StarPoint::ALL {
            assert_eq!(home_spaces(point).count(), 10);
            assert_eq!(point.opposite().opposite(), point);
        }
    }

    #[test]
    fn ui_anchors_are_deterministic() {
        assert_eq!(
            spaces()
                .iter()
                .map(|space| space.ui_anchor)
                .collect::<BTreeSet<_>>()
                .len(),
            usize::from(SPACE_COUNT)
        );
    }

    #[test]
    fn manifest_receipt_matches_generated_topology_and_rejects_bad_fields() {
        let manifest = load_manifest().expect("manifest parses");
        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.topology_generator, TOPOLOGY_GENERATOR);
        assert_eq!(manifest.space_count, SPACE_COUNT);
        assert_eq!(manifest.coordinate_system, "cube_qrs_sum_zero");
        assert_eq!(
            manifest.point_order,
            "north,north_east,south_east,south,south_west,north_west"
        );
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.data_version_label, DATA_VERSION_LABEL);

        assert!(Manifest::parse("game_id = \"starbridge_crossing\"\nunknown = \"bad\"\n").is_err());
        assert!(
            Manifest::parse("game_id = \"starbridge_crossing\"\npath_formula = \"bad\"\n").is_err()
        );
    }
}
