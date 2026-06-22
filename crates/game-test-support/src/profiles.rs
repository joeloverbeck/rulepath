//! Evidence profile driver boundaries.
//!
//! These helpers validate profile metadata and then hand off all game/tool
//! behavior to caller-provided adapters.

use std::collections::BTreeSet;
use std::fmt;

pub const PROFILE_VERSION_V1: &str = "v1";
pub const REPLAY_COMMAND_V1: &str = "replay-command-v1";
pub const PUBLIC_EXPORT_V1: &str = "public-export-v1";
pub const SEAT_PRIVATE_EXPORT_V1: &str = "seat-private-export-v1";
pub const SETUP_EVIDENCE_V1: &str = "setup-evidence-v1";
pub const DOMAIN_EVIDENCE_V1: &str = "domain-evidence-v1";

const VISIBILITY_PUBLIC: &str = "public";
const VISIBILITY_VIEWER_SCOPED: &str = "viewer-scoped";
const VISIBILITY_SEAT_PRIVATE: &str = "seat-private";
const VISIBILITY_INTERNAL_DEV: &str = "internal-dev";
const VISIBILITY_PRIVATE_SOURCE: &str = "private-source";

const COMMON_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "hash_surface_version",
    "canonical_byte_authority",
    "migration_update_note",
    "not_applicable",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileMetadata<'a> {
    pub profile_id: &'a str,
    pub profile_version: &'a str,
    pub visibility_class: Option<&'a str>,
    pub validator_owner: &'a str,
    pub canonical_byte_authority: &'a str,
    pub migration_update_note: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileArtifact<'a> {
    pub metadata: ProfileMetadata<'a>,
    pub fields: &'a [&'a str],
    pub canonical_byte_claim: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProfileValidationErrorKind {
    WrongProfileId,
    WrongProfileVersion,
    MissingVisibility,
    InvalidVisibility,
    WrongValidatorOwner,
    IllegalCanonicalByteClaim,
    MissingMigrationNote,
    UnknownField,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileValidationError {
    pub kind: ProfileValidationErrorKind,
    pub detail: String,
}

impl ProfileValidationError {
    fn new(kind: ProfileValidationErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

impl fmt::Display for ProfileValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.detail)
    }
}

impl std::error::Error for ProfileValidationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileValidationReport {
    pub profile_id: String,
    pub profile_version: String,
    pub visibility_class: String,
    pub validator_owner: String,
}

pub struct ReplayCommandV1Driver {
    validator_owner: String,
}

pub struct PublicExportV1Driver {
    validator_owner: String,
}

pub struct SeatPrivateExportV1Driver {
    validator_owner: String,
}

pub struct SetupEvidenceV1Driver {
    validator_owner: String,
}

pub struct DomainEvidenceV1Driver {
    validator_owner: String,
}

impl ReplayCommandV1Driver {
    pub fn new(validator_owner: impl Into<String>) -> Self {
        Self {
            validator_owner: validator_owner.into(),
        }
    }

    pub fn validate(
        &self,
        artifact: &ProfileArtifact<'_>,
    ) -> Result<ProfileValidationReport, ProfileValidationError> {
        validate_profile(
            artifact,
            REPLAY_COMMAND_V1,
            &self.validator_owner,
            &[VISIBILITY_INTERNAL_DEV, VISIBILITY_PUBLIC],
            &["commands", "checkpoints", "expected_hashes"],
        )
    }

    pub fn validate_with<T>(
        &self,
        artifact: &ProfileArtifact<'_>,
        adapter: impl FnOnce(&ProfileValidationReport) -> T,
    ) -> Result<T, ProfileValidationError> {
        let report = self.validate(artifact)?;
        Ok(adapter(&report))
    }
}

impl PublicExportV1Driver {
    pub fn new(validator_owner: impl Into<String>) -> Self {
        Self {
            validator_owner: validator_owner.into(),
        }
    }

    pub fn validate(
        &self,
        artifact: &ProfileArtifact<'_>,
    ) -> Result<ProfileValidationReport, ProfileValidationError> {
        validate_profile(
            artifact,
            PUBLIC_EXPORT_V1,
            &self.validator_owner,
            &[VISIBILITY_PUBLIC],
            &["export_steps", "import_round_trip", "hidden_absence_tokens"],
        )
    }

    pub fn validate_with<T>(
        &self,
        artifact: &ProfileArtifact<'_>,
        adapter: impl FnOnce(&ProfileValidationReport) -> T,
    ) -> Result<T, ProfileValidationError> {
        let report = self.validate(artifact)?;
        Ok(adapter(&report))
    }
}

impl SeatPrivateExportV1Driver {
    pub fn new(validator_owner: impl Into<String>) -> Self {
        Self {
            validator_owner: validator_owner.into(),
        }
    }

