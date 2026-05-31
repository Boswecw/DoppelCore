//! Differential scans and drift history (Phase 6).
//!
//! This module diffs two emitted [`DoppelManifestBundle`]s — a baseline and a
//! candidate — into a [`DoppelManifestComparison`], and projects a bundle into
//! a [`DoppelHistoryEntry`] for historical trending (Canvas 04 §5 — keep prior
//! manifest for diff comparison, retain drift across runs).
//!
//! The diff is structural and deterministic: it compares subjects and anchors
//! by their canonical ids (which [`crate::intake`] derives stably across runs)
//! and anchors by `anchor_hash`. Output ordering is sorted so the same pair of
//! bundles always produces the same comparison, regardless of vector order.

use std::collections::{BTreeSet, HashMap};

use crate::comparison::{
    DoppelAnchorDelta, DoppelDeltaKind, DoppelDriftDelta, DoppelHistoryEntry,
    DoppelManifestComparison,
};
use crate::contracts::DoppelDriftKind;
use crate::records::{DoppelAnchorRecord, DoppelManifestBundle, DoppelSubjectRecord};

/// Human-readable reference for an anchor, e.g. `src/routes/health.rs:10`.
fn anchor_ref(anchor: &DoppelAnchorRecord) -> String {
    match anchor.line_start {
        Some(line) => format!("{}:{}", anchor.path, line),
        None => anchor.path.clone(),
    }
}

fn index_subjects(bundle: &DoppelManifestBundle) -> HashMap<&str, &DoppelSubjectRecord> {
    bundle
        .subjects
        .iter()
        .map(|subject| (subject.subject_id.as_str(), subject))
        .collect()
}

/// Resolve a subject by id, preferring the candidate bundle's view.
fn lookup_subject<'a>(
    subject_id: &str,
    candidate: &HashMap<&str, &'a DoppelSubjectRecord>,
    baseline: &HashMap<&str, &'a DoppelSubjectRecord>,
) -> Option<&'a DoppelSubjectRecord> {
    candidate
        .get(subject_id)
        .or_else(|| baseline.get(subject_id))
        .copied()
}

fn diff_anchors(
    baseline: &DoppelManifestBundle,
    candidate: &DoppelManifestBundle,
    baseline_subjects: &HashMap<&str, &DoppelSubjectRecord>,
    candidate_subjects: &HashMap<&str, &DoppelSubjectRecord>,
) -> Vec<DoppelAnchorDelta> {
    let baseline_map: HashMap<&str, &DoppelAnchorRecord> = baseline
        .anchors
        .iter()
        .map(|anchor| (anchor.anchor_id.as_str(), anchor))
        .collect();
    let candidate_map: HashMap<&str, &DoppelAnchorRecord> = candidate
        .anchors
        .iter()
        .map(|anchor| (anchor.anchor_id.as_str(), anchor))
        .collect();

    let mut ids: BTreeSet<&str> = baseline_map.keys().copied().collect();
    ids.extend(candidate_map.keys().copied());

    let mut deltas = Vec::new();
    for id in ids {
        let baseline_anchor = baseline_map.get(id).copied();
        let candidate_anchor = candidate_map.get(id).copied();

        let (change_kind, subject_id, summary) = match (baseline_anchor, candidate_anchor) {
            (Some(b), Some(c)) => {
                if b.anchor_hash == c.anchor_hash {
                    continue; // unchanged
                }
                (
                    DoppelDeltaKind::Changed,
                    c.subject_id.as_str(),
                    format!("anchor `{id}` changed"),
                )
            }
            (None, Some(c)) => (
                DoppelDeltaKind::Added,
                c.subject_id.as_str(),
                format!("anchor `{id}` added"),
            ),
            (Some(b), None) => (
                DoppelDeltaKind::Removed,
                b.subject_id.as_str(),
                format!("anchor `{id}` removed"),
            ),
            (None, None) => continue,
        };

        let Some(subject) = lookup_subject(subject_id, candidate_subjects, baseline_subjects)
        else {
            // Anchor references a subject in neither bundle: malformed input we
            // refuse to classify rather than fabricate a kind.
            continue;
        };

        deltas.push(DoppelAnchorDelta {
            subject_id: subject.subject_id.clone(),
            subject_label: subject.display_name.clone(),
            subject_kind: subject.subject_kind,
            change_kind,
            baseline_anchor_id: baseline_anchor.map(|a| a.anchor_id.clone()),
            candidate_anchor_id: candidate_anchor.map(|a| a.anchor_id.clone()),
            baseline_anchor_ref: baseline_anchor.map(anchor_ref),
            candidate_anchor_ref: candidate_anchor.map(anchor_ref),
            summary,
        });
    }
    deltas
}

