# OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md

## 1. Position
This document defines the local `OxReplay` implementation boundary for bundle, witness, registry, and lifecycle handling.

Foundation remains the owner of replay doctrine and normative cross-program constraints.
Lane repos remain the owners of source semantic meaning.
`OxReplay` owns the shared mechanics needed to ingest, validate, index, operate on, and emit replay-governed artifacts.

## 2. Shared artifact families
The shared runtime is expected to handle these artifact families:
1. canonical replay bundles,
2. adapter capability manifests,
3. registry refs and optional registry snapshots,
4. witness bundles and reduction manifests,
5. witness lifecycle records,
6. explanation and conformance outputs where they are retained as evidence.

## 3. Ownership split
1. Foundation defines the doctrine, compatibility expectations, lifecycle policies, and registry governance.
2. Lane repos define the source artifact meaning, source schema truth, closure rules, and safe rewrite permissions.
3. `OxReplay` implements:
   - schema-aware ingest,
   - sidecar resolution,
   - lineage-preserving normalization,
   - compatibility checking,
   - replay and explanation execution over canonical forms,
   - witness emission and lifecycle handling.

## 4. Required local runtime responsibilities
`OxReplay` must be able to:
1. validate bundle and manifest schema/version compatibility,
2. preserve source lane, adapter id, schema lineage, capture mode, projection status, capture-loss or downgraded-instrumentation status when present, and registry pinning,
3. resolve sidecars and external artifact refs without silently mutating tracked inputs,
4. surface lifecycle state, quarantine reason, supersession lineage, and pack-eligibility state explicitly,
5. use canonical registry ids when a replay-governed registry family exists and snapshot the required registry versions in retained artifacts,
6. expose machine-usable failure classes for invalid bundles, unsupported capabilities, incompatible registries, or unstable distillation outcomes.

## 5. Non-goals
`OxReplay` must not:
1. invent missing source semantics,
2. infer lane-local safe rewrites,
3. silently coerce incompatible source schemas into "best effort" runtime behavior,
4. treat witness lifecycle state as optional decoration.

## 6. Suggested retained roots
The initial retained artifact layout should prefer:
1. `docs/test-corpus/bundles/`
   - canonical retained sample bundles and bundle-shape fixtures.
2. `docs/test-corpus/witnesses/`
   - retained witness bundles and reduction examples.
3. `states/registry/`
   - registry snapshots, compatibility baselines, and validator reference state.
4. `states/lifecycle/`
   - retained lifecycle examples, quarantine examples, or transition fixtures when checked-in state is required.
5. `docs/test-runs/<run-id>/`
   - human-readable summaries of validation, conformance, and retained replay runs.

## 7. Canonical registry families
The initial replay-governed registry families are:
1. predicate kind,
2. mismatch kind,
3. severity,
4. reduction outcome,
5. witness lifecycle state,
6. capability level.

Registry rule:
1. tool outputs use registry ids when a family exists,
2. bundles snapshot the registry versions they depend on,
3. lane-local labels may remain as explanatory metadata but do not replace canonical ids.

## 8. Bootstrap validation slices
The initial implementation sequence for these artifact families should be:
1. ingest and validate canonical bundles,
2. ingest and validate adapter capability manifests,
3. pin and validate registry refs,
4. support replay and diff/explain inputs,
5. add witness emission and lifecycle handling after the above are stable.

## 9. Evidence rule
Any runtime feature that claims support for one of the artifact families above must identify:
1. the tracked artifact root,
2. the relevant schema or registry versions,
3. the validator or replay command used to exercise it,
4. the resulting capability or lifecycle outcome.
