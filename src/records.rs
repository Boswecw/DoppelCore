use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{
    DoppelAnchorKind, DoppelClaimType, DoppelDeterminismClass, DoppelDriftKind,
    DoppelIntakeSourceKind, DoppelIntakeStatusKind, DoppelPostureKind, DoppelProfileId,
    DoppelSubjectKind, DoppelTruthClass,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelSubjectRecord {
    pub subject_id: String,
    pub subject_kind: DoppelSubjectKind,
    pub repo_id: String,
    pub path: Option<String>,
    pub display_name: String,
    pub revision_ref: String,
    pub source_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelAnchorRecord {
    pub anchor_id: String,
    pub subject_id: String,
    pub anchor_kind: DoppelAnchorKind,
    pub path: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub symbol_name: Option<String>,
    pub anchor_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelEvidenceRecord {
    pub evidence_id: String,
    pub subject_id: Option<String>,
    pub anchor_id: Option<String>,
    pub evidence_type: String,
    pub probe_method: String,
    pub source_ref: String,
    pub captured_at: DateTime<Utc>,
    pub digest: String,
    pub determinism_class: DoppelDeterminismClass,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelClaimRecord {
    pub claim_id: String,
    pub subject_id: String,
    pub claim_type: DoppelClaimType,
    pub statement: String,
    pub truth_class: DoppelTruthClass,
    pub posture: DoppelPostureKind,
    pub evidence_refs: Vec<String>,
    pub anchor_refs: Vec<String>,
    pub confidence_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelPostureRecord {
    pub posture_id: String,
    pub subject_id: Option<String>,
    pub claim_id: Option<String>,
    pub posture: DoppelPostureKind,
    pub basis: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelDriftRecord {
    pub drift_id: String,
    pub subject_id: String,
    pub anchor_id: Option<String>,
    pub drift_kind: DoppelDriftKind,
    pub posture: DoppelPostureKind,
    pub summary: String,
    pub detected_at: DateTime<Utc>,
    pub evidence_refs: Vec<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelRecordCounts {
    pub subjects: usize,
    pub anchors: usize,
    pub claims: usize,
    pub evidence: usize,
    pub postures: usize,
    pub drift: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelPostureSummary {
    pub aggregate_posture: DoppelPostureKind,
    pub verified_count: usize,
    pub stale_count: usize,
    pub blocked_count: usize,
    pub partial_count: usize,
    pub unknown_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelManifestRecord {
    pub manifest_id: String,
    pub repo_id: String,
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
