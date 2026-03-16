#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LaneId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdapterId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplayArtifactRef {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegistryRef {
    pub family: String,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLevel {
    C0IngestValid,
    C1ReplayValid,
    C2DiffValid,
    C3ExplainValid,
    C4DistillValid,
    C5PackValid,
}

impl CapabilityLevel {
    pub const ALL: [Self; 6] = [
        Self::C0IngestValid,
        Self::C1ReplayValid,
        Self::C2DiffValid,
        Self::C3ExplainValid,
        Self::C4DistillValid,
        Self::C5PackValid,
    ];

    pub const fn registry_id(self) -> &'static str {
        match self {
            Self::C0IngestValid => "cap.C0.ingest_valid",
            Self::C1ReplayValid => "cap.C1.replay_valid",
            Self::C2DiffValid => "cap.C2.diff_valid",
            Self::C3ExplainValid => "cap.C3.explain_valid",
            Self::C4DistillValid => "cap.C4.distill_valid",
            Self::C5PackValid => "cap.C5.pack_valid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityClass {
    Semantic,
    Instrumentation,
    Informational,
    Coverage,
}
