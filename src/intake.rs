//! Extraction → canonical record normalization (Phase 2, Canvas 04 Stage 2).
//!
//! This is the boundary where DoppelCore crosses from upstream *extraction
//! evidence* into *machine-truth representation* (Canvas 06 §5). It takes a
//! bounded [`DoppelExtractionPacket`] and normalizes it into canonical
//! [`DoppelSubjectRecord`], [`DoppelAnchorRecord`], and [`DoppelEvidenceRecord`]
//! values.
//!
//! Scope is deliberately bounded: this stage forms subjects, anchors, and
//! evidence only. It does **not** derive claims, posture, or drift — those are
//! later phases (Canvas 08, Phase 3+). It also never invents data: an anchor
//! that points at a subject the packet did not declare is an error, not a
//! silently dropped record.
//!
//! Canonical ids are derived deterministically from provenance so that re-runs
//! over the same revision produce stable subject and anchor ids (the basis for
//! later drift tracking), while evidence ids are scoped per run (Canvas 04,
//! Rule 1 — evidence is immutable per run).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::{DoppelCoreError, Result};
use crate::extraction::DoppelExtractionPacket;
use crate::records::{DoppelAnchorRecord, DoppelEvidenceRecord, DoppelSubjectRecord};

/// The canonical records produced by normalizing an extraction packet.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelNormalizedRecords {
    pub subjects: Vec<DoppelSubjectRecord>,
    pub anchors: Vec<DoppelAnchorRecord>,
    pub evidence: Vec<DoppelEvidenceRecord>,
}

/// Stable canonical subject id (consistent across runs of the same repo).
fn canonical_subject_id(repo_id: &str, external_id: &str) -> String {
    format!("subject:{repo_id}:{external_id}")
}

/// Stable canonical anchor id (consistent across runs).
fn canonical_anchor_id(subject_id: &str, external_id: &str) -> String {
    format!("anchor:{subject_id}:{external_id}")
}

/// Run-scoped evidence id (Canvas 04, Rule 1 — evidence is immutable per run).
fn evidence_id(run_id: &str, anchor_external_id: &str) -> String {
    format!("evidence:{run_id}:{anchor_external_id}")
}

