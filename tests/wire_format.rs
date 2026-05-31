//! Wire-format guarantees for the DoppelCore contract surface.
//!
//! DoppelCore's job is to be a *portable* contract surface: the JSON
//! representation of these records and enums is the actual interface other
//! systems depend on. These tests lock that representation down so a future
//! rename or reshape can't silently change the wire format.

use chrono::{DateTime, Utc};

use doppelcore::{
    DoppelAnchorDelta, DoppelAnchorKind, DoppelAnchorRecord, DoppelClaimRecord, DoppelClaimType,
    DoppelCorrectionEvaluationPacket, DoppelCorrectionExecutionLane,
    DoppelCorrectionOpportunityKind, DoppelCorrectionOpportunityRecord,
    DoppelCorrectionProposalSystem, DoppelCorrectionRiskPosture, DoppelCorrectionScoringInput,
    DoppelDeltaKind, DoppelDeterminismClass, DoppelDriftDelta, DoppelDriftKind, DoppelDriftRecord,
    DoppelEvidenceRecord, DoppelIntakeReceiptRecord, DoppelIntakeSourceKind,
    DoppelIntakeStatusKind, DoppelManifestBundle, DoppelManifestComparison, DoppelManifestRecord,
    DoppelPostureKind, DoppelPostureRecord, DoppelPostureSummary, DoppelProfileId,
    DoppelRecordCounts, DoppelSubjectKind, DoppelSubjectRecord, DoppelTruthClass,
};

fn ts(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).expect("valid timestamp")
}

/// Assert that a value serializes to an exact JSON token and round-trips back.
fn assert_wire<T>(value: T, expected_json: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(&value).expect("serialize");
    assert_eq!(json, expected_json, "unexpected wire format");

    let back: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, value, "value did not round-trip");
}

#[test]
fn contract_enums_use_snake_case_wire_values() {
    assert_wire(
        DoppelProfileId::RouteServiceSliceV1,
        "\"route_service_slice_v1\"",
    );
    assert_wire(DoppelSubjectKind::PersistencePath, "\"persistence_path\"");
    assert_wire(DoppelAnchorKind::EventVariant, "\"event_variant\"");
    assert_wire(
        DoppelClaimType::CanonicalOutputContract,
        "\"canonical_output_contract\"",
    );
    assert_wire(DoppelTruthClass::Mixed, "\"mixed\"");
    assert_wire(
        DoppelDeterminismClass::OperatorAsserted,
        "\"operator_asserted\"",
    );
    assert_wire(DoppelPostureKind::Unassessable, "\"unassessable\"");
    assert_wire(
        DoppelDriftKind::CanonicalArtifactFreshness,
        "\"canonical_artifact_freshness\"",
    );
    assert_wire(
        DoppelIntakeSourceKind::SelfHealingIncident,
        "\"self_healing_incident\"",
    );
    assert_wire(DoppelIntakeStatusKind::Duplicate, "\"duplicate\"");
}

#[test]
fn comparison_enums_use_snake_case_wire_values() {
    assert_wire(DoppelDeltaKind::Added, "\"added\"");
    assert_wire(DoppelDeltaKind::Removed, "\"removed\"");
    assert_wire(DoppelDeltaKind::Changed, "\"changed\"");
}

#[test]
fn correction_enums_use_snake_case_wire_values() {
    assert_wire(
        DoppelCorrectionOpportunityKind::CanonicalArtifactMissing,
        "\"canonical_artifact_missing\"",
    );
    assert_wire(DoppelCorrectionRiskPosture::Moderate, "\"moderate\"");
    assert_wire(
        DoppelCorrectionExecutionLane::AdvisoryOnly,
        "\"advisory_only\"",
    );
    assert_wire(DoppelCorrectionProposalSystem::ForgeHq, "\"forge_hq\"");
    assert_wire(
        DoppelCorrectionProposalSystem::EvalCalNode,
        "\"eval_cal_node\"",
    );
}

/// The serde wire value and the hand-written `as_str()` must never diverge.
#[test]
fn correction_as_str_matches_serde_wire_value() {
    let cases = [
        DoppelCorrectionOpportunityKind::CanonicalArtifactMissing,
        DoppelCorrectionOpportunityKind::ContractDrift,
        DoppelCorrectionOpportunityKind::IntakeProjectionGap,
        DoppelCorrectionOpportunityKind::PersistenceRepair,
    ];
    for case in cases {
        let serde_value = serde_json::to_string(&case).unwrap();
        assert_eq!(serde_value, format!("\"{}\"", case.as_str()));
    }
}

