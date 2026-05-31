//! Manifest assembly — machine-bundle emission (Canvas 04 Stage 5).
//!
//! This stage wraps the normalized records ([`crate::intake`]) and the derived
//! truth ([`crate::derivation`]) into a single [`DoppelManifestBundle`] that
//! Registry can store, compare, or publish. It is the point that satisfies
//! Canvas 09 Rule 2 — *every machine bundle must have a manifest*.
//!
//! It also produces a deterministic Markdown render of the bundle. The render
//! is strictly downstream of the records (Canvas 09 Rule 3 — *every rendered
//! output must be traceable to machine records*) and adds no fact that is not
//! already present in a claim, drift, or posture record. Richer projection
//! (review packets, UI) is a later phase; this is the traceable seed.
//!
//! As with derivation, no clock is read here — `generated_at` is supplied by
//! the caller so emission is deterministic and testable.

use std::fmt::Write as _;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::DoppelProfileId;
use crate::derivation::DoppelDerivedTruth;
use crate::extraction::DoppelExtractionProvenance;
use crate::intake::DoppelNormalizedRecords;
use crate::records::{DoppelManifestBundle, DoppelManifestRecord, DoppelRecordCounts};

/// Identity and provenance a manifest needs that is not derivable from the
/// records themselves.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelManifestInputs {
    pub run_id: String,
    pub system_id: String,
    pub compliance_run_id: Option<String>,
    pub repo_id: String,
    pub revision_ref: String,
    pub profile_id: DoppelProfileId,
    pub slice_id: String,
    pub generator_version: String,
}

impl DoppelManifestInputs {
    /// Build manifest inputs from the extraction provenance that produced the
    /// scan, supplying only the fields provenance does not carry.
    pub fn from_provenance(
        provenance: &DoppelExtractionProvenance,
        system_id: impl Into<String>,
        compliance_run_id: Option<String>,
        generator_version: impl Into<String>,
    ) -> Self {
        Self {
            run_id: provenance.run_id.clone(),
            system_id: system_id.into(),
            compliance_run_id,
            repo_id: provenance.repo_id.clone(),
            revision_ref: provenance.revision_ref.clone(),
            profile_id: provenance.profile_id,
            slice_id: provenance.slice_id.clone(),
            generator_version: generator_version.into(),
        }
    }
}

/// Non-cryptographic, deterministic, portable content digest (FNV-1a, 64-bit).
///
/// Used as the manifest's `render_digest` so two emissions of the same rendered
/// bundle produce the same digest across platforms and runs.
fn content_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Render a deterministic Markdown view of the bundle. Every line traces back
/// to a record; no severity, posture, or fact is invented here.
fn render_markdown(
    inputs: &DoppelManifestInputs,
    counts: &DoppelRecordCounts,
    truth: &DoppelDerivedTruth,
) -> String {
    let summary = &truth.posture_summary;
    let mut out = String::new();

    let _ = writeln!(out, "# DoppelCore manifest: manifest:{}", inputs.run_id);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- repo: `{}` @ `{}`",
        inputs.repo_id, inputs.revision_ref
    );
    let _ = writeln!(out, "- profile: `{}`", inputs.profile_id.as_str());
    let _ = writeln!(out, "- slice: `{}`", inputs.slice_id);
    let _ = writeln!(
        out,
        "- aggregate posture: **{}**",
        summary.aggregate_posture.as_str()
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "## Counts");
    let _ = writeln!(
        out,
        "- subjects: {}, anchors: {}, claims: {}, evidence: {}, postures: {}, drift: {}",
        counts.subjects,
        counts.anchors,
        counts.claims,
        counts.evidence,
        counts.postures,
        counts.drift,
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "## Claims");
    if truth.claims.is_empty() {
        let _ = writeln!(out, "- (none)");
    } else {
        for claim in &truth.claims {
            let _ = writeln!(out, "- [{}] {}", claim.posture.as_str(), claim.statement);
        }
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "## Drift");
    if truth.drift.is_empty() {
        let _ = writeln!(out, "- (none)");
    } else {
        for drift in &truth.drift {
            let _ = writeln!(out, "- [{}] {}", drift.drift_kind.as_str(), drift.summary);
        }
    }

    out
}

