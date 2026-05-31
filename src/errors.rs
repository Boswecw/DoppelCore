use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoppelCoreError {
    #[error("unknown doppel profile id: {0}")]
    UnknownProfileId(String),

    #[error("unknown doppel subject kind: {0}")]
    UnknownSubjectKind(String),

    #[error("unknown doppel anchor kind: {0}")]
    UnknownAnchorKind(String),

    #[error("unknown doppel claim type: {0}")]
    UnknownClaimType(String),

    #[error("unknown doppel truth class: {0}")]
    UnknownTruthClass(String),

    #[error("unknown doppel determinism class: {0}")]
    UnknownDeterminismClass(String),

    #[error("unknown doppel posture kind: {0}")]
    UnknownPostureKind(String),

    #[error("unknown doppel drift kind: {0}")]
    UnknownDriftKind(String),

    #[error("unknown doppel intake source kind: {0}")]
    UnknownIntakeSourceKind(String),

    #[error("unknown doppel intake status kind: {0}")]
    UnknownIntakeStatusKind(String),

    #[error("extracted anchor `{anchor}` references unknown subject `{subject}`")]
    DanglingExtractionAnchor { anchor: String, subject: String },
}

pub type Result<T> = std::result::Result<T, DoppelCoreError>;
