//! Claim, posture, and drift derivation — the bounded claim engine (Phase 3,
//! Canvas 04 Stages 3–4).
//!
//! This is the stage where DoppelCore turns normalized subjects/anchors/evidence
//! (the output of [`crate::intake`]) into the first real machine truth: claims,
//! their posture, anchor drift, and an aggregate posture summary.
//!
//! It is deliberately bounded and honest, per the doctrine (Canvas 03 §3,
//! Canvas 09):
//!
//! - **Every claim cites evidence.** A subject with no evidence yields no claim
//!   — only a surfaced `unknown` posture. (Canvas 09 Rule 1.)
//! - **Heuristic never silently becomes verified.** Posture and truth class are
//!   derived from the determinism of the supporting evidence; deterministic
//!   facts may gate (`verified`), heuristic/asserted facts only inform review
//!   (`inferred`). (Canvas 03 §3.)
//! - **Unknowns are output, not hidden.** Subjects we cannot derive a claim for
//!   are surfaced as `unassessable`, never dropped. (Canvas 09 anti-patterns.)
//! - **No clock is read here.** The caller supplies `evaluated_at`, keeping
//!   derivation deterministic and testable.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{
    DoppelClaimType, DoppelDeterminismClass, DoppelDriftKind, DoppelPostureKind, DoppelSubjectKind,
    DoppelTruthClass,
};
use crate::intake::DoppelNormalizedRecords;
use crate::records::{
    DoppelAnchorRecord, DoppelClaimRecord, DoppelDriftRecord, DoppelEvidenceRecord,
    DoppelPostureRecord, DoppelPostureSummary,
};

/// The machine truth derived from one slice of normalized records.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelDerivedTruth {
    pub claims: Vec<DoppelClaimRecord>,
    pub postures: Vec<DoppelPostureRecord>,
    pub drift: Vec<DoppelDriftRecord>,
    pub posture_summary: DoppelPostureSummary,
}

/// The bounded claim type derived for a subject kind, if any.
///
/// Returns `None` for kinds with no bounded derivation in this slice — those
/// subjects are surfaced as `unassessable` rather than given a fabricated
/// claim (Canvas 09: do not attempt whole-repo behavioral omniscience).
pub fn claim_type_for(kind: DoppelSubjectKind) -> Option<DoppelClaimType> {
    match kind {
        DoppelSubjectKind::Route => Some(DoppelClaimType::RouteDelegate),
        DoppelSubjectKind::Workflow => Some(DoppelClaimType::WorkflowGuard),
        DoppelSubjectKind::PersistencePath => Some(DoppelClaimType::PersistenceWrite),
        DoppelSubjectKind::EventPath => Some(DoppelClaimType::EventEmission),
        DoppelSubjectKind::DocumentationArtifact => Some(DoppelClaimType::DocStructure),
        DoppelSubjectKind::Repository | DoppelSubjectKind::Module | DoppelSubjectKind::Service => {
            None
        }
    }
}

/// Posture, truth class, and confidence policy implied by a set of evidence.
///
/// Deterministic facts may gate (`verified`); heuristic and operator-asserted
/// facts only inform review (`inferred`); a mix is `partial`. Empty evidence is
/// `unknown` — never invented as proof (Canvas 03 §3, Canvas 09 Rule 4).
pub fn posture_for_evidence(
    evidence: &[&DoppelEvidenceRecord],
) -> (DoppelPostureKind, DoppelTruthClass, &'static str) {
    if evidence.is_empty() {
        return (
            DoppelPostureKind::Unknown,
            DoppelTruthClass::Inferred,
            "no_evidence",
        );
    }

    let mut deterministic = false;
    let mut soft = false; // heuristic or operator-asserted
    for record in evidence {
        match record.determinism_class {
            DoppelDeterminismClass::Deterministic => deterministic = true,
            DoppelDeterminismClass::Heuristic | DoppelDeterminismClass::OperatorAsserted => {
                soft = true
            }
            DoppelDeterminismClass::Mixed => {
                deterministic = true;
                soft = true;
            }
        }
    }

    match (deterministic, soft) {
        (true, false) => (
            DoppelPostureKind::Verified,
            DoppelTruthClass::Deterministic,
            "deterministic_gate",
        ),
        (false, true) => (
            DoppelPostureKind::Inferred,
            DoppelTruthClass::Inferred,
            "review_only",
        ),
        // Mixed determinism must not collapse to hard truth.
        _ => (
            DoppelPostureKind::Partial,
            DoppelTruthClass::Mixed,
            "partial_review",
        ),
    }
}

