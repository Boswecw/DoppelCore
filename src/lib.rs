pub mod comparison;
pub mod contracts;
pub mod correction;
pub mod errors;
pub mod extraction;
pub mod intake;
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
pub use correction::{
    DoppelCorrectionEvaluationPacket, DoppelCorrectionExecutionLane,
    DoppelCorrectionOpportunityKind, DoppelCorrectionOpportunityRecord,
    DoppelCorrectionProposalSystem, DoppelCorrectionRiskPosture, DoppelCorrectionScoringInput,
};
pub use errors::{DoppelCoreError, Result};
pub use extraction::{
    DoppelExtractedAnchor, DoppelExtractedSubject, DoppelExtractionPacket,
    DoppelExtractionPacketKind, DoppelExtractionProvenance,
};
pub use intake::{normalize_extraction_packet, DoppelNormalizedRecords};
pub use records::{
    DoppelAnchorRecord, DoppelClaimRecord, DoppelDriftRecord, DoppelEvidenceRecord,
    DoppelIntakeReceiptRecord, DoppelManifestBundle, DoppelManifestRecord, DoppelPostureRecord,
    DoppelPostureSummary, DoppelRecordCounts, DoppelSubjectDetail, DoppelSubjectRecord,
};
