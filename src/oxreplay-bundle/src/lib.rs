#![forbid(unsafe_code)]

use oxreplay_abstractions::{AdapterId, LaneId, RegistryRef};
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
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BundleValidationError {
    #[error("bundle schema must not be empty")]
    MissingBundleSchema,
    #[error("scenario id must not be empty")]
    MissingScenarioId,
}

impl ReplayBundleManifest {
    pub fn validate(&self) -> Result<(), BundleValidationError> {
        if self.bundle_schema.trim().is_empty() {
            return Err(BundleValidationError::MissingBundleSchema);
        }

        if self.scenario_id.trim().is_empty() {
            return Err(BundleValidationError::MissingScenarioId);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        };

        assert_eq!(
            manifest.validate(),
            Err(BundleValidationError::MissingBundleSchema)
        );
    }
}