/// Worst-case severity used to fold many postures into one aggregate.
fn posture_severity(posture: DoppelPostureKind) -> u8 {
    match posture {
        DoppelPostureKind::Blocked => 8,
        DoppelPostureKind::Conflicted => 7,
        DoppelPostureKind::Unassessable => 6,
        DoppelPostureKind::Stale => 5,
        DoppelPostureKind::Partial => 4,
        DoppelPostureKind::Unknown => 3,
        DoppelPostureKind::Inferred => 2,
        DoppelPostureKind::Deterministic => 1,
        DoppelPostureKind::Verified => 0,
    }
}

/// Fold posture records into an aggregate summary (Canvas 04 Stage 4 —
/// "aggregate mirror posture"). The aggregate is the worst posture present;
/// an empty set is `unknown`, never an optimistic default.
pub fn summarize_posture(postures: &[DoppelPostureRecord]) -> DoppelPostureSummary {
    let mut summary = DoppelPostureSummary {
        aggregate_posture: DoppelPostureKind::Unknown,
        verified_count: 0,
        stale_count: 0,
        blocked_count: 0,
        partial_count: 0,
        unknown_count: 0,
    };

    let mut worst: Option<DoppelPostureKind> = None;
    for record in postures {
        match record.posture {
            DoppelPostureKind::Verified => summary.verified_count += 1,
            DoppelPostureKind::Stale => summary.stale_count += 1,
            DoppelPostureKind::Blocked => summary.blocked_count += 1,
            DoppelPostureKind::Partial => summary.partial_count += 1,
            DoppelPostureKind::Unknown => summary.unknown_count += 1,
            _ => {}
        }
        worst = Some(match worst {
            Some(current) if posture_severity(current) >= posture_severity(record.posture) => {
                current
            }
            _ => record.posture,
        });
    }

    if let Some(worst) = worst {
        summary.aggregate_posture = worst;
    }
    summary
}

/// Derive anchor drift by comparing a baseline anchor set against the current
/// candidate set (Canvas 04 Stage 4).
///
/// Emits `anchor_added` for anchors not seen in the baseline and
/// `anchor_changed` when an anchor's hash differs. Removed anchors have no
/// drift kind in the current contract, so they are not fabricated here.
pub fn evaluate_anchor_drift(
    baseline: &[DoppelAnchorRecord],
    candidate: &[DoppelAnchorRecord],
    evaluated_at: DateTime<Utc>,
) -> Vec<DoppelDriftRecord> {
    let baseline_hashes: HashMap<&str, &str> = baseline
        .iter()
        .map(|anchor| (anchor.anchor_id.as_str(), anchor.anchor_hash.as_str()))
        .collect();

    let mut drift = Vec::new();
    for anchor in candidate {
        let (kind, posture, summary) = match baseline_hashes.get(anchor.anchor_id.as_str()) {
            None => (
                DoppelDriftKind::AnchorAdded,
                DoppelPostureKind::Partial,
                format!("anchor `{}` newly observed", anchor.anchor_id),
            ),
            Some(&prior_hash) if prior_hash != anchor.anchor_hash => (
                DoppelDriftKind::AnchorChanged,
                DoppelPostureKind::Stale,
                format!(
                    "anchor `{}` hash changed ({prior_hash} -> {})",
                    anchor.anchor_id, anchor.anchor_hash
                ),
            ),
            // Unchanged anchor: no drift.
            Some(_) => continue,
        };

        drift.push(DoppelDriftRecord {
            drift_id: format!("drift:{}:{}", kind.as_str(), anchor.anchor_id),
            subject_id: anchor.subject_id.clone(),
            anchor_id: Some(anchor.anchor_id.clone()),
            drift_kind: kind,
            posture,
            summary,
            detected_at: evaluated_at,
            evidence_refs: Vec::new(),
            detail: None,
        });
    }
    drift
}

