//! Rule-agnostic seat-count and ring-index helpers.

use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SeatCount(NonZeroUsize);

impl SeatCount {
    pub fn new(actual: usize) -> Result<Self, SeatCountError> {
        Ok(Self(NonZeroUsize::new(actual).ok_or(SeatCountError::Zero)?))
    }

    pub const fn get(self) -> usize {
        self.0.get()
    }

    pub fn checked_index(self, index: usize) -> Result<usize, SeatIndexError> {
        if index < self.get() {
            Ok(index)
        } else {
            Err(SeatIndexError::OutOfRange {
                index,
                count: self.get(),
            })
        }
    }

    pub fn next_ring_index(self, current: usize) -> Result<usize, SeatIndexError> {
        let current = self.checked_index(current)?;
        Ok(if current + 1 == self.get() {
            0
        } else {
            current + 1
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SeatCountRange {
    min: SeatCount,
    max: SeatCount,
}

impl SeatCountRange {
    pub fn inclusive(min: usize, max: usize) -> Result<Self, SeatCountRangeError> {
        if min == 0 || max == 0 {
            return Err(SeatCountRangeError::Zero);
        }
        if min > max {
            return Err(SeatCountRangeError::Inverted { min, max });
        }
        Ok(Self {
            min: SeatCount::new(min).expect("range min is nonzero"),
            max: SeatCount::new(max).expect("range max is nonzero"),
        })
    }

    pub fn validate(self, actual: usize) -> Result<SeatCount, SeatCountError> {
        let count = SeatCount::new(actual)?;
        if count.get() < self.min.get() {
            return Err(SeatCountError::BelowMinimum {
                min: self.min.get(),
                actual,
            });
        }
        if count.get() > self.max.get() {
            return Err(SeatCountError::AboveMaximum {
                max: self.max.get(),
                actual,
            });
        }
        Ok(count)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeatCountError {
    Zero,
    BelowMinimum { min: usize, actual: usize },
    AboveMaximum { max: usize, actual: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeatIndexError {
    OutOfRange { index: usize, count: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeatCountRangeError {
    Zero,
    Inverted { min: usize, max: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_count_accepts_nonzero_counts_and_rejects_zero() {
        for actual in 1..=32 {
            assert_eq!(SeatCount::new(actual).unwrap().get(), actual);
        }

        assert_eq!(SeatCount::new(0), Err(SeatCountError::Zero));
    }

    #[test]
    fn seat_count_range_validates_inclusive_bounds() {
        let range = SeatCountRange::inclusive(3, 6).expect("valid range");

        for actual in 3..=6 {
            assert_eq!(range.validate(actual).unwrap().get(), actual);
        }
        assert_eq!(
            range.validate(2),
            Err(SeatCountError::BelowMinimum { min: 3, actual: 2 })
        );
        assert_eq!(
            range.validate(7),
            Err(SeatCountError::AboveMaximum { max: 6, actual: 7 })
        );
        assert_eq!(range.validate(0), Err(SeatCountError::Zero));
    }

    #[test]
    fn seat_count_range_rejects_zero_and_inverted_bounds() {
        assert_eq!(
            SeatCountRange::inclusive(0, 2),
            Err(SeatCountRangeError::Zero)
        );
        assert_eq!(
            SeatCountRange::inclusive(2, 0),
            Err(SeatCountRangeError::Zero)
        );
        assert_eq!(
            SeatCountRange::inclusive(6, 3),
            Err(SeatCountRangeError::Inverted { min: 6, max: 3 })
        );
    }

    #[test]
    fn checked_index_accepts_only_indices_inside_count() {
        let count = SeatCount::new(4).unwrap();

        for index in 0..4 {
            assert_eq!(count.checked_index(index), Ok(index));
        }
        assert_eq!(
            count.checked_index(4),
            Err(SeatIndexError::OutOfRange { index: 4, count: 4 })
        );
    }

    #[test]
    fn next_ring_index_wraps_without_policy() {
        for actual in 1..=12 {
            let count = SeatCount::new(actual).unwrap();
            for index in 0..actual {
                assert_eq!(count.next_ring_index(index), Ok((index + 1) % actual));
            }
            assert_eq!(
                count.next_ring_index(actual),
                Err(SeatIndexError::OutOfRange {
                    index: actual,
                    count: actual,
                })
            );
        }
    }

    #[test]
    fn maximum_count_does_not_overflow_valid_ring_step() {
        let count = SeatCount::new(usize::MAX).unwrap();

        assert_eq!(count.checked_index(usize::MAX - 1), Ok(usize::MAX - 1));
        assert_eq!(count.next_ring_index(usize::MAX - 1), Ok(0));
        assert_eq!(
            count.checked_index(usize::MAX),
            Err(SeatIndexError::OutOfRange {
                index: usize::MAX,
                count: usize::MAX,
            })
        );
    }
}
