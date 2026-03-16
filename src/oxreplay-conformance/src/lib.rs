#![forbid(unsafe_code)]

use oxreplay_abstractions::{AdapterId, CapabilityLevel, LaneId, RegistryRef};
use oxreplay_governance::WitnessLifecycleState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayAdapterCapabilityManifest {
    pub adapter_id: AdapterId,
    pub lane_id: LaneId,
    pub supported_source_schemas: Vec<String>,
    pub supported_bundle_schemas: Vec<String>,
    pub supported_capture_modes: Vec<String>,
    pub claimed_capabilities: Vec<CapabilityLevel>,
    pub known_limits: Vec<String>,
    pub registry_refs: Vec<RegistryRef>,
    pub lifecycle_states: Vec<WitnessLifecycleState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityValidationResult {
    pub adapter_id: AdapterId,
    pub accepted: bool,
    pub diagnostics: Vec<String>,
}

pub fn validate_manifest(manifest: &ReplayAdapterCapabilityManifest) -> CapabilityValidationResult {
    let requires_lifecycle = manifest.claimed_capabilities.iter().any(|capability| {
        matches!(
            capability,
            CapabilityLevel::C4DistillValid | CapabilityLevel::C5PackValid
        )
    });

    let mut diagnostics = Vec::new();

    if requires_lifecycle && manifest.lifecycle_states.is_empty() {
        diagnostics.push("distill/pack claims require declared lifecycle states".to_string());
    }

    CapabilityValidationResult {
        adapter_id: manifest.adapter_id.clone(),
        accepted: diagnostics.is_empty(),
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distill_requires_lifecycle_states() {
        let manifest = ReplayAdapterCapabilityManifest {
            adapter_id: AdapterId("adapter.test".to_string()),
            lane_id: LaneId("oxcalc".to_string()),
            supported_source_schemas: vec!["tracecalc-s1".to_string()],
            supported_bundle_schemas: vec!["replay-bundle-v1".to_string()],
            supported_capture_modes: vec!["deterministic".to_string()],
            claimed_capabilities: vec![CapabilityLevel::C4DistillValid],
            known_limits: vec![],
            registry_refs: vec![],
            lifecycle_states: vec![],
        };

        let result = validate_manifest(&manifest);

        assert!(!result.accepted);
        assert_eq!(
            result.diagnostics,
            vec!["distill/pack claims require declared lifecycle states"]
        );
    }
}
