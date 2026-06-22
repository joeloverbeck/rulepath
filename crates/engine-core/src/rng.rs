use crate::Seed;

pub trait DeterministicRng {
    fn next_u64(&mut self) -> u64;

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Legacy modulo bounded-index sampling.
    ///
    /// This method consumes exactly one `next_u64` word for every nonzero
    /// bound and returns `word % upper_bound`. It is intentionally biased for
    /// bounds that do not divide the `u64` range and is retained for legacy
    /// deterministic surfaces that already pin this consumption pattern.
    fn next_index(&mut self, upper_bound: usize) -> Option<usize> {
        if upper_bound == 0 {
            return None;
        }

        Some((self.next_u64() % upper_bound as u64) as usize)
    }

    /// Unbiased bounded-index sampling, version 1.
    ///
    /// This method returns `None` without consuming a word for zero bounds. For
    /// nonzero bounds it computes the largest `u64` prefix divisible by
    /// `upper_bound`, redraws words outside that accepted zone, and returns the
    /// accepted word modulo the bound. Rejections consume additional `next_u64`
    /// words and are part of the deterministic surface.
    fn next_index_unbiased_v1(&mut self, upper_bound: usize) -> Option<usize> {
        if upper_bound == 0 {
            return None;
        }

        let upper = upper_bound as u128;
        let range = u128::from(u64::MAX) + 1;
        let accepted_zone = range - (range % upper);

        loop {
            let value = u128::from(self.next_u64());
            if value < accepted_zone {
                return Some((value % upper) as usize);
            }
        }
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

    struct CountingRng {
        values: Vec<u64>,
        draws: usize,
    }

    impl CountingRng {
        fn new(values: Vec<u64>) -> Self {
            Self { values, draws: 0 }
        }
    }

    impl DeterministicRng for CountingRng {
        fn next_u64(&mut self) -> u64 {
            self.draws += 1;
            self.values.remove(0)
        }
    }

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

    #[test]
    fn legacy_next_index_modulo_vectors_and_draw_count_are_pinned() {
        let mut rng = CountingRng::new(vec![10, 11]);

        assert_eq!(rng.next_index(0), None);
        assert_eq!(rng.draws, 0);
        assert_eq!(rng.next_index(3), Some(1));
        assert_eq!(rng.draws, 1);
        assert_eq!(rng.next_index(8), Some(3));
        assert_eq!(rng.draws, 2);
    }

    #[test]
    fn next_index_unbiased_v1_zero_and_accepted_vectors_are_pinned() {
        let mut rng = CountingRng::new(vec![15, 1_000_005]);

        assert_eq!(rng.next_index_unbiased_v1(0), None);
        assert_eq!(rng.draws, 0);
        assert_eq!(rng.next_index_unbiased_v1(8), Some(7));
        assert_eq!(rng.draws, 1);
        assert_eq!(rng.next_index_unbiased_v1(1_000_003), Some(2));
        assert_eq!(rng.draws, 2);
    }

    #[test]
    fn next_index_unbiased_v1_rejection_vector_and_draw_count_are_pinned() {
        let range = u128::from(u64::MAX) + 1;
        let accepted_zone_for_three = range - (range % 3);
        let rejected = accepted_zone_for_three as u64;
        let mut rng = CountingRng::new(vec![rejected, 4, 9]);

        assert_eq!(rejected, u64::MAX);
        assert_eq!(rng.next_index_unbiased_v1(3), Some(1));
        assert_eq!(rng.draws, 2);
        assert_eq!(rng.next_index_unbiased_v1(3), Some(0));
        assert_eq!(rng.draws, 3);
    }

    #[test]
    fn next_index_unbiased_v1_matches_existing_local_algorithm() {
        fn local_algorithm<R: DeterministicRng>(rng: &mut R, upper_bound: usize) -> Option<usize> {
            if upper_bound == 0 {
                return None;
            }

            let upper = upper_bound as u128;
            let range = u128::from(u64::MAX) + 1;
            let accepted_zone = range - (range % upper);

            loop {
                let value = u128::from(rng.next_u64());
                if value < accepted_zone {
                    return Some((value % upper) as usize);
                }
            }
        }

        let values = vec![u64::MAX, 4, 9, 1_000_005];
        let mut shared = CountingRng::new(values.clone());
        let mut local = CountingRng::new(values);

        assert_eq!(
            shared.next_index_unbiased_v1(3),
            local_algorithm(&mut local, 3)
        );
        assert_eq!(shared.draws, local.draws);
        assert_eq!(
            shared.next_index_unbiased_v1(1_000_003),
            local_algorithm(&mut local, 1_000_003)
        );
        assert_eq!(shared.draws, local.draws);
    }
}