/// Human-readable phrase describing a set of anchors for a claim statement.
fn describe_anchors(anchors: &[&DoppelAnchorRecord]) -> String {
    let names: Vec<&str> = anchors
        .iter()
        .map(|anchor| {
            anchor
                .symbol_name
                .as_deref()
                .unwrap_or(anchor.path.as_str())
        })
        .collect();
    if names.is_empty() {
        "no anchors".to_string()
    } else {
        names.join(", ")
    }
}

/// Build the deterministic claim statement for a subject of the given kind.
fn claim_statement(
    claim_type: DoppelClaimType,
    display_name: &str,
    anchors: &[&DoppelAnchorRecord],
) -> String {
    let detail = describe_anchors(anchors);
    match claim_type {
        DoppelClaimType::RouteDelegate => {
            format!("route `{display_name}` delegates through {detail}")
        }
        DoppelClaimType::WorkflowGuard => {
            format!("workflow `{display_name}` is guarded at {detail}")
        }
        DoppelClaimType::PersistenceWrite => {
            format!("persistence path `{display_name}` writes via {detail}")
        }
        DoppelClaimType::EventEmission => {
            format!("event path `{display_name}` emits via {detail}")
        }
        DoppelClaimType::DocStructure => {
            format!("documentation artifact `{display_name}` is structured at {detail}")
        }
        other => format!(
            "{} for `{display_name}` anchored at {detail}",
            other.as_str()
        ),
    }
}

