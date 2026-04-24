use serde::{Deserialize, Serialize};

use crate::errors::{DoppelCoreError, Result};

macro_rules! string_enum {
    ($name:ident, $err_variant:ident, { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value),+
                }
            }

            pub fn from_str(value: &str) -> Result<Self> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    other => Err(DoppelCoreError::$err_variant(other.to_string())),
                }
            }
        }
    };
}

string_enum!(DoppelProfileId, UnknownProfileId, {
    SystemReality => "system_reality",
    CorrectionReady => "correction_ready"
});

string_enum!(DoppelClaimType, UnknownClaimType, {
    CanonicalFact => "canonical_fact",
    ComplianceFact => "compliance_fact",
    BehavioralFact => "behavioral_fact"
});

string_enum!(DoppelPostureKind, UnknownPostureKind, {
    Clear => "clear",
    Review => "review",
    Blocked => "blocked"
});

string_enum!(DoppelDriftKind, UnknownDriftKind, {
    AnchorDrift => "anchor_drift",
    EvidenceGap => "evidence_gap",
    ClaimConflict => "claim_conflict"
});

string_enum!(DoppelIntakeSourceKind, UnknownIntakeSourceKind, {
    SelfHealingIncident => "self_healing_incident"
});

string_enum!(DoppelIntakeStatusKind, UnknownIntakeStatusKind, {
    Accepted => "accepted",
    Blocked => "blocked",
    Duplicate => "duplicate"
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_id_round_trip() {
        assert_eq!(
            DoppelProfileId::from_str(DoppelProfileId::SystemReality.as_str()).unwrap(),
            DoppelProfileId::SystemReality
        );
    }

    #[test]
    fn intake_status_round_trip() {
        assert_eq!(
            DoppelIntakeStatusKind::from_str(DoppelIntakeStatusKind::Accepted.as_str()).unwrap(),
            DoppelIntakeStatusKind::Accepted
        );
    }
}