/// Assemble a [`DoppelManifestBundle`] from normalized records and derived
/// truth.
///
/// Record counts and the posture summary come straight from the inputs — the
/// manifest never recomputes posture, only reports what derivation decided.
pub fn assemble_manifest_bundle(
    inputs: &DoppelManifestInputs,
    normalized: &DoppelNormalizedRecords,
    truth: &DoppelDerivedTruth,
    generated_at: DateTime<Utc>,
) -> DoppelManifestBundle {
    let record_counts = DoppelRecordCounts {
        subjects: normalized.subjects.len(),
        anchors: normalized.anchors.len(),
        claims: truth.claims.len(),
        evidence: normalized.evidence.len(),
        postures: truth.postures.len(),
        drift: truth.drift.len(),
    };

    let rendered_markdown = render_markdown(inputs, &record_counts, truth);
    let render_digest = content_digest(&rendered_markdown);

    let manifest = DoppelManifestRecord {
        manifest_id: format!("manifest:{}", inputs.run_id),
        repo_id: inputs.repo_id.clone(),
        revision_ref: inputs.revision_ref.clone(),
        profile_id: inputs.profile_id,
        slice_id: inputs.slice_id.clone(),
        record_counts,
        posture_summary: truth.posture_summary.clone(),
        generated_at,
        generator_version: inputs.generator_version.clone(),
        render_digest,
    };

    DoppelManifestBundle {
        run_id: inputs.run_id.clone(),
        system_id: inputs.system_id.clone(),
        compliance_run_id: inputs.compliance_run_id.clone(),
        manifest,
        subjects: normalized.subjects.clone(),
        anchors: normalized.anchors.clone(),
        claims: truth.claims.clone(),
        evidence: normalized.evidence.clone(),
        postures: truth.postures.clone(),
        drift: truth.drift.clone(),
        rendered_markdown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        DoppelAnchorKind, DoppelClaimType, DoppelPostureKind, DoppelSubjectKind, DoppelTruthClass,
    };
    use crate::records::{
        DoppelAnchorRecord, DoppelClaimRecord, DoppelPostureRecord, DoppelPostureSummary,
        DoppelSubjectRecord,
    };

    fn ts() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn inputs() -> DoppelManifestInputs {
        DoppelManifestInputs {
            run_id: "run-1".into(),
            system_id: "doppelcore".into(),
            compliance_run_id: Some("compliance-1".into()),
            repo_id: "repo-1".into(),
            revision_ref: "abc123".into(),
            profile_id: DoppelProfileId::RouteServiceSliceV1,
            slice_id: "slice-1".into(),
            generator_version: "0.1.0".into(),
        }
    }

    fn normalized() -> DoppelNormalizedRecords {
        DoppelNormalizedRecords {
            subjects: vec![DoppelSubjectRecord {
                subject_id: "s1".into(),
                subject_kind: DoppelSubjectKind::Route,
                repo_id: "repo-1".into(),
                path: Some("src/routes/health.rs".into()),
                display_name: "GET /health".into(),
                revision_ref: "abc123".into(),
                source_profile: "route_service_slice_v1".into(),
            }],
            anchors: vec![DoppelAnchorRecord {
                anchor_id: "a1".into(),
                subject_id: "s1".into(),
                anchor_kind: DoppelAnchorKind::Function,
                path: "src/routes/health.rs".into(),
                line_start: Some(10),
                line_end: Some(24),
                symbol_name: Some("health_handler".into()),
                anchor_hash: "hash-1".into(),
            }],
            evidence: vec![],
        }
    }

    fn truth() -> DoppelDerivedTruth {
        DoppelDerivedTruth {
            claims: vec![DoppelClaimRecord {
                claim_id: "claim:s1".into(),
                subject_id: "s1".into(),
                claim_type: DoppelClaimType::RouteDelegate,
                statement: "route `GET /health` delegates through health_handler".into(),
                truth_class: DoppelTruthClass::Deterministic,
                posture: DoppelPostureKind::Verified,
                evidence_refs: vec!["e1".into()],
                anchor_refs: vec!["a1".into()],
                confidence_policy: "deterministic_gate".into(),
            }],
            postures: vec![DoppelPostureRecord {
                posture_id: "posture:claim:s1".into(),
                subject_id: Some("s1".into()),
                claim_id: Some("claim:s1".into()),
                posture: DoppelPostureKind::Verified,
                basis: "derived from 1 evidence record(s)".into(),
                updated_at: ts(),
            }],
            drift: vec![],
            posture_summary: DoppelPostureSummary {
                aggregate_posture: DoppelPostureKind::Verified,
                verified_count: 1,
                stale_count: 0,
                blocked_count: 0,
                partial_count: 0,
                unknown_count: 0,
            },
        }
    }

    #[test]
    fn manifest_reports_record_counts_and_carries_posture_summary() {
        let bundle = assemble_manifest_bundle(&inputs(), &normalized(), &truth(), ts());

        assert_eq!(bundle.manifest.manifest_id, "manifest:run-1");
        assert_eq!(bundle.manifest.record_counts.subjects, 1);
        assert_eq!(bundle.manifest.record_counts.anchors, 1);
        assert_eq!(bundle.manifest.record_counts.claims, 1);
        assert_eq!(bundle.manifest.record_counts.postures, 1);
        // The manifest reports derivation's posture, it does not recompute it.
        assert_eq!(
            bundle.manifest.posture_summary.aggregate_posture,
            DoppelPostureKind::Verified
        );
        assert_eq!(bundle.run_id, "run-1");
        assert_eq!(bundle.compliance_run_id.as_deref(), Some("compliance-1"));
    }

    #[test]
    fn rendered_markdown_is_traceable_and_digest_is_stable() {
        let bundle = assemble_manifest_bundle(&inputs(), &normalized(), &truth(), ts());

        // Every rendered fact traces to a record (Canvas 09 Rule 3).
        assert!(bundle.rendered_markdown.contains("health_handler"));
        assert!(bundle.rendered_markdown.contains("verified"));
        assert!(!bundle.manifest.render_digest.is_empty());

        // Re-emitting the same bundle yields the same digest.
        let again = assemble_manifest_bundle(&inputs(), &normalized(), &truth(), ts());
        assert_eq!(bundle.manifest.render_digest, again.manifest.render_digest);
    }

    #[test]
    fn changing_a_claim_changes_the_render_digest() {
        let base = assemble_manifest_bundle(&inputs(), &normalized(), &truth(), ts());

        let mut changed = truth();
        changed.claims[0].statement = "route `GET /health` delegates through other_handler".into();
        let other = assemble_manifest_bundle(&inputs(), &normalized(), &changed, ts());

        assert_ne!(base.manifest.render_digest, other.manifest.render_digest);
    }

    #[test]
    fn inputs_can_be_built_from_provenance() {
        let provenance = DoppelExtractionProvenance {
            run_id: "run-9".into(),
            repo_id: "repo-9".into(),
            revision_ref: "rev-9".into(),
            profile_id: DoppelProfileId::DocSystemV1,
            slice_id: "slice-9".into(),
            producer: "cortex".into(),
            extracted_at: ts(),
        };

        let built = DoppelManifestInputs::from_provenance(&provenance, "doppelcore", None, "0.1.0");

        assert_eq!(built.run_id, "run-9");
        assert_eq!(built.repo_id, "repo-9");
        assert_eq!(built.profile_id, DoppelProfileId::DocSystemV1);
        assert_eq!(built.system_id, "doppelcore");
        assert!(built.compliance_run_id.is_none());
    }
}
