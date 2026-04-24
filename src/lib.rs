pub mod contracts;
pub mod errors;
pub mod records;

pub use contracts::{
    DoppelClaimType, DoppelDriftKind, DoppelIntakeSourceKind, DoppelIntakeStatusKind,
    DoppelPostureKind, DoppelProfileId,
};
pub use errors::{DoppelCoreError, Result};
pub use records::{
    DoppelAnchorRecord, DoppelClaimRecord, DoppelDriftRecord, DoppelEvidenceRecord,
    DoppelIntakeReceiptRecord, DoppelManifestBundle, DoppelManifestRecord, DoppelPostureRecord,
    DoppelPostureSummary, DoppelRecordCounts, DoppelSubjectDetail, DoppelSubjectRecord,
};
