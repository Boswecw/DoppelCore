pub mod comparison;
pub mod contracts;
pub mod errors;
pub mod records;

pub use comparison::{
    DoppelAnchorDelta, DoppelDeltaKind, DoppelDriftDelta, DoppelHistoryEntry,
    DoppelManifestComparison,
};
pub use contracts::{
    DoppelAnchorKind, DoppelClaimType, DoppelDeterminismClass, DoppelDriftKind,
    DoppelIntakeSourceKind, DoppelIntakeStatusKind, DoppelPostureKind, DoppelProfileId,
    DoppelSubjectKind, DoppelTruthClass,
};
pub use errors::{DoppelCoreError, Result};
pub use records::{
    DoppelAnchorRecord, DoppelClaimRecord, DoppelDriftRecord, DoppelEvidenceRecord,
    DoppelIntakeReceiptRecord, DoppelManifestBundle, DoppelManifestRecord, DoppelPostureRecord,
    DoppelPostureSummary, DoppelRecordCounts, DoppelSubjectDetail, DoppelSubjectRecord,
};