/// Normalize a single bounded extraction packet into canonical subjects,
/// anchors, and evidence.
///
/// # Errors
///
/// Returns [`DoppelCoreError::DanglingExtractionAnchor`] if an extracted anchor
/// references a `subject_external_id` that the packet does not declare.
pub fn normalize_extraction_packet(
    packet: &DoppelExtractionPacket,
) -> Result<DoppelNormalizedRecords> {
    let provenance = &packet.provenance;
    let source_profile = provenance.profile_id.as_str().to_string();

    // Map each extracted subject's external id to its canonical id so anchors
    // and evidence can link back to it.
    let mut canonical_by_external: HashMap<&str, String> =
        HashMap::with_capacity(packet.subjects.len());

    let mut subjects = Vec::with_capacity(packet.subjects.len());
    for extracted in &packet.subjects {
        let subject_id = canonical_subject_id(&provenance.repo_id, &extracted.external_id);
        canonical_by_external.insert(extracted.external_id.as_str(), subject_id.clone());

        subjects.push(DoppelSubjectRecord {
            subject_id,
            subject_kind: extracted.subject_kind,
            repo_id: provenance.repo_id.clone(),
            path: extracted.repo_path.clone(),
            display_name: extracted.display_name.clone(),
            revision_ref: provenance.revision_ref.clone(),
            source_profile: source_profile.clone(),
        });
    }

    let mut anchors = Vec::with_capacity(packet.anchors.len());
    let mut evidence = Vec::with_capacity(packet.anchors.len());
    for extracted in &packet.anchors {
        let subject_id = canonical_by_external
            .get(extracted.subject_external_id.as_str())
            .ok_or_else(|| DoppelCoreError::DanglingExtractionAnchor {
                anchor: extracted.external_id.clone(),
                subject: extracted.subject_external_id.clone(),
            })?
            .clone();

        let anchor_id = canonical_anchor_id(&subject_id, &extracted.external_id);

        anchors.push(DoppelAnchorRecord {
            anchor_id: anchor_id.clone(),
            subject_id: subject_id.clone(),
            anchor_kind: extracted.anchor_kind,
            path: extracted.path.clone(),
            line_start: extracted.line_start,
            line_end: extracted.line_end,
            symbol_name: extracted.symbol_name.clone(),
            anchor_hash: extracted.content_hash.clone(),
        });

        // The extraction observation itself is the first-class evidence that
        // backs this anchor. Source refs are mandatory (Canvas 04, Rule 2).
        evidence.push(DoppelEvidenceRecord {
            evidence_id: evidence_id(&provenance.run_id, &extracted.external_id),
            subject_id: Some(subject_id),
            anchor_id: Some(anchor_id),
            evidence_type: packet.packet_kind.as_str().to_string(),
            probe_method: packet.probe_method.clone(),
            source_ref: extracted.path.clone(),
            captured_at: provenance.extracted_at,
            digest: extracted.content_hash.clone(),
            determinism_class: packet.determinism_class,
            detail: extracted.symbol_name.clone(),
        });
    }

    Ok(DoppelNormalizedRecords {
        subjects,
        anchors,
        evidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        DoppelAnchorKind, DoppelDeterminismClass, DoppelProfileId, DoppelSubjectKind,
    };
    use crate::extraction::{
        DoppelExtractedAnchor, DoppelExtractedSubject, DoppelExtractionPacketKind,
        DoppelExtractionProvenance,
    };
    use chrono::{DateTime, Utc};

    fn ts() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn provenance(run_id: &str) -> DoppelExtractionProvenance {
        DoppelExtractionProvenance {
            run_id: run_id.into(),
            repo_id: "repo-1".into(),
            revision_ref: "abc123".into(),
            profile_id: DoppelProfileId::RouteServiceSliceV1,
            slice_id: "slice-1".into(),
            producer: "cortex".into(),
            extracted_at: ts(),
        }
    }

    fn sample_packet(run_id: &str) -> DoppelExtractionPacket {
        DoppelExtractionPacket {
            packet_id: "packet-1".into(),
            packet_kind: DoppelExtractionPacketKind::RouteRegistration,
            provenance: provenance(run_id),
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
                content_hash: "deadbeef".into(),
            }],
            probe_method: "ast_scan".into(),
            determinism_class: DoppelDeterminismClass::Deterministic,
        }
    }

    #[test]
    fn normalizes_subjects_anchors_and_evidence() {
        let out = normalize_extraction_packet(&sample_packet("run-1")).unwrap();

        assert_eq!(out.subjects.len(), 1);
        assert_eq!(out.anchors.len(), 1);
        assert_eq!(out.evidence.len(), 1);

        let subject = &out.subjects[0];
        assert_eq!(subject.subject_id, "subject:repo-1:route-health");
        assert_eq!(subject.revision_ref, "abc123");
        assert_eq!(subject.source_profile, "route_service_slice_v1");

        let anchor = &out.anchors[0];
        assert_eq!(anchor.subject_id, subject.subject_id);
        assert_eq!(
            anchor.anchor_id,
            "anchor:subject:repo-1:route-health:fn-health-handler"
        );
        assert_eq!(anchor.anchor_hash, "deadbeef");

        // Evidence links back to both the subject and the anchor it backs.
        let evidence = &out.evidence[0];
        assert_eq!(
            evidence.subject_id.as_deref(),
            Some(subject.subject_id.as_str())
        );
        assert_eq!(
            evidence.anchor_id.as_deref(),
            Some(anchor.anchor_id.as_str())
        );
        assert_eq!(evidence.evidence_type, "route_registration");
        assert_eq!(evidence.digest, "deadbeef");
    }

    #[test]
    fn subject_and_anchor_ids_are_stable_across_runs() {
        let first = normalize_extraction_packet(&sample_packet("run-1")).unwrap();
        let second = normalize_extraction_packet(&sample_packet("run-2")).unwrap();

        // Subjects and anchors are stable across runs (basis for drift tracking)...
        assert_eq!(first.subjects[0].subject_id, second.subjects[0].subject_id);
        assert_eq!(first.anchors[0].anchor_id, second.anchors[0].anchor_id);

        // ...while evidence is scoped per run (immutable per run).
        assert_ne!(
            first.evidence[0].evidence_id,
            second.evidence[0].evidence_id
        );
    }

    #[test]
    fn dangling_anchor_subject_is_an_error() {
        let mut packet = sample_packet("run-1");
        packet.anchors[0].subject_external_id = "does-not-exist".into();

        let err = normalize_extraction_packet(&packet).unwrap_err();
        match err {
            DoppelCoreError::DanglingExtractionAnchor { anchor, subject } => {
                assert_eq!(anchor, "fn-health-handler");
                assert_eq!(subject, "does-not-exist");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
