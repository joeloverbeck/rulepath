use crate::Seed;

pub trait DeterministicRng {
    fn next_u64(&mut self) -> u64;

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_index(&mut self, upper_bound: usize) -> Option<usize> {
        if upper_bound == 0 {
            return None;
        }

        Some((self.next_u64() % upper_bound as u64) as usize)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn from_seed(seed: Seed) -> Self {
        Self { state: seed.0 }
    }
}

impl DeterministicRng for SeededRng {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut left = SeededRng::from_seed(Seed(42));
        let mut right = SeededRng::from_seed(Seed(42));

        let left_values = [
            left.next_u64(),
            left.next_u64(),
            left.next_u64(),
            left.next_u64(),
        ];
        let right_values = [
            right.next_u64(),
            right.next_u64(),
            right.next_u64(),
            right.next_u64(),
        ];

        assert_eq!(left_values, right_values);
    }

    #[test]
    fn next_index_respects_empty_and_non_empty_bounds() {
        let mut rng = SeededRng::from_seed(Seed(11));

        assert_eq!(rng.next_index(0), None);
        assert!(rng.next_index(3).is_some_and(|index| index < 3));
    }
}