    pub fn validate(
        &self,
        artifact: &ProfileArtifact<'_>,
    ) -> Result<ProfileValidationReport, ProfileValidationError> {
        validate_profile(
            artifact,
            SEAT_PRIVATE_EXPORT_V1,
            &self.validator_owner,
            &[VISIBILITY_SEAT_PRIVATE],
            &[
                "viewer_seat",
                "viewer_seat_version",
                "export_steps",
                "pairwise_no_leak",
            ],
        )
    }

    pub fn validate_with<T>(
        &self,
        artifact: &ProfileArtifact<'_>,
        adapter: impl FnOnce(&ProfileValidationReport) -> T,
    ) -> Result<T, ProfileValidationError> {
        let report = self.validate(artifact)?;
        Ok(adapter(&report))
    }
}

impl SetupEvidenceV1Driver {
    pub fn new(validator_owner: impl Into<String>) -> Self {
        Self {
            validator_owner: validator_owner.into(),
        }
    }

    pub fn validate(
        &self,
        artifact: &ProfileArtifact<'_>,
    ) -> Result<ProfileValidationReport, ProfileValidationError> {
        validate_profile(
            artifact,
            SETUP_EVIDENCE_V1,
            &self.validator_owner,
            &[
                VISIBILITY_PUBLIC,
                VISIBILITY_VIEWER_SCOPED,
                VISIBILITY_SEAT_PRIVATE,
                VISIBILITY_INTERNAL_DEV,
            ],
            &["seat_grammar_version", "setup_options", "expected_setup"],
        )
    }

    pub fn validate_with<T>(
        &self,
        artifact: &ProfileArtifact<'_>,
        adapter: impl FnOnce(&ProfileValidationReport) -> T,
    ) -> Result<T, ProfileValidationError> {
        let report = self.validate(artifact)?;
        Ok(adapter(&report))
    }
}

impl DomainEvidenceV1Driver {
    pub fn new(validator_owner: impl Into<String>) -> Self {
        Self {
            validator_owner: validator_owner.into(),
        }
    }

    pub fn validate(
        &self,
        artifact: &ProfileArtifact<'_>,
    ) -> Result<ProfileValidationReport, ProfileValidationError> {
        validate_profile(
            artifact,
            DOMAIN_EVIDENCE_V1,
            &self.validator_owner,
            &[
                VISIBILITY_PUBLIC,
                VISIBILITY_VIEWER_SCOPED,
                VISIBILITY_SEAT_PRIVATE,
                VISIBILITY_INTERNAL_DEV,
                VISIBILITY_PRIVATE_SOURCE,
            ],
            &["domain_schema_version", "domain_input", "expected_domain"],
        )
    }

    pub fn validate_with<T>(
        &self,
        artifact: &ProfileArtifact<'_>,
        adapter: impl FnOnce(&ProfileValidationReport) -> T,
    ) -> Result<T, ProfileValidationError> {
        let report = self.validate(artifact)?;
        Ok(adapter(&report))
    }
}

