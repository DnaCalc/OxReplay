#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use oxreplay_abstractions::{AdapterId, LaneId, RegistryRef, ReplayArtifactRef};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleManifest {
    pub bundle_id: String,
    pub scenario_id: String,
    pub bundle_schema: String,
    pub source_schema: String,
    pub lane_id: LaneId,
    pub adapter_id: AdapterId,
    pub capture_mode: String,
    pub registry_refs: Vec<RegistryRef>,
    #[serde(default)]
    pub projection_status: ProjectionStatus,
    #[serde(default)]
    pub capture_loss: CaptureLossStatus,
    #[serde(default)]
    pub sidecars: Vec<BundleArtifactRef>,
    #[serde(default)]
    pub views: Vec<BundleArtifactRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionStatus {
    #[default]
    Lossless,
    Lossy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CaptureLossStatus {
    #[default]
    None,
    DowngradedInstrumentation,
    ProjectionLoss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleArtifactRef {
    pub artifact_family: String,
    pub path: String,
}

#[derive(Debug, Error)]
pub enum BundleValidationError {
    #[error("bundle schema must not be empty")]
    MissingBundleSchema,
    #[error("bundle schema `{0}` is not supported")]
    UnsupportedBundleSchema(String),
    #[error("scenario id must not be empty")]
    MissingScenarioId,
    #[error("bundle id must not be empty")]
    MissingBundleId,
    #[error("source schema must not be empty")]
    MissingSourceSchema,
    #[error("capture mode must not be empty")]
    MissingCaptureMode,
    #[error("registry family `{0}` is not supported")]
    UnsupportedRegistryFamily(String),
    #[error("artifact path `{0}` must be repo-relative and stay inside the bundle directory")]
    InvalidArtifactPath(String),
    #[error("artifact `{0}` is missing")]
    MissingArtifact(String),
    #[error("failed to read manifest from `{path}`: {source}")]
    ReadManifest {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse manifest from `{path}`: {source}")]
    ParseManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

impl ReplayBundleManifest {
    pub fn validate(&self) -> Result<(), BundleValidationError> {
        if self.bundle_id.trim().is_empty() {
            return Err(BundleValidationError::MissingBundleId);
        }

        if self.bundle_schema.trim().is_empty() {
            return Err(BundleValidationError::MissingBundleSchema);
        }

        if self.bundle_schema != BUNDLE_SCHEMA_V1 {
            return Err(BundleValidationError::UnsupportedBundleSchema(
                self.bundle_schema.clone(),
            ));
        }

        if self.scenario_id.trim().is_empty() {
            return Err(BundleValidationError::MissingScenarioId);
        }

        if self.source_schema.trim().is_empty() {
            return Err(BundleValidationError::MissingSourceSchema);
        }

        if self.capture_mode.trim().is_empty() {
            return Err(BundleValidationError::MissingCaptureMode);
        }

        for registry_ref in &self.registry_refs {
            if !CANONICAL_REGISTRY_FAMILIES.contains(&registry_ref.family.as_str()) {
                return Err(BundleValidationError::UnsupportedRegistryFamily(
                    registry_ref.family.clone(),
                ));
            }
        }

        Ok(())
    }
}

pub const BUNDLE_SCHEMA_V1: &str = "replay.bundle.v1";

const CANONICAL_REGISTRY_FAMILIES: [&str; 6] = [
    "predicate_kind",
    "mismatch_kind",
    "severity",
    "reduction_outcome",
    "witness_lifecycle_state",
    "capability_level",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationIssueCode {
    InvalidManifest,
    InvalidArtifactPath,
    MissingArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub code: ValidationIssueCode,
    pub message: String,
    pub artifact_ref: Option<ReplayArtifactRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleIndex {
    pub registry_families: Vec<String>,
    pub sidecar_artifacts: Vec<ReplayArtifactRef>,
    pub view_artifacts: Vec<ReplayArtifactRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleValidationReport {
    pub status: ValidationStatus,
    pub manifest_path: String,
    pub bundle_id: Option<String>,
    pub scenario_id: Option<String>,
    pub pack_impact: &'static str,
    pub capability_impact: &'static str,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub index: Option<BundleIndex>,
}

pub fn load_manifest_from_path(
    path: impl AsRef<Path>,
) -> Result<ReplayBundleManifest, BundleValidationError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let manifest_text =
        fs::read_to_string(path).map_err(|source| BundleValidationError::ReadManifest {
            path: path_text.clone(),
            source,
        })?;

    serde_json::from_str(&manifest_text).map_err(|source| BundleValidationError::ParseManifest {
        path: path_text,
        source,
    })
}

pub fn validate_bundle_at_path(
    path: impl AsRef<Path>,
) -> Result<BundleValidationReport, BundleValidationError> {
    let path = path.as_ref();
    let manifest = load_manifest_from_path(path)?;
    let manifest_path = path.display().to_string();

    let mut errors = Vec::new();
    let warnings = Vec::new();

    if let Err(error) = manifest.validate() {
        errors.push(ValidationIssue {
            code: ValidationIssueCode::InvalidManifest,
            message: error.to_string(),
            artifact_ref: Some(ReplayArtifactRef {
                path: manifest_path.clone(),
            }),
        });
    }

    let index = if errors.is_empty() {
        Some(build_index(&manifest, path, &mut errors))
    } else {
        None
    };

    Ok(BundleValidationReport {
        status: if errors.is_empty() {
            ValidationStatus::Valid
        } else {
            ValidationStatus::Invalid
        },
        manifest_path,
        bundle_id: Some(manifest.bundle_id),
        scenario_id: Some(manifest.scenario_id),
        capability_impact: "shared validate/ingest surface only; no adapter capability claim",
        pack_impact: "candidate validate/ingest surface for PACK.replay.appliance",
        errors,
        warnings,
        index,
    })
}

pub fn render_text_report(report: &BundleValidationReport) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "status: {:?}", report.status);
    let _ = writeln!(text, "manifest_path: {}", report.manifest_path);

    if let Some(bundle_id) = &report.bundle_id {
        let _ = writeln!(text, "bundle_id: {bundle_id}");
    }

    if let Some(scenario_id) = &report.scenario_id {
        let _ = writeln!(text, "scenario_id: {scenario_id}");
    }

    let _ = writeln!(text, "capability_impact: {}", report.capability_impact);
    let _ = writeln!(text, "pack_impact: {}", report.pack_impact);

    if let Some(index) = &report.index {
        let _ = writeln!(
            text,
            "index: registries={}, sidecars={}, views={}",
            index.registry_families.len(),
            index.sidecar_artifacts.len(),
            index.view_artifacts.len()
        );
    }

    if report.errors.is_empty() {
        let _ = writeln!(text, "errors: none");
    } else {
        let _ = writeln!(text, "errors:");
        for error in &report.errors {
            let _ = writeln!(text, "  - {:?}: {}", error.code, error.message);
        }
    }

    text
}

fn build_index(
    manifest: &ReplayBundleManifest,
    manifest_path: &Path,
    errors: &mut Vec<ValidationIssue>,
) -> BundleIndex {
    let bundle_root = manifest_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    BundleIndex {
        registry_families: manifest
            .registry_refs
            .iter()
            .map(|registry_ref| registry_ref.family.clone())
            .collect(),
        sidecar_artifacts: resolve_artifacts(&manifest.sidecars, &bundle_root, errors),
        view_artifacts: resolve_artifacts(&manifest.views, &bundle_root, errors),
    }
}

fn resolve_artifacts(
    artifacts: &[BundleArtifactRef],
    bundle_root: &Path,
    errors: &mut Vec<ValidationIssue>,
) -> Vec<ReplayArtifactRef> {
    let mut resolved = Vec::new();

    for artifact in artifacts {
        if !is_relative_bundle_path(&artifact.path) {
            errors.push(ValidationIssue {
                code: ValidationIssueCode::InvalidArtifactPath,
                message: format!(
                    "artifact family `{}` uses invalid path `{}`",
                    artifact.artifact_family, artifact.path
                ),
                artifact_ref: Some(ReplayArtifactRef {
                    path: artifact.path.clone(),
                }),
            });
            continue;
        }

        let artifact_path = bundle_root.join(&artifact.path);
        if !artifact_path.exists() {
            errors.push(ValidationIssue {
                code: ValidationIssueCode::MissingArtifact,
                message: format!(
                    "artifact family `{}` missing at `{}`",
                    artifact.artifact_family, artifact.path
                ),
                artifact_ref: Some(ReplayArtifactRef {
                    path: artifact.path.clone(),
                }),
            });
            continue;
        }

        resolved.push(ReplayArtifactRef {
            path: artifact.path.clone(),
        });
    }

    resolved
}

fn is_relative_bundle_path(path: &str) -> bool {
    let path = Path::new(path);
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn rejects_empty_schema() {
        let manifest = ReplayBundleManifest {
            bundle_id: "bundle-1".to_string(),
            scenario_id: "rb_manifest_minimal_valid_001".to_string(),
            bundle_schema: String::new(),
            source_schema: "tracecalc-s1".to_string(),
            lane_id: LaneId("oxcalc".to_string()),
            adapter_id: AdapterId("oxcalc.replay.v1".to_string()),
            capture_mode: "deterministic".to_string(),
            registry_refs: vec![],
            projection_status: ProjectionStatus::Lossless,
            capture_loss: CaptureLossStatus::None,
            sidecars: vec![],
            views: vec![],
        };

        let error = manifest
            .validate()
            .expect_err("missing schema should be rejected");
        assert!(matches!(error, BundleValidationError::MissingBundleSchema));
    }

    #[test]
    fn validates_retained_minimal_fixture() {
        let report =
            validate_bundle_at_path(fixture_path("rb_manifest_minimal_valid_001/manifest.json"))
                .expect("fixture should load");

        assert_eq!(report.status, ValidationStatus::Valid);
        assert!(report.errors.is_empty());
        assert_eq!(
            report.bundle_id.as_deref(),
            Some("bundle-minimal-valid-001")
        );
    }

    #[test]
    fn rejects_retained_invalid_fixture() {
        let report =
            validate_bundle_at_path(fixture_path("rb_manifest_schema_invalid_001/manifest.json"))
                .expect("fixture should load");

        assert_eq!(report.status, ValidationStatus::Invalid);
        assert_eq!(report.errors.len(), 1);
    }

    #[test]
    fn resolves_sidecars_and_views_for_indexing() {
        let report = validate_bundle_at_path(fixture_path(
            "rb_sidecar_resolution_roundtrip_001/manifest.json",
        ))
        .expect("fixture should load");

        let index = report.index.expect("valid fixture should build index");
        assert_eq!(index.sidecar_artifacts.len(), 2);
        assert_eq!(index.view_artifacts.len(), 1);
        assert_eq!(report.status, ValidationStatus::Valid);
    }

    #[test]
    fn indexes_projection_fixture() {
        let report =
            validate_bundle_at_path(fixture_path("rb_bundle_index_projection_001/manifest.json"))
                .expect("fixture should load");

        let index = report.index.expect("valid fixture should build index");
        assert_eq!(index.registry_families.len(), 2);
        assert_eq!(index.view_artifacts.len(), 2);
    }

    fn fixture_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/test-corpus/bundles")
            .join(relative)
    }
}