/// Derive claims, posture, and drift from normalized records, comparing anchors
/// against an optional baseline (a prior run's anchors) for drift.
///
/// `evaluated_at` is supplied by the caller; this function reads no clock.
pub fn derive_truth(
    normalized: &DoppelNormalizedRecords,
    baseline_anchors: &[DoppelAnchorRecord],
    evaluated_at: DateTime<Utc>,
) -> DoppelDerivedTruth {
    // Index anchors and evidence by subject for quick per-subject lookup.
    let mut anchors_by_subject: HashMap<&str, Vec<&DoppelAnchorRecord>> = HashMap::new();
    for anchor in &normalized.anchors {
        anchors_by_subject
            .entry(anchor.subject_id.as_str())
            .or_default()
            .push(anchor);
    }
    let mut evidence_by_subject: HashMap<&str, Vec<&DoppelEvidenceRecord>> = HashMap::new();
    for record in &normalized.evidence {
        if let Some(subject_id) = record.subject_id.as_deref() {
            evidence_by_subject
                .entry(subject_id)
                .or_default()
                .push(record);
        }
    }

    let mut claims = Vec::new();
    let mut postures = Vec::new();

    for subject in &normalized.subjects {
        let subject_id = subject.subject_id.as_str();
        let anchors = anchors_by_subject
            .get(subject_id)
            .cloned()
            .unwrap_or_default();
        let evidence = evidence_by_subject
            .get(subject_id)
            .cloned()
            .unwrap_or_default();

        let Some(claim_type) = claim_type_for(subject.subject_kind) else {
            // No bounded derivation for this kind: surface it, do not fabricate.
            postures.push(DoppelPostureRecord {
                posture_id: format!("posture:subject:{subject_id}"),
                subject_id: Some(subject.subject_id.clone()),
                claim_id: None,
                posture: DoppelPostureKind::Unassessable,
                basis: format!(
                    "no bounded claim derivation for subject kind `{}`",
                    subject.subject_kind.as_str()
                ),
                updated_at: evaluated_at,
            });
            continue;
        };

        if evidence.is_empty() {
            // A claim without evidence is forbidden (Canvas 09 Rule 1); surface
            // the unknown instead.
            postures.push(DoppelPostureRecord {
                posture_id: format!("posture:subject:{subject_id}"),
                subject_id: Some(subject.subject_id.clone()),
                claim_id: None,
                posture: DoppelPostureKind::Unknown,
                basis: "no evidence backing subject".to_string(),
                updated_at: evaluated_at,
            });
            continue;
        }

        let (posture, truth_class, confidence_policy) = posture_for_evidence(&evidence);
        let claim_id = format!("claim:{subject_id}");

        claims.push(DoppelClaimRecord {
            claim_id: claim_id.clone(),
            subject_id: subject.subject_id.clone(),
            claim_type,
            statement: claim_statement(claim_type, &subject.display_name, &anchors),
            truth_class,
            posture,
            evidence_refs: evidence.iter().map(|e| e.evidence_id.clone()).collect(),
            anchor_refs: anchors.iter().map(|a| a.anchor_id.clone()).collect(),
            confidence_policy: confidence_policy.to_string(),
        });

        postures.push(DoppelPostureRecord {
            posture_id: format!("posture:{claim_id}"),
            subject_id: Some(subject.subject_id.clone()),
            claim_id: Some(claim_id),
            posture,
            basis: format!("derived from {} evidence record(s)", evidence.len()),
            updated_at: evaluated_at,
        });
    }

    let drift = evaluate_anchor_drift(baseline_anchors, &normalized.anchors, evaluated_at);
    let posture_summary = summarize_posture(&postures);

    DoppelDerivedTruth {
        claims,
        postures,
        drift,
        posture_summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::DoppelAnchorKind;
    use crate::records::DoppelSubjectRecord;

    fn ts() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn evidence(id: &str, subject: &str, class: DoppelDeterminismClass) -> DoppelEvidenceRecord {
        DoppelEvidenceRecord {
            evidence_id: id.into(),
            subject_id: Some(subject.into()),
            anchor_id: None,
            evidence_type: "route_registration".into(),
            probe_method: "ast_scan".into(),
            source_ref: "src/routes/health.rs".into(),
            captured_at: ts(),
            digest: "deadbeef".into(),
            determinism_class: class,
            detail: None,
        }
    }

    fn anchor(id: &str, subject: &str, hash: &str) -> DoppelAnchorRecord {
        DoppelAnchorRecord {
            anchor_id: id.into(),
            subject_id: subject.into(),
            anchor_kind: DoppelAnchorKind::Function,
            path: "src/routes/health.rs".into(),
            line_start: Some(10),
            line_end: Some(24),
            symbol_name: Some("health_handler".into()),
            anchor_hash: hash.into(),
        }
    }

    fn subject(id: &str, kind: DoppelSubjectKind) -> DoppelSubjectRecord {
        DoppelSubjectRecord {
            subject_id: id.into(),
            subject_kind: kind,
            repo_id: "repo-1".into(),
            path: Some("src/routes/health.rs".into()),
            display_name: "GET /health".into(),
            revision_ref: "abc123".into(),
            source_profile: "route_service_slice_v1".into(),
        }
    }

    #[test]
    fn deterministic_evidence_yields_verified_gating_claim() {
        let (posture, truth, policy) =
            posture_for_evidence(&[&evidence("e1", "s1", DoppelDeterminismClass::Deterministic)]);
        assert_eq!(posture, DoppelPostureKind::Verified);
        assert_eq!(truth, DoppelTruthClass::Deterministic);
        assert_eq!(policy, "deterministic_gate");
    }

    #[test]
    fn heuristic_evidence_never_becomes_verified() {
        let (posture, truth, policy) =
            posture_for_evidence(&[&evidence("e1", "s1", DoppelDeterminismClass::Heuristic)]);
        assert_eq!(posture, DoppelPostureKind::Inferred);
        assert_eq!(truth, DoppelTruthClass::Inferred);
        assert_eq!(policy, "review_only");
    }

    #[test]
    fn mixed_determinism_is_partial_not_hard_truth() {
        let (posture, truth, _) = posture_for_evidence(&[
            &evidence("e1", "s1", DoppelDeterminismClass::Deterministic),
            &evidence("e2", "s1", DoppelDeterminismClass::Heuristic),
        ]);
        assert_eq!(posture, DoppelPostureKind::Partial);
        assert_eq!(truth, DoppelTruthClass::Mixed);
    }

    #[test]
    fn no_evidence_is_unknown() {
        let (posture, _, policy) = posture_for_evidence(&[]);
        assert_eq!(posture, DoppelPostureKind::Unknown);
        assert_eq!(policy, "no_evidence");
    }

    #[test]
    fn derive_truth_emits_evidence_backed_claim() {
        let normalized = DoppelNormalizedRecords {
            subjects: vec![subject("s1", DoppelSubjectKind::Route)],
            anchors: vec![anchor("a1", "s1", "hash-1")],
            evidence: vec![evidence("e1", "s1", DoppelDeterminismClass::Deterministic)],
        };

        let truth = derive_truth(&normalized, &[], ts());

        assert_eq!(truth.claims.len(), 1);
        let claim = &truth.claims[0];
        assert_eq!(claim.claim_type, DoppelClaimType::RouteDelegate);
        assert_eq!(claim.posture, DoppelPostureKind::Verified);
        assert_eq!(claim.evidence_refs, vec!["e1".to_string()]);
        assert_eq!(claim.anchor_refs, vec!["a1".to_string()]);
        assert!(claim.statement.contains("health_handler"));

        // A new anchor against an empty baseline is reported as added drift.
        assert_eq!(truth.drift.len(), 1);
        assert_eq!(truth.drift[0].drift_kind, DoppelDriftKind::AnchorAdded);

        assert_eq!(
            truth.posture_summary.aggregate_posture,
            DoppelPostureKind::Verified
        );
        assert_eq!(truth.posture_summary.verified_count, 1);
    }

    #[test]
    fn subject_without_evidence_yields_unknown_not_a_claim() {
        let normalized = DoppelNormalizedRecords {
            subjects: vec![subject("s1", DoppelSubjectKind::Route)],
            anchors: vec![],
            evidence: vec![],
        };

        let truth = derive_truth(&normalized, &[], ts());

        assert!(truth.claims.is_empty());
        assert_eq!(truth.postures.len(), 1);
        assert_eq!(truth.postures[0].posture, DoppelPostureKind::Unknown);
        assert!(truth.postures[0].claim_id.is_none());
    }

    #[test]
    fn unmapped_subject_kind_is_unassessable() {
        let normalized = DoppelNormalizedRecords {
            subjects: vec![subject("s1", DoppelSubjectKind::Service)],
            anchors: vec![],
            evidence: vec![evidence("e1", "s1", DoppelDeterminismClass::Deterministic)],
        };

        let truth = derive_truth(&normalized, &[], ts());

        assert!(truth.claims.is_empty());
        assert_eq!(truth.postures[0].posture, DoppelPostureKind::Unassessable);
    }

    #[test]
    fn changed_anchor_hash_is_stale_drift() {
        let baseline = vec![anchor("a1", "s1", "hash-1")];
        let candidate = vec![anchor("a1", "s1", "hash-2")];

        let drift = evaluate_anchor_drift(&baseline, &candidate, ts());
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].drift_kind, DoppelDriftKind::AnchorChanged);
        assert_eq!(drift[0].posture, DoppelPostureKind::Stale);
    }

    #[test]
    fn unchanged_anchor_produces_no_drift() {
        let baseline = vec![anchor("a1", "s1", "hash-1")];
        let candidate = vec![anchor("a1", "s1", "hash-1")];
        assert!(evaluate_anchor_drift(&baseline, &candidate, ts()).is_empty());
    }

    #[test]
    fn aggregate_posture_is_worst_case() {
        let postures = vec![
            DoppelPostureRecord {
                posture_id: "p1".into(),
                subject_id: None,
                claim_id: None,
                posture: DoppelPostureKind::Verified,
                basis: "x".into(),
                updated_at: ts(),
            },
            DoppelPostureRecord {
                posture_id: "p2".into(),
                subject_id: None,
                claim_id: None,
                posture: DoppelPostureKind::Stale,
                basis: "x".into(),
                updated_at: ts(),
            },
        ];
        let summary = summarize_posture(&postures);
        assert_eq!(summary.aggregate_posture, DoppelPostureKind::Stale);
        assert_eq!(summary.verified_count, 1);
        assert_eq!(summary.stale_count, 1);
    }
}
