use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{DoppelDriftKind, DoppelPostureKind, DoppelSubjectKind};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoppelDeltaKind {
    Added,
    Removed,
    Changed,
}

impl DoppelDeltaKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelHistoryEntry {
    pub manifest_id: String,
    pub run_id: String,
    pub compliance_run_id: Option<String>,
    pub generated_at: DateTime<Utc>,
    pub revision_ref: String,
    pub aggregate_posture: DoppelPostureKind,
    pub drift_count: usize,
    pub anchor_drift_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelAnchorDelta {
    pub subject_id: String,
    pub subject_label: String,
    pub subject_kind: DoppelSubjectKind,
    pub change_kind: DoppelDeltaKind,
    pub baseline_anchor_id: Option<String>,
    pub candidate_anchor_id: Option<String>,
    pub baseline_anchor_ref: Option<String>,
    pub candidate_anchor_ref: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelDriftDelta {
    pub subject_id: String,
    pub subject_label: String,
    pub drift_kind: DoppelDriftKind,
    pub change_kind: DoppelDeltaKind,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelManifestComparison {
    pub baseline_manifest_id: String,
    pub candidate_manifest_id: String,
    pub baseline_generated_at: DateTime<Utc>,
    pub candidate_generated_at: DateTime<Utc>,
    pub baseline_posture: DoppelPostureKind,
    pub candidate_posture: DoppelPostureKind,
    pub added_subjects: Vec<String>,
    pub removed_subjects: Vec<String>,
    pub anchor_deltas: Vec<DoppelAnchorDelta>,
    pub drift_deltas: Vec<DoppelDriftDelta>,
    pub summary: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_kind_wire_values_are_stable() {
        assert_eq!(DoppelDeltaKind::Added.as_str(), "added");
        assert_eq!(DoppelDeltaKind::Removed.as_str(), "removed");
        assert_eq!(DoppelDeltaKind::Changed.as_str(), "changed");
    }
}
