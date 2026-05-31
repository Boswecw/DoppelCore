//! Rendered projection — human review packet and UI slice (Phase 5).
//!
//! This module projects an emitted [`DoppelManifestBundle`] into a structured,
//! serializable review view ([`DoppelReviewProjection`]) and a Markdown render.
//! The projection serves as both the backend review packet and the UI slice: a
//! frontend renders it directly and never computes posture, severity, or truth
//! class on its own (Canvas 09 anti-patterns).
//!
//! Everything here is strictly downstream of the records (Canvas 09 Rule 3 —
//! *every rendered output must be traceable to machine records*). Posture is
//! reported, not invented: the headline posture comes from the manifest's
//! summary and per-subject posture is folded by the canonical
//! [`crate::derivation::summarize_posture`], so the view cannot disagree with
//! the machine truth (guarding against render inversion, Canvas 09 Risk D).

use std::fmt::Write as _;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{DoppelPostureKind, DoppelSubjectKind};
use crate::derivation::summarize_posture;
use crate::records::{
    DoppelManifestBundle, DoppelPostureRecord, DoppelPostureSummary, DoppelRecordCounts,
};

/// A single claim as presented for review.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelClaimProjection {
    pub claim_id: String,
    pub claim_type: String,
    pub statement: String,
    pub posture: DoppelPostureKind,
    pub truth_class: String,
    pub evidence_refs: Vec<String>,
    pub anchor_refs: Vec<String>,
}

/// A subject and its claims/drift, with a posture folded from its own posture
/// records (never recomputed in the view).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelSubjectProjection {
    pub subject_id: String,
    pub label: String,
    pub subject_kind: DoppelSubjectKind,
    pub posture: DoppelPostureKind,
    pub claims: Vec<DoppelClaimProjection>,
    pub drift_summaries: Vec<String>,
}

/// The full review projection for a manifest bundle — the backend review packet
/// and UI slice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelReviewProjection {
    pub manifest_id: String,
    pub repo_id: String,
    pub revision_ref: String,
    pub generated_at: DateTime<Utc>,
    pub headline_posture: DoppelPostureKind,
    pub posture_summary: DoppelPostureSummary,
    pub record_counts: DoppelRecordCounts,
    pub subjects: Vec<DoppelSubjectProjection>,
    pub drift_summaries: Vec<String>,
    /// The manifest's render digest, carried so a rendered surface can be tied
    /// back to the exact bundle it came from.
    pub source_render_digest: String,
}

/// Project a manifest bundle into a review view.
///
/// Reorganizes records by subject; computes no new posture beyond folding each
/// subject's existing posture records via the canonical aggregator. Output is
/// sorted for deterministic rendering.
pub fn project_review(bundle: &DoppelManifestBundle) -> DoppelReviewProjection {
    let mut subjects: Vec<DoppelSubjectProjection> = bundle
        .subjects
        .iter()
        .map(|subject| {
            let subject_id = subject.subject_id.as_str();

            let mut claims: Vec<DoppelClaimProjection> = bundle
                .claims
                .iter()
                .filter(|claim| claim.subject_id == subject_id)
                .map(|claim| DoppelClaimProjection {
                    claim_id: claim.claim_id.clone(),
                    claim_type: claim.claim_type.as_str().to_string(),
                    statement: claim.statement.clone(),
                    posture: claim.posture,
                    truth_class: claim.truth_class.as_str().to_string(),
                    evidence_refs: claim.evidence_refs.clone(),
                    anchor_refs: claim.anchor_refs.clone(),
                })
                .collect();
            claims.sort_by(|a, b| a.claim_id.cmp(&b.claim_id));

            // Fold this subject's posture records with the canonical aggregator
            // rather than inventing a posture in the view.
            let subject_postures: Vec<DoppelPostureRecord> = bundle
                .postures
                .iter()
                .filter(|posture| posture.subject_id.as_deref() == Some(subject_id))
                .cloned()
                .collect();
            let posture = summarize_posture(&subject_postures).aggregate_posture;

            let mut drift_summaries: Vec<String> = bundle
                .drift
                .iter()
                .filter(|drift| drift.subject_id == subject_id)
                .map(|drift| drift.summary.clone())
                .collect();
            drift_summaries.sort();

            DoppelSubjectProjection {
                subject_id: subject.subject_id.clone(),
                label: subject.display_name.clone(),
                subject_kind: subject.subject_kind,
                posture,
                claims,
                drift_summaries,
            }
        })
        .collect();
    subjects.sort_by(|a, b| a.subject_id.cmp(&b.subject_id));

    let mut drift_summaries: Vec<String> = bundle
        .drift
        .iter()
        .map(|drift| drift.summary.clone())
        .collect();
    drift_summaries.sort();

    DoppelReviewProjection {
        manifest_id: bundle.manifest.manifest_id.clone(),
        repo_id: bundle.manifest.repo_id.clone(),
        revision_ref: bundle.manifest.revision_ref.clone(),
        generated_at: bundle.manifest.generated_at,
        headline_posture: bundle.manifest.posture_summary.aggregate_posture,
        posture_summary: bundle.manifest.posture_summary.clone(),
        record_counts: bundle.manifest.record_counts.clone(),
        subjects,
        drift_summaries,
        source_render_digest: bundle.manifest.render_digest.clone(),
    }
}

/// Render a review projection as Markdown.
///
/// The headline posture and every subject's posture are shown prominently so a
/// reader cannot mistake a partial or stale mirror for a verified one (Canvas
/// 09 Risk D). The footer states that posture is reported from records, not
/// computed in the view (Rule 3).
pub fn render_review_markdown(projection: &DoppelReviewProjection) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# DoppelCore review — {}", projection.manifest_id);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- repo: `{}` @ `{}`",
        projection.repo_id, projection.revision_ref
    );
    let _ = writeln!(
        out,
        "- headline posture: **{}**",
        projection.headline_posture.as_str()
    );
    let summary = &projection.posture_summary;
    let _ = writeln!(
        out,
        "- posture counts — verified: {}, partial: {}, stale: {}, blocked: {}, unknown: {}",
        summary.verified_count,
        summary.partial_count,
        summary.stale_count,
        summary.blocked_count,
        summary.unknown_count,
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "## Subjects");
    if projection.subjects.is_empty() {
        let _ = writeln!(out, "- (none)");
    }
    for subject in &projection.subjects {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "### [{}] {} ({})",
            subject.posture.as_str(),
            subject.label,
            subject.subject_kind.as_str()
        );
        if subject.claims.is_empty() {
            let _ = writeln!(out, "- no claims (posture surfaced as above)");
        }
        for claim in &subject.claims {
            let _ = writeln!(
                out,
                "- [{} / {}] {}",
                claim.posture.as_str(),
                claim.truth_class,
                claim.statement
            );
            if !claim.evidence_refs.is_empty() {
                let _ = writeln!(out, "  - evidence: {}", claim.evidence_refs.join(", "));
            }
        }
        for drift in &subject.drift_summaries {
            let _ = writeln!(out, "- drift: {drift}");
        }
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "## Drift");
    if projection.drift_summaries.is_empty() {
        let _ = writeln!(out, "- (none)");
    } else {
        for drift in &projection.drift_summaries {
            let _ = writeln!(out, "- {drift}");
        }
    }
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "_Rendered from machine records (source digest `{}`). Posture is reported from records, not computed in this view._",
        projection.source_render_digest
    );

    out
}
