//! Cortex extraction packet contracts (Phase 2).
//!
//! These types describe the bounded extraction packets produced upstream by
//! Cortex (Canvas 04, Stage 1) and relayed through Centipede intake (Canvas
//! 06). They are *signal*, not canonical truth: DoppelCore normalizes them into
//! canonical records via [`crate::intake`]. Every packet carries provenance so
//! the resulting records remain traceable back to the scan request that
//! produced them.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::contracts::{
    DoppelAnchorKind, DoppelDeterminismClass, DoppelProfileId, DoppelSubjectKind,
};

/// Kind of bounded extraction packet a Cortex profile can emit (Canvas 04,
/// Stage 1 — "Possible packet types").
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DoppelExtractionPacketKind {
    FileIndex,
    SafeText,
    Symbol,
    RouteRegistration,
    ImportGraph,
    DocumentationTree,
}

impl DoppelExtractionPacketKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileIndex => "file_index",
            Self::SafeText => "safe_text",
            Self::Symbol => "symbol",
            Self::RouteRegistration => "route_registration",
            Self::ImportGraph => "import_graph",
            Self::DocumentationTree => "documentation_tree",
        }
    }
}

/// Provenance shared by every extraction packet.
///
/// Carries the scan-request identity (Canvas 04, Stage 0) so that normalized
/// subjects, anchors, and evidence can be traced back to their source run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelExtractionProvenance {
    pub run_id: String,
    pub repo_id: String,
    pub revision_ref: String,
    pub profile_id: DoppelProfileId,
    pub slice_id: String,
    /// Upstream producer, e.g. `"cortex"`. Records the handoff origin.
    pub producer: String,
    pub extracted_at: DateTime<Utc>,
}

/// A subject candidate observed by the extractor (a route, service, doc
/// artifact, …). The `external_id` is the stable identifier assigned by the
/// extractor; DoppelCore derives its own canonical id during normalization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelExtractedSubject {
    pub external_id: String,
    pub subject_kind: DoppelSubjectKind,
    pub repo_path: Option<String>,
    pub display_name: String,
}

/// An anchor candidate tied to a subject's `external_id`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelExtractedAnchor {
    pub external_id: String,
    pub subject_external_id: String,
    pub anchor_kind: DoppelAnchorKind,
    pub path: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub symbol_name: Option<String>,
    /// Content hash captured by the extractor; carried into the canonical
    /// anchor and used downstream for drift detection.
    pub content_hash: String,
}

/// A bounded extraction packet: provenance plus the subjects and anchors the
/// extractor observed for one packet kind.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoppelExtractionPacket {
    pub packet_id: String,
    pub packet_kind: DoppelExtractionPacketKind,
    pub provenance: DoppelExtractionProvenance,
    pub subjects: Vec<DoppelExtractedSubject>,
    pub anchors: Vec<DoppelExtractedAnchor>,
    /// How the extractor gathered this packet (e.g. `"ast_scan"`). Recorded on
    /// every emitted evidence record.
    pub probe_method: String,
    /// Determinism of the extraction method (Canvas 04 — inference must not
    /// outrun proof). Carried onto emitted evidence.
    pub determinism_class: DoppelDeterminismClass,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_kind_wire_values_are_stable() {
        assert_eq!(DoppelExtractionPacketKind::FileIndex.as_str(), "file_index");
        assert_eq!(
            DoppelExtractionPacketKind::RouteRegistration.as_str(),
            "route_registration"
        );
        assert_eq!(
            DoppelExtractionPacketKind::DocumentationTree.as_str(),
            "documentation_tree"
        );
    }

    #[test]
    fn packet_kind_serde_matches_as_str() {
        let kind = DoppelExtractionPacketKind::ImportGraph;
        assert_eq!(
            serde_json::to_string(&kind).unwrap(),
            format!("\"{}\"", kind.as_str())
        );
    }
}
