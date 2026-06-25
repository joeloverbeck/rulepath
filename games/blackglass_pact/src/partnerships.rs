use crate::ids::{BlackglassSeat, TeamId};

pub fn canonical_team_ids() -> [TeamId; 2] {
    TeamId::ALL
}

pub fn team_id_for_index(index: usize) -> Option<TeamId> {
    match index {
        0 => Some(TeamId::NorthSouth),
        1 => Some(TeamId::EastWest),
        _ => None,
    }
}

pub const fn team_for_seat(seat: BlackglassSeat) -> TeamId {
    match seat {
        BlackglassSeat::North | BlackglassSeat::South => TeamId::NorthSouth,
        BlackglassSeat::East | BlackglassSeat::West => TeamId::EastWest,
    }
}

pub const fn members_for_team(team: TeamId) -> [BlackglassSeat; 2] {
    match team {
        TeamId::NorthSouth => [BlackglassSeat::North, BlackglassSeat::South],
        TeamId::EastWest => [BlackglassSeat::East, BlackglassSeat::West],
    }
}

pub const fn partner_for(seat: BlackglassSeat) -> BlackglassSeat {
    match seat {
        BlackglassSeat::North => BlackglassSeat::South,
        BlackglassSeat::East => BlackglassSeat::West,
        BlackglassSeat::South => BlackglassSeat::North,
        BlackglassSeat::West => BlackglassSeat::East,
    }
}