fn sample_subject() -> DoppelSubjectRecord {
    DoppelSubjectRecord {
        subject_id: "subject-1".into(),
        subject_kind: DoppelSubjectKind::Route,
        repo_id: "repo-1".into(),
        path: Some("src/routes/mod.rs".into()),
        display_name: "GET /health".into(),
        revision_ref: "abc123".into(),
        source_profile: "route_service_slice_v1".into(),
    }
}

fn sample_anchor() -> DoppelAnchorRecord {
    DoppelAnchorRecord {
        anchor_id: "anchor-1".into(),
        subject_id: "subject-1".into(),
        anchor_kind: DoppelAnchorKind::Function,
        path: "src/routes/health.rs".into(),
        line_start: Some(10),
        line_end: Some(24),
        symbol_name: Some("health_handler".into()),
        anchor_hash: "deadbeef".into(),
    }
}

fn sample_bundle() -> DoppelManifestBundle {
    DoppelManifestBundle {
        run_id: "run-1".into(),
        system_id: "doppelcore".into(),
        compliance_run_id: Some("compliance-1".into()),
        manifest: DoppelManifestRecord {
            manifest_id: "manifest-1".into(),
            repo_id: "repo-1".into(),
            revision_ref: "abc123".into(),
            profile_id: DoppelProfileId::RouteServiceSliceV1,
            slice_id: "slice-1".into(),
            record_counts: DoppelRecordCounts {
                subjects: 1,
                anchors: 1,
                claims: 1,
                evidence: 1,
                postures: 1,
                drift: 1,
            },
            posture_summary: DoppelPostureSummary {
                aggregate_posture: DoppelPostureKind::Verified,
                verified_count: 1,
                stale_count: 0,
                blocked_count: 0,
                partial_count: 0,
                unknown_count: 0,
            },
            generated_at: ts(1_700_000_000),
            generator_version: "0.1.0".into(),
            render_digest: "render-digest".into(),
        },
        subjects: vec![sample_subject()],
        anchors: vec![sample_anchor()],
        claims: vec![DoppelClaimRecord {
            claim_id: "claim-1".into(),
            subject_id: "subject-1".into(),
            claim_type: DoppelClaimType::RouteDelegate,
            statement: "route delegates to handler".into(),
            truth_class: DoppelTruthClass::Deterministic,
            posture: DoppelPostureKind::Verified,
            evidence_refs: vec!["evidence-1".into()],
            anchor_refs: vec!["anchor-1".into()],
            confidence_policy: "strict".into(),
        }],
        evidence: vec![DoppelEvidenceRecord {
            evidence_id: "evidence-1".into(),
            subject_id: Some("subject-1".into()),
            anchor_id: Some("anchor-1".into()),
            evidence_type: "static_analysis".into(),
            probe_method: "ast_scan".into(),
            source_ref: "src/routes/health.rs".into(),
            captured_at: ts(1_699_000_000),
            digest: "evidence-digest".into(),
            determinism_class: DoppelDeterminismClass::Deterministic,
            detail: None,
        }],
        postures: vec![DoppelPostureRecord {
            posture_id: "posture-1".into(),
            subject_id: Some("subject-1".into()),
            claim_id: Some("claim-1".into()),
            posture: DoppelPostureKind::Verified,
            basis: "evidence-backed".into(),
            updated_at: ts(1_700_000_000),
        }],
        drift: vec![DoppelDriftRecord {
            drift_id: "drift-1".into(),
            subject_id: "subject-1".into(),
            anchor_id: Some("anchor-1".into()),
            drift_kind: DoppelDriftKind::AnchorChanged,
            posture: DoppelPostureKind::Stale,
            summary: "anchor moved".into(),
            detected_at: ts(1_700_500_000),
            evidence_refs: vec!["evidence-1".into()],
            detail: Some("line range shifted".into()),
        }],
        rendered_markdown: "# Manifest\n".into(),
    }
}

#[test]
fn manifest_bundle_round_trips() {
    let bundle = sample_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize bundle");
    let back: DoppelManifestBundle = serde_json::from_str(&json).expect("deserialize bundle");
    assert_eq!(back, bundle);
}

