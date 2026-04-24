use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{DoppelPostureKind, DoppelSubjectKind};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoppelCorrectionOpportunityKind {
    CanonicalArtifactMissing,
    CanonicalArtifactStale,
    ContractDrift,
    EvidenceConflict,
    IntakeProjectionGap,
    DocumentationRepair,
    RouteServiceRepair,
    PersistenceRepair,
}

impl DoppelCorrectionOpportunityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalArtifactMissing => "canonical_artifact_missing",
            Self::CanonicalArtifactStale => "canonical_artifact_stale",
            Self::ContractDrift => "contract_drift",
            Self::EvidenceConflict => "evidence_conflict",
            Self::IntakeProjectionGap => "intake_projection_gap",
            Self::DocumentationRepair => "documentation_repair",
            Self::RouteServiceRepair => "route_service_repair",
            Self::PersistenceRepair => "persistence_repair",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoppelCorrectionRiskPosture {
    Low,
    Moderate,
    High,
    Blocked,
}

impl DoppelCorrectionRiskPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoppelCorrectionExecutionLane {
    AdvisoryOnly,
    LocalProposal,
    CloudProposal,
    ManualOperator,
}

impl DoppelCorrectionExecutionLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryOnly => "advisory_only",
            Self::LocalProposal => "local_proposal",
            Self::CloudProposal => "cloud_proposal",
            Self::ManualOperator => "manual_operator",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DoppelCorrectionProposalSystem {
    ForgeHq,
    ForgeMath,
    ForgeEval,
    EvalCalNode,
    ManualOperator,
}

impl DoppelCorrectionProposalSystem {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForgeHq => "forge_hq",
            Self::ForgeMath => "forge_math",
            Self::ForgeEval => "forge_eval",
            Self::EvalCalNode => "eval_cal_node",
            Self::ManualOperator => "manual_operator",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoppelCorrectionScoringInput {
    pub correctness_weight: f32,
    pub safety_weight: f32,
    pub reversibility_weight: f32,
    pub evidence_strength_weight: f32,
    pub operator_priority_weight: f32,
}

impl Default for DoppelCorrectionScoringInput {
    fn default() -> Self {
        Self {
            correctness_weight: 1.0,
            safety_weight: 1.0,
            reversibility_weight: 1.0,
            evidence_strength_weight: 1.0,
            operator_priority_weight: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoppelCorrectionOpportunityRecord {
    pub opportunity_id: String,
    pub manifest_id: String,
    pub repo_id: String,
    pub revision_ref: String,
    pub subject_id: String,
    pub subject_kind: DoppelSubjectKind,
    pub claim_id: Option<String>,
    pub drift_id: Option<String>,
    pub opportunity_kind: DoppelCorrectionOpportunityKind,
    pub target_path: Option<String>,
    pub summary: String,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub source_posture: DoppelPostureKind,
    pub risk_posture: DoppelCorrectionRiskPosture,
    pub recommended_lane: DoppelCorrectionExecutionLane,
    pub created_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoppelCorrectionEvaluationPacket {
    pub packet_id: String,
    pub opportunity: DoppelCorrectionOpportunityRecord,
    pub scoring_input: DoppelCorrectionScoringInput,
    pub candidate_systems: Vec<DoppelCorrectionProposalSystem>,
    pub calibration_profile: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correction_enum_wire_values_are_stable() {
        assert_eq!(
            DoppelCorrectionOpportunityKind::ContractDrift.as_str(),
            "contract_drift"
        );
        assert_eq!(DoppelCorrectionRiskPosture::Blocked.as_str(), "blocked");
        assert_eq!(
            DoppelCorrectionExecutionLane::LocalProposal.as_str(),
            "local_proposal"
        );
        assert_eq!(
            DoppelCorrectionProposalSystem::ForgeEval.as_str(),
            "forge_eval"
        );
    }

    #[test]
    fn default_scoring_weights_are_balanced() {
        let scoring = DoppelCorrectionScoringInput::default();

        assert_eq!(scoring.correctness_weight, 1.0);
        assert_eq!(scoring.safety_weight, 1.0);
        assert_eq!(scoring.reversibility_weight, 1.0);
        assert_eq!(scoring.evidence_strength_weight, 1.0);
        assert_eq!(scoring.operator_priority_weight, 1.0);
    }
}
