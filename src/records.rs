use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{
    DoppelClaimType, DoppelDriftKind, DoppelIntakeSourceKind, DoppelIntakeStatusKind,
    DoppelPostureKind, DoppelProfileId,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelSubjectRecord {
    pub subject_id: String,
    pub system_id: String,
    pub subject_kind: String,
    pub subject_key: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelAnchorRecord {
    pub anchor_id: String,
    pub subject_id: String,
    pub anchor_kind: String,
    pub anchor_key: String,
    pub anchor_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelEvidenceRecord {
    pub evidence_id: String,
    pub subject_id: String,
    pub anchor_id: Option<String>,
    pub evidence_kind: String,
    pub evidence_ref: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelClaimRecord {
    pub claim_id: String,
    pub subject_id: String,
    pub anchor_id: Option<String>,
    pub claim_type: DoppelClaimType,
    pub claim_key: String,
    pub claim_value: String,
    pub posture: DoppelPostureKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelPostureRecord {
    pub posture_id: String,
    pub subject_id: String,
    pub claim_id: Option<String>,
    pub posture: DoppelPostureKind,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelDriftRecord {
    pub drift_id: String,
    pub subject_id: String,
    pub anchor_id: Option<String>,
    pub drift_kind: DoppelDriftKind,
    pub posture: DoppelPostureKind,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DoppelRecordCounts {
    pub subjects: u32,
    pub anchors: u32,
    pub evidence: u32,
    pub claims: u32,
    pub postures: u32,
    pub drift: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelPostureSummary {
    pub aggregate_posture: DoppelPostureKind,
    pub review_count: u32,
    pub blocked_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelManifestRecord {
    pub manifest_id: String,
    pub run_id: String,
    pub system_id: String,
    pub compliance_run_id: Option<String>,
    pub revision_ref: String,
    pub profile_id: DoppelProfileId,
    pub slice_id: String,
    pub record_counts: DoppelRecordCounts,
    pub posture_summary: DoppelPostureSummary,
    pub generated_at: DateTime<Utc>,
    pub generator_version: String,
    pub render_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelManifestBundle {
    pub run_id: String,
    pub system_id: String,
    pub compliance_run_id: Option<String>,
    pub manifest: DoppelManifestRecord,
    pub subjects: Vec<DoppelSubjectRecord>,
    pub anchors: Vec<DoppelAnchorRecord>,
    pub claims: Vec<DoppelClaimRecord>,
    pub evidence: Vec<DoppelEvidenceRecord>,
    pub postures: Vec<DoppelPostureRecord>,
    pub drift: Vec<DoppelDriftRecord>,
    pub rendered_markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelSubjectDetail {
    pub subject: DoppelSubjectRecord,
    pub anchors: Vec<DoppelAnchorRecord>,
    pub claims: Vec<DoppelClaimRecord>,
    pub evidence: Vec<DoppelEvidenceRecord>,
    pub postures: Vec<DoppelPostureRecord>,
    pub drift: Vec<DoppelDriftRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelIntakeReceiptRecord {
    pub doppel_intake_receipt_id: String,
    pub intake_source: DoppelIntakeSourceKind,
    pub source_incident_id: String,
    pub source_projection_receipt_id: Option<String>,
    pub source_projection_id: Option<String>,
    pub system_id: String,
    pub status: DoppelIntakeStatusKind,
    pub decision_basis: String,
    pub created_at: DateTime<Utc>,
    pub detail_json: String,
}