#[test]
fn manifest_comparison_round_trips() {
    let comparison = DoppelManifestComparison {
        baseline_manifest_id: "manifest-0".into(),
        candidate_manifest_id: "manifest-1".into(),
        baseline_generated_at: ts(1_699_000_000),
        candidate_generated_at: ts(1_700_000_000),
        baseline_posture: DoppelPostureKind::Verified,
        candidate_posture: DoppelPostureKind::Partial,
        added_subjects: vec!["subject-2".into()],
        removed_subjects: vec![],
        anchor_deltas: vec![DoppelAnchorDelta {
            subject_id: "subject-1".into(),
            subject_label: "GET /health".into(),
            subject_kind: DoppelSubjectKind::Route,
            change_kind: DoppelDeltaKind::Changed,
            baseline_anchor_id: Some("anchor-0".into()),
            candidate_anchor_id: Some("anchor-1".into()),
            baseline_anchor_ref: Some("src/routes/health.rs:8".into()),
            candidate_anchor_ref: Some("src/routes/health.rs:10".into()),
            summary: "anchor moved".into(),
        }],
        drift_deltas: vec![DoppelDriftDelta {
            subject_id: "subject-1".into(),
            subject_label: "GET /health".into(),
            drift_kind: DoppelDriftKind::AnchorChanged,
            change_kind: DoppelDeltaKind::Added,
            summary: "new drift detected".into(),
        }],
        summary: "1 subject added, 1 anchor changed".into(),
    };

    let json = serde_json::to_string(&comparison).expect("serialize");
    let back: DoppelManifestComparison = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, comparison);
}

#[test]
fn correction_evaluation_packet_round_trips() {
    let packet = DoppelCorrectionEvaluationPacket {
        packet_id: "packet-1".into(),
        opportunity: DoppelCorrectionOpportunityRecord {
            opportunity_id: "opp-1".into(),
            manifest_id: "manifest-1".into(),
            repo_id: "repo-1".into(),
            revision_ref: "abc123".into(),
            subject_id: "subject-1".into(),
            subject_kind: DoppelSubjectKind::DocumentationArtifact,
            claim_id: Some("claim-1".into()),
            drift_id: None,
            opportunity_kind: DoppelCorrectionOpportunityKind::CanonicalArtifactStale,
            target_path: Some("docs/manifest.json".into()),
            summary: "regenerate stale artifact".into(),
            rationale: "artifact older than source".into(),
            evidence_refs: vec!["evidence-1".into()],
            source_posture: DoppelPostureKind::Stale,
            risk_posture: DoppelCorrectionRiskPosture::Low,
            recommended_lane: DoppelCorrectionExecutionLane::LocalProposal,
            created_at: ts(1_700_000_000),
            valid_until: Some(ts(1_700_600_000)),
        },
        scoring_input: DoppelCorrectionScoringInput::default(),
        candidate_systems: vec![
            DoppelCorrectionProposalSystem::ForgeEval,
            DoppelCorrectionProposalSystem::ManualOperator,
        ],
        calibration_profile: "balanced_v1".into(),
        created_at: ts(1_700_000_000),
    };

    let json = serde_json::to_string(&packet).expect("serialize");
    let back: DoppelCorrectionEvaluationPacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, packet);
}

#[test]
fn intake_receipt_round_trips_and_detail_json_is_parseable() {
    let receipt = DoppelIntakeReceiptRecord {
        doppel_intake_receipt_id: "receipt-1".into(),
        intake_source: DoppelIntakeSourceKind::SelfHealingIncident,
        source_incident_id: "incident-1".into(),
        source_projection_receipt_id: Some("proj-receipt-1".into()),
        source_projection_id: None,
        system_id: "doppelcore".into(),
        status: DoppelIntakeStatusKind::Accepted,
        decision_basis: "incident maps to known subject".into(),
        created_at: ts(1_700_000_000),
        detail_json: "{\"reason\":\"matched\",\"score\":0.92}".into(),
    };

    let json = serde_json::to_string(&receipt).expect("serialize");
    let back: DoppelIntakeReceiptRecord = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, receipt);

    // The `detail_json` field carries an embedded JSON document; it must be
    // independently parseable so consumers can read structured detail.
    let detail: serde_json::Value =
        serde_json::from_str(&back.detail_json).expect("detail_json is valid JSON");
    assert_eq!(detail["reason"], "matched");
}
