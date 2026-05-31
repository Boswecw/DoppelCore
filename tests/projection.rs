//! Rendered projection tests (Phase 5): project a real pipeline-built manifest
//! bundle into a review view and Markdown, proving the render is downstream of
//! the records and never disagrees with machine posture.

use chrono::{DateTime, Utc};

use doppelcore::contracts::{
    DoppelAnchorKind, DoppelDeterminismClass, DoppelPostureKind, DoppelProfileId, DoppelSubjectKind,
};
use doppelcore::extraction::{
    DoppelExtractedAnchor, DoppelExtractedSubject, DoppelExtractionPacket,
    DoppelExtractionPacketKind, DoppelExtractionProvenance,
};
use doppelcore::{
    assemble_manifest_bundle, derive_truth, normalize_extraction_packet, project_review,
    render_review_markdown, DoppelManifestBundle, DoppelManifestInputs,
};

fn ts() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).unwrap()
}

fn bundle() -> DoppelManifestBundle {
    let provenance = DoppelExtractionProvenance {
        run_id: "run-1".into(),
        repo_id: "repo-1".into(),
        revision_ref: "abc123".into(),
        profile_id: DoppelProfileId::RouteServiceSliceV1,
        slice_id: "slice-1".into(),
        producer: "cortex".into(),
        extracted_at: ts(),
    };
    let packet = DoppelExtractionPacket {
        packet_id: "packet-1".into(),
        packet_kind: DoppelExtractionPacketKind::RouteRegistration,
        provenance: provenance.clone(),
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
            content_hash: "hash-1".into(),
        }],
        probe_method: "ast_scan".into(),
        determinism_class: DoppelDeterminismClass::Deterministic,
    };

    let normalized = normalize_extraction_packet(&packet).unwrap();
    let truth = derive_truth(&normalized, &[], ts());
    let inputs = DoppelManifestInputs::from_provenance(&provenance, "doppelcore", None, "0.1.0");
    assemble_manifest_bundle(&inputs, &normalized, &truth, ts())
}

#[test]
fn projection_reports_machine_posture_without_recomputing() {
    let bundle = bundle();
    let projection = project_review(&bundle);

    // Headline posture is exactly what the manifest derived.
    assert_eq!(
        projection.headline_posture,
        bundle.manifest.posture_summary.aggregate_posture
    );
    assert_eq!(projection.subjects.len(), 1);

    let subject = &projection.subjects[0];
    assert_eq!(subject.subject_id, "subject:repo-1:route-health");
    assert_eq!(subject.posture, DoppelPostureKind::Verified);
    assert_eq!(subject.claims.len(), 1);
    assert!(subject.claims[0].statement.contains("health_handler"));

    // The projection ties back to the exact bundle it came from.
    assert_eq!(
        projection.source_render_digest,
        bundle.manifest.render_digest
    );
}

#[test]
fn markdown_shows_posture_and_traceability_footer() {
    let projection = project_review(&bundle());
    let md = render_review_markdown(&projection);

    // Posture is shown prominently (no render inversion).
    assert!(md.contains("headline posture: **verified**"));
    assert!(md.contains("[verified]"));
    assert!(md.contains("health_handler"));
    // Footer asserts traceability to records (Canvas 09 Rule 3).
    assert!(md.contains("Posture is reported from records, not computed in this view"));
    assert!(md.contains(&projection.source_render_digest));
}

#[test]
fn projection_and_render_are_deterministic() {
    let bundle = bundle();
    assert_eq!(project_review(&bundle), project_review(&bundle));

    let md1 = render_review_markdown(&project_review(&bundle));
    let md2 = render_review_markdown(&project_review(&bundle));
    assert_eq!(md1, md2);
}
