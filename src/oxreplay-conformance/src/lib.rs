#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use oxreplay_abstractions::{AdapterId, CapabilityLevel, LaneId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayAdapterCapabilityManifest {
    pub adapter_id: AdapterId,
    #[serde(default)]
    pub adapter_version: Option<String>,
    pub lane_id: LaneId,
    pub supported_source_schema_ids: Vec<String>,
    pub supported_replay_bundle_schema_versions: Vec<String>,
    #[serde(default)]
    pub supported_capture_modes: Vec<String>,
    pub claimed_capability_levels: Vec<String>,
    #[serde(default)]
    pub target_capability_levels: Vec<String>,
    #[serde(default)]
    pub scaffolded_capability_levels: Vec<String>,
    #[serde(default)]
    pub known_limits: Vec<KnownLimit>,
    #[serde(default)]
    pub conformance_artifact_refs: Vec<String>,
    #[serde(default)]
    pub registry_version_refs: Vec<RegistryVersionRef>,
    #[serde(default)]
    pub lifecycle_states: Vec<String>,
    #[serde(default)]
    pub rollout_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityValidationResult {
    pub adapter_id: AdapterId,
    pub accepted: bool,
    pub diagnostics: Vec<String>,
    pub normalized_claimed_capabilities: Vec<CapabilityLevel>,
    pub lifecycle_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnownLimit {
    Summary(String),
    Structured { limit_id: String, summary: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryVersionRef {
    pub registry_family: String,
    pub registry_version: String,
    #[serde(default)]
    pub source_ref: Option<String>,
}

#[derive(Debug, Error)]
pub enum ManifestLoadError {
    #[error("failed to read manifest from `{path}`: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse manifest from `{path}`: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

pub fn validate_manifest(manifest: &ReplayAdapterCapabilityManifest) -> CapabilityValidationResult {
    let mut diagnostics = Vec::new();
    let mut normalized_claimed_capabilities = Vec::new();

    if manifest.adapter_id.0.trim().is_empty() {
        diagnostics.push("adapter id must not be empty".to_string());
    }

    if manifest.lane_id.0.trim().is_empty() {
        diagnostics.push("lane id must not be empty".to_string());
    }

    if manifest.supported_source_schema_ids.is_empty() {
        diagnostics.push("supported source schemas must not be empty".to_string());
    }

    if manifest.supported_replay_bundle_schema_versions.is_empty() {
        diagnostics.push("supported replay bundle schema versions must not be empty".to_string());
    }

    for schema in &manifest.supported_replay_bundle_schema_versions {
        if !matches!(schema.as_str(), "dna-replay-bundle/v1" | "replay.bundle.v1") {
            diagnostics.push(format!(
                "unsupported replay bundle schema version `{schema}`"
            ));
        }
    }

    for capability in &manifest.claimed_capability_levels {
        match parse_capability_level(capability) {
            Some(parsed) => normalized_claimed_capabilities.push(parsed),
            None => diagnostics.push(format!("unsupported capability level `{capability}`")),
        }
    }

    for capability in manifest
        .target_capability_levels
        .iter()
        .chain(manifest.scaffolded_capability_levels.iter())
    {
        if parse_capability_level(capability).is_none() {
            diagnostics.push(format!(
                "unsupported target/scaffolded capability `{capability}`"
            ));
        }
    }

    for registry_ref in &manifest.registry_version_refs {
        if normalize_registry_family(&registry_ref.registry_family).is_none() {
            diagnostics.push(format!(
                "unsupported registry family `{}`",
                registry_ref.registry_family
            ));
        }
    }

    let lifecycle_required = normalized_claimed_capabilities.iter().any(|capability| {
        matches!(
            capability,
            CapabilityLevel::C4DistillValid | CapabilityLevel::C5PackValid
        )
    });

    if lifecycle_required && manifest.lifecycle_states.is_empty() {
        diagnostics.push("distill/pack claims require declared lifecycle states".to_string());
    }

    CapabilityValidationResult {
        adapter_id: manifest.adapter_id.clone(),
        accepted: diagnostics.is_empty(),
        diagnostics,
        normalized_claimed_capabilities,
        lifecycle_required,
    }
}

pub fn load_manifest_from_path(
    path: impl AsRef<Path>,
) -> Result<ReplayAdapterCapabilityManifest, ManifestLoadError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let manifest_text = fs::read_to_string(path).map_err(|source| ManifestLoadError::Read {
        path: path_text.clone(),
        source,
    })?;

    serde_json::from_str(&manifest_text).map_err(|source| ManifestLoadError::Parse {
        path: path_text,
        source,
    })
}

fn parse_capability_level(capability: &str) -> Option<CapabilityLevel> {
    match capability {
        "cap.C0.ingest_valid" => Some(CapabilityLevel::C0IngestValid),
        "cap.C1.replay_valid" => Some(CapabilityLevel::C1ReplayValid),
        "cap.C2.diff_valid" => Some(CapabilityLevel::C2DiffValid),
        "cap.C3.explain_valid" => Some(CapabilityLevel::C3ExplainValid),
        "cap.C4.distill_valid" => Some(CapabilityLevel::C4DistillValid),
        "cap.C5.pack_valid" => Some(CapabilityLevel::C5PackValid),
        _ => None,
    }
}

fn normalize_registry_family(family: &str) -> Option<&'static str> {
    match family {
        "predicate_kind" => Some("predicate_kind"),
        "mismatch_kind" => Some("mismatch_kind"),
        "severity" | "severity_class" => Some("severity"),
        "reduction_outcome" | "reduction_status" => Some("reduction_outcome"),
        "witness_lifecycle_state" => Some("witness_lifecycle_state"),
        "capability_level" => Some("capability_level"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn distill_requires_lifecycle_states() {
        let manifest = ReplayAdapterCapabilityManifest {
            adapter_id: AdapterId("adapter.test".to_string()),
            adapter_version: Some("0.1.0".to_string()),
            lane_id: LaneId("oxcalc".to_string()),
            supported_source_schema_ids: vec!["tracecalc-s1".to_string()],
            supported_replay_bundle_schema_versions: vec!["dna-replay-bundle/v1".to_string()],
            supported_capture_modes: vec!["deterministic".to_string()],
            claimed_capability_levels: vec!["cap.C4.distill_valid".to_string()],
            target_capability_levels: vec![],
            scaffolded_capability_levels: vec![],
            known_limits: vec![],
            conformance_artifact_refs: vec![],
            registry_version_refs: vec![],
            lifecycle_states: vec![],
            rollout_notes: vec![],
        };

        let result = validate_manifest(&manifest);

        assert!(!result.accepted);
        assert_eq!(
            result.diagnostics,
            vec!["distill/pack claims require declared lifecycle states"]
        );
    }

    #[test]
    fn validates_local_fixture_manifest() {
        let manifest =
            load_manifest_from_path(fixture_path("cap_manifest_minimal_valid_001/manifest.json"))
                .expect("fixture should load");

        let result = validate_manifest(&manifest);

        assert!(result.accepted);
        assert_eq!(
            result.normalized_claimed_capabilities,
            vec![CapabilityLevel::C0IngestValid]
        );
    }

    #[test]
    fn rejects_invalid_fixture_manifest() {
        let manifest = load_manifest_from_path(fixture_path(
            "cap_manifest_schema_invalid_001/manifest.json",
        ))
        .expect("fixture should load");

        let result = validate_manifest(&manifest);

        assert!(!result.accepted);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("unsupported capability level"))
        );
    }

    #[test]
    fn flags_real_oxcalc_manifest_lifecycle_gap() {
        let manifest = load_manifest_from_path(PathBuf::from(
            env!("CARGO_MANIFEST_DIR"),
        )
        .join("../../../OxCalc/docs/spec/core-engine/CORE_ENGINE_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json"))
        .expect("oxcalc manifest should load");

        let result = validate_manifest(&manifest);

        assert!(!result.accepted);
        assert_eq!(
            result.normalized_claimed_capabilities,
            vec![
                CapabilityLevel::C0IngestValid,
                CapabilityLevel::C1ReplayValid,
                CapabilityLevel::C2DiffValid,
                CapabilityLevel::C3ExplainValid,
                CapabilityLevel::C4DistillValid,
            ]
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("lifecycle states"))
        );
    }

    fn fixture_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/test-corpus/bundles")
            .join(relative)
    }
}
