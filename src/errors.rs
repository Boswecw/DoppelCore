use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoppelCoreError {
    #[error("unknown doppel profile id: {0}")]
    UnknownProfileId(String),

    #[error("unknown doppel claim type: {0}")]
    UnknownClaimType(String),

    #[error("unknown doppel posture kind: {0}")]
    UnknownPostureKind(String),

    #[error("unknown doppel drift kind: {0}")]
    UnknownDriftKind(String),

    #[error("unknown doppel intake source kind: {0}")]
    UnknownIntakeSourceKind(String),

    #[error("unknown doppel intake status kind: {0}")]
    UnknownIntakeStatusKind(String),
}

pub type Result<T> = std::result::Result<T, DoppelCoreError>;