fn diff_drift(
    baseline: &DoppelManifestBundle,
    candidate: &DoppelManifestBundle,
    baseline_subjects: &HashMap<&str, &DoppelSubjectRecord>,
    candidate_subjects: &HashMap<&str, &DoppelSubjectRecord>,
) -> Vec<DoppelDriftDelta> {
    let baseline_map: HashMap<&str, &crate::records::DoppelDriftRecord> = baseline
        .drift
        .iter()
        .map(|drift| (drift.drift_id.as_str(), drift))
        .collect();
    let candidate_map: HashMap<&str, &crate::records::DoppelDriftRecord> = candidate
        .drift
        .iter()
        .map(|drift| (drift.drift_id.as_str(), drift))
        .collect();

    let mut ids: BTreeSet<&str> = baseline_map.keys().copied().collect();
    ids.extend(candidate_map.keys().copied());

    let mut deltas = Vec::new();
    for id in ids {
        let baseline_drift = baseline_map.get(id).copied();
        let candidate_drift = candidate_map.get(id).copied();

        let (change_kind, source, summary) = match (baseline_drift, candidate_drift) {
            (Some(b), Some(c)) => {
                if b.summary == c.summary {
                    continue; // unchanged drift state
                }
                (DoppelDeltaKind::Changed, c, format!("drift `{id}` updated"))
            }
            (None, Some(c)) => (DoppelDeltaKind::Added, c, format!("drift `{id}` appeared")),
            (Some(b), None) => (
                DoppelDeltaKind::Removed,
                b,
                format!("drift `{id}` resolved"),
            ),
            (None, None) => continue,
        };

        let label = lookup_subject(&source.subject_id, candidate_subjects, baseline_subjects)
            .map(|subject| subject.display_name.clone())
            .unwrap_or_else(|| source.subject_id.clone());

        deltas.push(DoppelDriftDelta {
            subject_id: source.subject_id.clone(),
            subject_label: label,
            drift_kind: source.drift_kind,
            change_kind,
            summary,
        });
    }
    deltas
}

/// Compare a baseline manifest bundle against a candidate, producing a
/// deterministic structural diff.
pub fn compare_manifests(
    baseline: &DoppelManifestBundle,
    candidate: &DoppelManifestBundle,
) -> DoppelManifestComparison {
    let baseline_subjects = index_subjects(baseline);
    let candidate_subjects = index_subjects(candidate);

    let baseline_ids: BTreeSet<&str> = baseline_subjects.keys().copied().collect();
    let candidate_ids: BTreeSet<&str> = candidate_subjects.keys().copied().collect();

    let added_subjects: Vec<String> = candidate_ids
        .difference(&baseline_ids)
        .map(|id| id.to_string())
        .collect();
    let removed_subjects: Vec<String> = baseline_ids
        .difference(&candidate_ids)
        .map(|id| id.to_string())
        .collect();

    let anchor_deltas = diff_anchors(baseline, candidate, &baseline_subjects, &candidate_subjects);
    let drift_deltas = diff_drift(baseline, candidate, &baseline_subjects, &candidate_subjects);

    let summary = format!(
        "{} subject(s) added, {} removed; {} anchor delta(s); {} drift delta(s)",
        added_subjects.len(),
        removed_subjects.len(),
        anchor_deltas.len(),
        drift_deltas.len(),
    );

    DoppelManifestComparison {
        baseline_manifest_id: baseline.manifest.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest.manifest_id.clone(),
        baseline_generated_at: baseline.manifest.generated_at,
        candidate_generated_at: candidate.manifest.generated_at,
        baseline_posture: baseline.manifest.posture_summary.aggregate_posture,
        candidate_posture: candidate.manifest.posture_summary.aggregate_posture,
        added_subjects,
        removed_subjects,
        anchor_deltas,
        drift_deltas,
        summary,
    }
}

/// Project a manifest bundle into a history entry for trending across runs.
pub fn build_history_entry(bundle: &DoppelManifestBundle) -> DoppelHistoryEntry {
    let anchor_drift_count = bundle
        .drift
        .iter()
        .filter(|drift| {
            matches!(
                drift.drift_kind,
                DoppelDriftKind::AnchorAdded | DoppelDriftKind::AnchorChanged
            )
        })
        .count();

    DoppelHistoryEntry {
        manifest_id: bundle.manifest.manifest_id.clone(),
        run_id: bundle.run_id.clone(),
        compliance_run_id: bundle.compliance_run_id.clone(),
        generated_at: bundle.manifest.generated_at,
        revision_ref: bundle.manifest.revision_ref.clone(),
        aggregate_posture: bundle.manifest.posture_summary.aggregate_posture,
        drift_count: bundle.drift.len(),
        anchor_drift_count,
    }
}
