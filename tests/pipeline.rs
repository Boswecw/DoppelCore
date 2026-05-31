//! End-to-end proving slice: a Cortex extraction packet flows through intake
//! normalization (Phase 2) and claim/posture/drift derivation (Phase 3) into
//! machine truth — Canvas 04 Stages 1→4, the "first real machine-truth
//! emission" of Canvas 08 Phase 3.

use chrono::{DateTime, Utc};

use doppelcore::contracts::{
    DoppelAnchorKind, DoppelClaimType, DoppelDeterminismClass, DoppelPostureKind, DoppelProfileId,
    DoppelSubjectKind,
};
use doppelcore::extraction::{
    DoppelExtractedAnchor, DoppelExtractedSubject, DoppelExtractionPacket,
    DoppelExtractionPacketKind, DoppelExtractionProvenance,
};
use doppelcore::{
    assemble_manifest_bundle, derive_truth, normalize_extraction_packet, DoppelManifestInputs,
};

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

fn route_packet(run_id: &str, handler_hash: &str) -> DoppelExtractionPacket {
    DoppelExtractionPacket {
        packet_id: "packet-1".into(),
        packet_kind: DoppelExtractionPacketKind::RouteRegistration,
        provenance: DoppelExtractionProvenance {
            run_id: run_id.into(),
            repo_id: "repo-1".into(),
            revision_ref: "abc123".into(),
            profile_id: DoppelProfileId::RouteServiceSliceV1,
            slice_id: "slice-1".into(),
            producer: "cortex".into(),
            extracted_at: ts(),
        },
        subjects: vec![DoppelExtractedSubject {
            external_id: "route-health".into(),
            subject_kind: DoppelSubjectKind::Route,
            repo_path: Some("src/routes/health.rs".into()),
            display_name: "GET /health".into(),
        }],
        anchors: vec![DoppelExtractedAnchor {
            external_id: "fn-health-handler".into(),
            subject_external_id: "route-health".into(),
            anchor_kind: DoppelAnchorKind::Function,
            path: "src/routes/health.rs".into(),
            line_start: Some(10),
            line_end: Some(24),
            symbol_name: Some("health_handler".into()),
            content_hash: handler_hash.into(),
        }],
        probe_method: "ast_scan".into(),
        determinism_class: DoppelDeterminismClass::Deterministic,
    }
}

#[test]
fn extraction_packet_becomes_verified_machine_truth() {
    let normalized = normalize_extraction_packet(&route_packet("run-1", "hash-1")).unwrap();

    // First run has no baseline, so every anchor is reported as added drift.
    let truth = derive_truth(&normalized, &[], ts());

    assert_eq!(truth.claims.len(), 1);
    let claim = &truth.claims[0];
    assert_eq!(claim.claim_type, DoppelClaimType::RouteDelegate);
    assert_eq!(claim.posture, DoppelPostureKind::Verified);
    // The claim cites the evidence and anchor produced by intake (Rule 1 & 2).
    assert_eq!(claim.evidence_refs.len(), 1);
    assert_eq!(claim.anchor_refs.len(), 1);

    assert_eq!(
        truth.posture_summary.aggregate_posture,
        DoppelPostureKind::Verified
    );
}

#[test]
fn changed_handler_across_runs_surfaces_stale_drift() {
    let baseline = normalize_extraction_packet(&route_packet("run-1", "hash-1")).unwrap();
    let candidate = normalize_extraction_packet(&route_packet("run-2", "hash-2")).unwrap();

    // Compare the new run against the prior run's anchors.
    let truth = derive_truth(&candidate, &baseline.anchors, ts());

    assert_eq!(truth.drift.len(), 1);
    let drift = &truth.drift[0];
    assert_eq!(drift.drift_kind, doppelcore::DoppelDriftKind::AnchorChanged);
    assert_eq!(drift.posture, DoppelPostureKind::Stale);
}

#[test]
fn full_pipeline_emits_a_manifest_bundle() {
    let packet = route_packet("run-1", "hash-1");
    let normalized = normalize_extraction_packet(&packet).unwrap();
    let truth = derive_truth(&normalized, &[], ts());

    let inputs = DoppelManifestInputs::from_provenance(
        &packet.provenance,
        "doppelcore",
        Some("compliance-1".into()),
        "0.1.0",
    );
    let bundle = assemble_manifest_bundle(&inputs, &normalized, &truth, ts());

    // Canvas 09 Rule 2: the machine bundle has a manifest with accurate counts.
    assert_eq!(bundle.manifest.manifest_id, "manifest:run-1");
    assert_eq!(bundle.manifest.record_counts.subjects, 1);
    assert_eq!(bundle.manifest.record_counts.claims, 1);
    assert_eq!(
        bundle.manifest.posture_summary.aggregate_posture,
        DoppelPostureKind::Verified
    );
    assert!(!bundle.manifest.render_digest.is_empty());
    assert!(bundle.rendered_markdown.contains("health_handler"));
}
