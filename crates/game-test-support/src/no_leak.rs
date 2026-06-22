//! Pairwise no-leak assertion geometry.
//!
//! Games own projection, reveal timing, authorization, snapshots, and canary
//! construction. This module only enumerates source/viewer/surface/probe cases
//! and compares caller-supplied expectations against caller-supplied
//! containment checks.

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ExposureExpectation {
    MustBeAbsent,
    MustBePresent,
    NotApplicable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeakProbe<SourceSeat, CanaryId, Canary> {
    pub source_seat: SourceSeat,
    pub canary_id: CanaryId,
    pub canary: Canary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PairwiseLeakFailureKind {
    UnexpectedPresence,
    MissingPresence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairwiseLeakFailure<SourceSeat, Viewer, Surface, CanaryId> {
    pub source_seat: SourceSeat,
    pub viewer: Viewer,
    pub surface: Surface,
    pub canary_id: CanaryId,
    pub expectation: ExposureExpectation,
    pub observed_present: bool,
    pub kind: PairwiseLeakFailureKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairwiseLeakFailures<SourceSeat, Viewer, Surface, CanaryId> {
    pub failures: Vec<PairwiseLeakFailure<SourceSeat, Viewer, Surface, CanaryId>>,
}

impl<SourceSeat, Viewer, Surface, CanaryId>
    PairwiseLeakFailures<SourceSeat, Viewer, Surface, CanaryId>
{
    pub fn is_empty(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn len(&self) -> usize {
        self.failures.len()
    }
}

impl<SourceSeat, Viewer, Surface, CanaryId> fmt::Display
    for PairwiseLeakFailure<SourceSeat, Viewer, Surface, CanaryId>
where
    SourceSeat: fmt::Debug,
    Viewer: fmt::Debug,
    Surface: fmt::Debug,
    CanaryId: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "pairwise no-leak failure: source={:?} viewer={:?} surface={:?} canary={:?} expectation={:?} observed_present={} kind={:?}",
            self.source_seat,
            self.viewer,
            self.surface,
            self.canary_id,
            self.expectation,
            self.observed_present,
            self.kind
        )
    }
}

impl<SourceSeat, Viewer, Surface, CanaryId> fmt::Display
    for PairwiseLeakFailures<SourceSeat, Viewer, Surface, CanaryId>
where
    SourceSeat: fmt::Debug,
    Viewer: fmt::Debug,
    Surface: fmt::Debug,
    CanaryId: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "{} pairwise no-leak failure(s)",
            self.failures.len()
        )?;
        for failure in &self.failures {
            writeln!(formatter, "{failure}")?;
        }
        Ok(())
    }
}

pub fn assert_pairwise_no_leak<
    Viewers,
    Surfaces,
    Probes,
    SourceSeat,
    Viewer,
    Surface,
    CanaryId,
    Canary,
    Snapshot,
    SnapshotFn,
    ExpectationFn,
    ContainsFn,
