//! Differential scan tests (Phase 6): diff two real manifest bundles built by
//! the full extraction → intake → derivation → manifest pipeline.

use chrono::{DateTime, Utc};

use doppelcore::comparison::DoppelDeltaKind;
use doppelcore::contracts::{
    DoppelAnchorKind, DoppelDeterminismClass, DoppelProfileId, DoppelSubjectKind,
};
use doppelcore::extraction::{
    DoppelExtractedAnchor, DoppelExtractedSubject, DoppelExtractionPacket,
    DoppelExtractionPacketKind, DoppelExtractionProvenance,
};
use doppelcore::{
    assemble_manifest_bundle, build_history_entry, compare_manifests, derive_truth,
    normalize_extraction_packet, DoppelManifestBundle,
};

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

/// Build a manifest bundle from a set of `(subject_external, anchor_external,
/// content_hash)` tuples, running the whole pipeline.
fn bundle(run_id: &str, entries: &[(&str, &str, &str)]) -> DoppelManifestBundle {
    let provenance = DoppelExtractionProvenance {
        run_id: run_id.into(),
        repo_id: "repo-1".into(),
        revision_ref: format!("rev-{run_id}"),
        profile_id: DoppelProfileId::RouteServiceSliceV1,
        slice_id: "slice-1".into(),
        producer: "cortex".into(),
        extracted_at: ts(),
    };

    let subjects = entries
        .iter()
        .map(|(subject, _, _)| DoppelExtractedSubject {
            external_id: (*subject).into(),
            subject_kind: DoppelSubjectKind::Route,
            repo_path: Some(format!("src/routes/{subject}.rs")),
            display_name: format!("GET /{subject}"),
        })
        .collect();

    let anchors = entries
        .iter()
        .map(|(subject, anchor, hash)| DoppelExtractedAnchor {
            external_id: (*anchor).into(),
            subject_external_id: (*subject).into(),
            anchor_kind: DoppelAnchorKind::Function,
            path: format!("src/routes/{subject}.rs"),
            line_start: Some(10),
            line_end: Some(24),
            symbol_name: Some((*anchor).into()),
            content_hash: (*hash).into(),
        })
        .collect();

    let packet = DoppelExtractionPacket {
        packet_id: format!("packet-{run_id}"),
        packet_kind: DoppelExtractionPacketKind::RouteRegistration,
        provenance: provenance.clone(),
        subjects,
        anchors,
        probe_method: "ast_scan".into(),
        determinism_class: DoppelDeterminismClass::Deterministic,
    };

    let normalized = normalize_extraction_packet(&packet).unwrap();
    let truth = derive_truth(&normalized, &[], ts());
    let inputs =
        doppelcore::DoppelManifestInputs::from_provenance(&provenance, "doppelcore", None, "0.1.0");
    assemble_manifest_bundle(&inputs, &normalized, &truth, ts())
}

#[test]
fn detects_added_subject_and_changed_and_added_anchors() {
    let baseline = bundle("run-1", &[("route-a", "fn-a", "h1")]);
    let candidate = bundle(
        "run-2",
        &[("route-a", "fn-a", "h2"), ("route-b", "fn-b", "h3")],
    );

    let comparison = compare_manifests(&baseline, &candidate);

    assert_eq!(comparison.added_subjects, vec!["subject:repo-1:route-b"]);
    assert!(comparison.removed_subjects.is_empty());

    // route-a's anchor changed hash; route-b's anchor is new.
    let changed: Vec<_> = comparison
        .anchor_deltas
        .iter()
        .filter(|d| d.change_kind == DoppelDeltaKind::Changed)
        .collect();
    let added: Vec<_> = comparison
        .anchor_deltas
        .iter()
        .filter(|d| d.change_kind == DoppelDeltaKind::Added)
        .collect();
    assert_eq!(changed.len(), 1);
    assert_eq!(changed[0].subject_id, "subject:repo-1:route-a");
    assert_eq!(added.len(), 1);
    assert_eq!(added[0].subject_id, "subject:repo-1:route-b");
}

#[test]
fn detects_removed_subject() {
    let baseline = bundle(
        "run-1",
        &[("route-a", "fn-a", "h1"), ("route-b", "fn-b", "h2")],
    );
    let candidate = bundle("run-2", &[("route-a", "fn-a", "h1")]);

    let comparison = compare_manifests(&baseline, &candidate);

    assert_eq!(comparison.removed_subjects, vec!["subject:repo-1:route-b"]);
    assert!(comparison.added_subjects.is_empty());
    // route-b's anchor is gone in the candidate.
    assert!(comparison
        .anchor_deltas
        .iter()
        .any(|d| d.change_kind == DoppelDeltaKind::Removed));
}

#[test]
fn identical_bundles_produce_no_deltas() {
    let baseline = bundle("run-1", &[("route-a", "fn-a", "h1")]);
    let candidate = bundle("run-2", &[("route-a", "fn-a", "h1")]);

    let comparison = compare_manifests(&baseline, &candidate);

    assert!(comparison.added_subjects.is_empty());
    assert!(comparison.removed_subjects.is_empty());
    assert!(comparison.anchor_deltas.is_empty());
    // Both bundles report the same "newly observed" drift, so it is unchanged.
    assert!(comparison.drift_deltas.is_empty());
}

#[test]
fn comparison_is_deterministic() {
    let baseline = bundle("run-1", &[("route-a", "fn-a", "h1")]);
    let candidate = bundle(
        "run-2",
        &[("route-a", "fn-a", "h2"), ("route-b", "fn-b", "h3")],
    );

    let first = compare_manifests(&baseline, &candidate);
    let second = compare_manifests(&baseline, &candidate);
    assert_eq!(first, second);
}

#[test]
fn history_entry_counts_anchor_drift() {
    let candidate = bundle("run-1", &[("route-a", "fn-a", "h1")]);

    let entry = build_history_entry(&candidate);
    assert_eq!(entry.manifest_id, "manifest:run-1");
    assert_eq!(entry.run_id, "run-1");
    // The first run reports its single anchor as added drift.
    assert_eq!(entry.drift_count, 1);
    assert_eq!(entry.anchor_drift_count, 1);
}