fn validate_profile(
    artifact: &ProfileArtifact<'_>,
    expected_profile_id: &str,
    expected_validator_owner: &str,
    allowed_visibility: &[&str],
    profile_fields: &[&str],
) -> Result<ProfileValidationReport, ProfileValidationError> {
    if artifact.metadata.profile_id != expected_profile_id {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::WrongProfileId,
            format!(
                "expected {expected_profile_id}, got {}",
                artifact.metadata.profile_id
            ),
        ));
    }
    if artifact.metadata.profile_version != PROFILE_VERSION_V1 {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::WrongProfileVersion,
            format!(
                "expected {PROFILE_VERSION_V1}, got {}",
                artifact.metadata.profile_version
            ),
        ));
    }

    let visibility = artifact.metadata.visibility_class.ok_or_else(|| {
        ProfileValidationError::new(
            ProfileValidationErrorKind::MissingVisibility,
            "visibility_class is required",
        )
    })?;
    if !allowed_visibility.contains(&visibility) {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::InvalidVisibility,
            format!("{visibility} is not valid for {expected_profile_id}"),
        ));
    }

    if artifact.metadata.validator_owner != expected_validator_owner {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::WrongValidatorOwner,
            format!(
                "expected {expected_validator_owner}, got {}",
                artifact.metadata.validator_owner
            ),
        ));
    }

    if artifact.metadata.canonical_byte_authority == "none" && artifact.canonical_byte_claim {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::IllegalCanonicalByteClaim,
            "canonical_byte_authority=none cannot declare canonical bytes",
        ));
    }

    if artifact
        .metadata
        .migration_update_note
        .is_none_or(str::is_empty)
    {
        return Err(ProfileValidationError::new(
            ProfileValidationErrorKind::MissingMigrationNote,
            "migration_update_note is required",
        ));
    }

    let allowed_fields = COMMON_FIELDS
        .iter()
        .chain(profile_fields.iter())
        .copied()
        .collect::<BTreeSet<_>>();
    for field in artifact.fields {
        if !allowed_fields.contains(field) {
            return Err(ProfileValidationError::new(
                ProfileValidationErrorKind::UnknownField,
                format!("{field} is not valid for {expected_profile_id}"),
            ));
        }
    }

    Ok(ProfileValidationReport {
        profile_id: artifact.metadata.profile_id.to_owned(),
        profile_version: artifact.metadata.profile_version.to_owned(),
        visibility_class: visibility.to_owned(),
        validator_owner: artifact.metadata.validator_owner.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMON: &[&str] = &[
        "profile_id",
        "profile_version",
        "visibility_class",
        "validator_owner",
        "game_id",
        "rules_version",
        "data_version",
        "hash_surface_version",
        "canonical_byte_authority",
        "migration_update_note",
    ];

    fn artifact<'a>(
        profile_id: &'a str,
        visibility_class: Option<&'a str>,
        validator_owner: &'a str,
        profile_fields: &'a [&'a str],
    ) -> ProfileArtifact<'a> {
        let mut fields = COMMON.to_vec();
        fields.extend_from_slice(profile_fields);
        ProfileArtifact {
            metadata: ProfileMetadata {
                profile_id,
                profile_version: PROFILE_VERSION_V1,
                visibility_class,
                validator_owner,
                canonical_byte_authority: "validator",
                migration_update_note: Some("profile migration reviewed"),
            },
            fields: Box::leak(fields.into_boxed_slice()),
            canonical_byte_claim: true,
        }
    }

    fn assert_kind(error: ProfileValidationError, kind: ProfileValidationErrorKind) {
        assert_eq!(error.kind, kind, "{error}");
    }

    #[test]
    fn replay_command_driver_validates_positive_and_delegates_after_metadata() {
        let driver = ReplayCommandV1Driver::new("replay-check");
        let artifact = artifact(
            REPLAY_COMMAND_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "replay-check",
            &["commands", "checkpoints", "expected_hashes"],
        );

        let report = driver.validate(&artifact).expect("valid replay profile");
        assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
        let delegated = driver
            .validate_with(&artifact, |report| format!("{}:adapter", report.profile_id))
            .expect("adapter runs");
        assert_eq!(delegated, "replay-command-v1:adapter");
    }

    #[test]
    fn public_export_driver_validates_positive() {
        let driver = PublicExportV1Driver::new("wasm-export");
        let artifact = artifact(
            PUBLIC_EXPORT_V1,
            Some(VISIBILITY_PUBLIC),
            "wasm-export",
            &["export_steps", "import_round_trip", "hidden_absence_tokens"],
        );

        assert_eq!(
            driver
                .validate(&artifact)
                .expect("valid public export")
                .profile_id,
            PUBLIC_EXPORT_V1
        );
    }

    #[test]
    fn seat_private_export_driver_validates_positive() {
        let driver = SeatPrivateExportV1Driver::new("wasm-export");
        let artifact = artifact(
            SEAT_PRIVATE_EXPORT_V1,
            Some(VISIBILITY_SEAT_PRIVATE),
            "wasm-export",
            &[
                "viewer_seat",
                "viewer_seat_version",
                "export_steps",
                "pairwise_no_leak",
            ],
        );

        assert_eq!(
            driver
                .validate(&artifact)
                .expect("valid seat-private export")
                .visibility_class,
            VISIBILITY_SEAT_PRIVATE
        );
    }

    #[test]
    fn setup_evidence_driver_validates_positive() {
        let driver = SetupEvidenceV1Driver::new("fixture-check");
        let artifact = artifact(
            SETUP_EVIDENCE_V1,
            Some(VISIBILITY_PUBLIC),
            "fixture-check",
            &["seat_grammar_version", "setup_options", "expected_setup"],
        );

        assert_eq!(
            driver.validate(&artifact).expect("valid setup").profile_id,
            SETUP_EVIDENCE_V1
        );
    }

    #[test]
    fn domain_evidence_driver_validates_positive() {
        let driver = DomainEvidenceV1Driver::new("briar_circuit");
        let artifact = artifact(
            DOMAIN_EVIDENCE_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "briar_circuit",
            &["domain_schema_version", "domain_input", "expected_domain"],
        );

        assert_eq!(
            driver.validate(&artifact).expect("valid domain").profile_id,
            DOMAIN_EVIDENCE_V1
        );
    }

    #[test]
    fn each_driver_rejects_wrong_profile_id() {
        let artifact = artifact(
            PUBLIC_EXPORT_V1,
            Some(VISIBILITY_PUBLIC),
            "replay-check",
            &["export_steps"],
        );

        assert_kind(
            ReplayCommandV1Driver::new("replay-check")
                .validate(&artifact)
                .expect_err("wrong profile rejects"),
            ProfileValidationErrorKind::WrongProfileId,
        );
    }

    #[test]
    fn each_driver_rejects_wrong_profile_version() {
        let mut artifact = artifact(
            REPLAY_COMMAND_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "replay-check",
            &["commands"],
        );
        artifact.metadata.profile_version = "v2";

        assert_kind(
            ReplayCommandV1Driver::new("replay-check")
                .validate(&artifact)
                .expect_err("wrong version rejects"),
            ProfileValidationErrorKind::WrongProfileVersion,
        );
    }

    #[test]
    fn each_driver_rejects_missing_visibility() {
        let artifact = artifact(REPLAY_COMMAND_V1, None, "replay-check", &["commands"]);

        assert_kind(
            ReplayCommandV1Driver::new("replay-check")
                .validate(&artifact)
                .expect_err("missing visibility rejects"),
            ProfileValidationErrorKind::MissingVisibility,
        );
    }

    #[test]
    fn public_export_rejects_non_public_visibility() {
        let artifact = artifact(
            PUBLIC_EXPORT_V1,
            Some(VISIBILITY_SEAT_PRIVATE),
            "wasm-export",
            &["export_steps"],
        );

        assert_kind(
            PublicExportV1Driver::new("wasm-export")
                .validate(&artifact)
                .expect_err("wrong visibility rejects"),
            ProfileValidationErrorKind::InvalidVisibility,
        );
    }

    #[test]
    fn each_driver_rejects_mismatched_validator_owner() {
        let artifact = artifact(
            REPLAY_COMMAND_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "fixture-check",
            &["commands"],
        );

        assert_kind(
            ReplayCommandV1Driver::new("replay-check")
                .validate(&artifact)
                .expect_err("wrong owner rejects"),
            ProfileValidationErrorKind::WrongValidatorOwner,
        );
    }

    #[test]
    fn each_driver_rejects_illegal_canonical_byte_claim() {
        let mut artifact = artifact(
            SETUP_EVIDENCE_V1,
            Some(VISIBILITY_PUBLIC),
            "fixture-check",
            &["setup_options"],
        );
        artifact.metadata.canonical_byte_authority = "none";
        artifact.canonical_byte_claim = true;

        assert_kind(
            SetupEvidenceV1Driver::new("fixture-check")
                .validate(&artifact)
                .expect_err("illegal byte claim rejects"),
            ProfileValidationErrorKind::IllegalCanonicalByteClaim,
        );
    }

    #[test]
    fn canonical_byte_authority_none_accepts_absent_byte_claim() {
        let mut artifact = artifact(
            DOMAIN_EVIDENCE_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "briar_circuit",
            &["domain_input"],
        );
        artifact.metadata.canonical_byte_authority = "none";
        artifact.canonical_byte_claim = false;

        DomainEvidenceV1Driver::new("briar_circuit")
            .validate(&artifact)
            .expect("authority none without byte claim passes");
    }

    #[test]
    fn each_driver_rejects_absent_migration_note() {
        let mut artifact = artifact(
            DOMAIN_EVIDENCE_V1,
            Some(VISIBILITY_INTERNAL_DEV),
            "briar_circuit",
            &["domain_input"],
        );
        artifact.metadata.migration_update_note = None;

        assert_kind(
            DomainEvidenceV1Driver::new("briar_circuit")
                .validate(&artifact)
                .expect_err("missing migration note rejects"),
            ProfileValidationErrorKind::MissingMigrationNote,
        );
    }

    #[test]
    fn cross_profile_field_valid_for_one_profile_rejects_in_another() {
        let artifact = artifact(
            SETUP_EVIDENCE_V1,
            Some(VISIBILITY_PUBLIC),
            "fixture-check",
            &["viewer_seat"],
        );

        assert_kind(
            SetupEvidenceV1Driver::new("fixture-check")
                .validate(&artifact)
                .expect_err("foreign field rejects"),
            ProfileValidationErrorKind::UnknownField,
        );
    }

    #[test]
    fn adapter_is_not_called_when_metadata_rejects() {
        let artifact = artifact(
            PUBLIC_EXPORT_V1,
            Some(VISIBILITY_SEAT_PRIVATE),
            "wasm-export",
            &["export_steps"],
        );
        let mut called = false;

        let error = PublicExportV1Driver::new("wasm-export")
            .validate_with(&artifact, |_| {
                called = true;
            })
            .expect_err("invalid metadata rejects before adapter");

        assert_eq!(error.kind, ProfileValidationErrorKind::InvalidVisibility);
        assert!(!called);
    }
}