>(
    viewers: Viewers,
    surfaces: Surfaces,
    probes: Probes,
    mut snapshot: SnapshotFn,
    mut expectation: ExpectationFn,
    mut contains: ContainsFn,
) -> Result<(), PairwiseLeakFailures<SourceSeat, Viewer, Surface, CanaryId>>
where
    Viewers: IntoIterator<Item = Viewer>,
    Surfaces: IntoIterator<Item = Surface>,
    Probes: IntoIterator<Item = LeakProbe<SourceSeat, CanaryId, Canary>>,
    SourceSeat: Clone,
    Viewer: Clone,
    Surface: Clone,
    CanaryId: Clone,
    SnapshotFn: FnMut(&Viewer, &Surface) -> Snapshot,
    ExpectationFn: FnMut(&SourceSeat, &Viewer, &Surface, &CanaryId) -> ExposureExpectation,
    ContainsFn: FnMut(&Snapshot, &Canary) -> bool,
{
    let viewers = viewers.into_iter().collect::<Vec<_>>();
    let surfaces = surfaces.into_iter().collect::<Vec<_>>();
    let probes = probes.into_iter().collect::<Vec<_>>();
    let mut failures = Vec::new();

    for viewer in &viewers {
        for surface in &surfaces {
            for probe in &probes {
                let expected = expectation(&probe.source_seat, viewer, surface, &probe.canary_id);
                if expected == ExposureExpectation::NotApplicable {
                    continue;
                }

                let snapshot = snapshot(viewer, surface);
                let observed_present = contains(&snapshot, &probe.canary);
                let kind = match (expected, observed_present) {
                    (ExposureExpectation::MustBeAbsent, true) => {
                        Some(PairwiseLeakFailureKind::UnexpectedPresence)
                    }
                    (ExposureExpectation::MustBePresent, false) => {
                        Some(PairwiseLeakFailureKind::MissingPresence)
                    }
                    _ => None,
                };

                if let Some(kind) = kind {
                    failures.push(PairwiseLeakFailure {
                        source_seat: probe.source_seat.clone(),
                        viewer: viewer.clone(),
                        surface: surface.clone(),
                        canary_id: probe.canary_id.clone(),
                        expectation: expected,
                        observed_present,
                        kind,
                    });
                }
            }
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(PairwiseLeakFailures { failures })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    enum Viewer {
        Public,
        Seat(u8),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    enum Surface {
        PrivateView,
        RevealedExport,
        Diagnostics,
    }

    fn probes() -> Vec<LeakProbe<u8, &'static str, &'static str>> {
        vec![
            LeakProbe {
                source_seat: 0,
                canary_id: "seat0-card",
                canary: "S0_SECRET",
            },
            LeakProbe {
                source_seat: 1,
                canary_id: "seat1-card",
                canary: "S1_SECRET",
            },
        ]
    }

    fn viewers() -> Vec<Viewer> {
        vec![Viewer::Public, Viewer::Seat(0), Viewer::Seat(1)]
    }

    fn surfaces() -> Vec<Surface> {
        vec![
            Surface::PrivateView,
            Surface::RevealedExport,
            Surface::Diagnostics,
        ]
    }

    fn snapshot(viewer: &Viewer, surface: &Surface) -> Vec<&'static str> {
        match (viewer, surface) {
            (Viewer::Seat(0), Surface::PrivateView) => vec!["S0_SECRET"],
            (Viewer::Seat(1), Surface::PrivateView) => vec!["S1_SECRET"],
            (_, Surface::RevealedExport) => vec!["S0_SECRET", "S1_SECRET"],
            (_, Surface::Diagnostics) => vec!["S0_SECRET", "S1_SECRET"],
            _ => Vec::new(),
        }
    }

    fn expectation(
        source_seat: &u8,
        viewer: &Viewer,
        surface: &Surface,
        _canary_id: &&'static str,
    ) -> ExposureExpectation {
        match surface {
            Surface::Diagnostics => ExposureExpectation::NotApplicable,
            Surface::RevealedExport => ExposureExpectation::MustBePresent,
            Surface::PrivateView => match viewer {
                Viewer::Seat(viewer_seat) if viewer_seat == source_seat => {
                    ExposureExpectation::MustBePresent
                }
                _ => ExposureExpectation::MustBeAbsent,
            },
        }
    }

    fn contains(snapshot: &Vec<&'static str>, canary: &&'static str) -> bool {
        snapshot.iter().any(|token| token == canary)
    }

    #[test]
    fn authorized_unauthorized_revealed_and_not_applicable_cases_pass() {
        assert_pairwise_no_leak(
            viewers(),
            surfaces(),
            probes(),
            snapshot,
            expectation,
            contains,
        )
        .expect("matrix passes");
    }

    #[test]
    fn missing_canary_is_reported_for_required_presence() {
        let err = assert_pairwise_no_leak(
            [Viewer::Seat(0)],
            [Surface::PrivateView],
            [LeakProbe {
                source_seat: 0,
                canary_id: "missing",
                canary: "MISSING_SECRET",
            }],
            |_viewer, _surface| Vec::<&'static str>::new(),
            |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBePresent,
            contains,
        )
        .expect_err("missing canary fails");

        assert_eq!(err.len(), 1);
        assert_eq!(
            err.failures[0].kind,
            PairwiseLeakFailureKind::MissingPresence
        );
    }

    #[test]
    fn unexpected_canary_is_reported_for_absence_expectation() {
        let err = assert_pairwise_no_leak(
            [Viewer::Public],
            [Surface::PrivateView],
            [LeakProbe {
                source_seat: 0,
                canary_id: "public-leak",
                canary: "S0_SECRET",
            }],
            |_viewer, _surface| vec!["S0_SECRET"],
            |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
            contains,
        )
        .expect_err("unexpected canary fails");

        assert_eq!(err.len(), 1);
        assert_eq!(
            err.failures[0].kind,
            PairwiseLeakFailureKind::UnexpectedPresence
        );
    }

    #[test]
    fn not_applicable_cases_do_not_call_containment() {
        assert_pairwise_no_leak(
            [Viewer::Public],
            [Surface::Diagnostics],
            [LeakProbe {
                source_seat: 0,
                canary_id: "ignored",
                canary: "S0_SECRET",
            }],
            |_viewer, _surface| vec!["S0_SECRET"],
            |_source, _viewer, _surface, _canary_id| ExposureExpectation::NotApplicable,
            |_snapshot: &Vec<&'static str>, _canary: &&'static str| {
                panic!("not-applicable cases must skip containment")
            },
        )
        .expect("not-applicable case ignored");
    }

    #[test]
    fn exact_containment_resists_false_positive_probe() {
        assert_pairwise_no_leak(
            [Viewer::Public],
            [Surface::PrivateView],
            [LeakProbe {
                source_seat: 0,
                canary_id: "substring",
                canary: "S0_SECRET",
            }],
            |_viewer, _surface| vec!["prefix-S0_SECRET-suffix"],
            |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
            contains,
        )
        .expect("substring is not treated as exact containment");
    }

    #[test]
    fn diagnostic_rendering_names_case_coordinates() {
        let err = assert_pairwise_no_leak(
            [Viewer::Public],
            [Surface::PrivateView],
            [LeakProbe {
                source_seat: 0,
                canary_id: "rendered",
                canary: "S0_SECRET",
            }],
            |_viewer, _surface| vec!["S0_SECRET"],
            |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
            contains,
        )
        .expect_err("unexpected canary fails");

        let rendered = err.to_string();
        assert!(rendered.contains("source=0"));
        assert!(rendered.contains("viewer=Public"));
        assert!(rendered.contains("surface=PrivateView"));
        assert!(rendered.contains("canary=\"rendered\""));
        assert!(rendered.contains("expectation=MustBeAbsent"));
        assert!(rendered.contains("kind=UnexpectedPresence"));
    }
}
