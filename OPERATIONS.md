# OPERATIONS.md — OxReplay Operations

## 1. Purpose
Define day-to-day execution rules for the shared Replay appliance runtime and the `DNA ReCalc` host.

## 2. Operating Principles
1. Shared replay mechanics are centralized; semantic meaning remains lane-owned.
2. All shared outputs must preserve source identity and schema lineage explicitly.
3. Canonical registry ids and witness lifecycle state are first-class runtime concerns, not optional metadata.
4. Distillation is offline and must remain predicate-driven and replay-closed.
5. `DNA ReCalc` is a host surface, not a replacement semantics authority.

## 3. Working Strata
Initial implementation strata are:
1. `Abstractions`
   - adapter interfaces, ids, manifest types, registry refs, lifecycle refs.
2. `Bundle`
   - manifest parsing, validation, sidecar resolution, indexing.
3. `Core`
   - normalized replay runtime types and orchestration context.
4. `Diff` and `Explain`
   - typed mismatch and causal-query surfaces.
5. `Distill`
   - predicate execution, reduction search, reduction-manifest emission.
6. `Governance` and `Conformance`
   - capability validation, registry snapshots, lifecycle transitions, conformance harnesses.
7. `DNA ReCalc`
   - CLI first, optional later UI host over the same runtime surfaces.

## 4. Required Packs (baseline)
1. `PACK.replay.appliance`
2. `PACK.trace.forensic_plane`
3. `PACK.diff.cross_engine.continuous`
4. `PACK.reject.calculus` when typed reject replay evidence is part of the claim.

## 5. Cross-Repo Handoff Rule
When shared runtime changes affect lane adapters or replay doctrine:
1. confirm the change is mechanical rather than semantic,
2. identify affected capability levels,
3. record required migration or fallback behavior,
4. route doctrine-affecting changes back to Foundation through managed-run promotion packets,
5. file handoff packets to affected lane repos when adapter-facing obligations change.

## 6. Promotion Gate
No shared Replay runtime promotion without:
1. updated spec text for affected strata,
2. declared adapter and capability impact,
3. replay-governed pack impact notes,
4. explicit check that no lane-semantic authority was absorbed into `OxReplay`,
5. retained artifact paths for conformance or replay evidence when runtime behavior changed.

## 7. Pre-Closure Verification Checklist

Before claiming any workset or feature item as complete, answer each item yes or no.
All items must be "yes" for a completion claim. Any "no" means the item is `in_progress`.

| # | Check | Yes/No |
|---|-------|--------|
| 1 | Spec text updated for all in-scope items? | |
| 2 | Capability, registry, and witness-lifecycle impacts stated? | |
| 3 | At least one deterministic replay-governed artifact exists per in-scope runtime behavior? | |
| 4 | Conformance evidence updated for any claimed adapter or host capability changes? | |
| 5 | Cross-repo impact assessed and handoff filed if needed? | |
| 6 | Pack impact stated for affected packs? | |
| 7 | All required tests pass? | |
| 8 | No lane-semantic ownership drift remains in declared scope? | |
| 9 | IN_PROGRESS_FEATURE_WORKLIST.md updated? | |
| 10 | CURRENT_BLOCKERS.md updated (new/resolved)? | |

## 8. Expanded Definition of Done

A workset or feature item is done for its declared scope only when all of the following hold:

1. **Spec text**: all in-scope shared-runtime and host text is updated and internally consistent.
2. **Bundle and registry invariants**: affected bundle, registry, lifecycle, and manifest responsibilities are stated and exercised.
3. **Replay evidence**: at least one deterministic replay-governed artifact per in-scope behavior proves intended behavior.
4. **Conformance evidence**: any changed capability claim is backed by conformance output or explicitly marked unsupported.
5. **Cross-repo impact**: affected lane adapters or Foundation doctrine paths are assessed; handoff packets filed if needed.
6. **Pack impact**: affected pack surfaces are updated with evidence links or explicitly marked unchanged.
7. **No ownership drift**: no known lane-semantic authority has leaked into shared runtime code or local spec text.
8. **No semantic gaps**: no known semantic gap remains between the local spec and the exercised shared-runtime behavior for declared scope.
9. **Three-axis report**: completion report includes `scope_completeness`, `target_completeness`, `integration_completeness`, and `open_lanes` per `AGENTS.md` Section 3.
10. **Checklist attached**: Pre-Closure Verification Checklist (Section 7) is filled in and all items are "yes".

## 9. Completion Claim Self-Audit

Before submitting a completion claim, the agent must perform this self-audit and include the results.

### Step 1: Scope Re-Read
Re-read the workset scope declaration. For each in-scope item, verify that exercised implementation (not scaffolding) matches. Any missing item = `in_progress`.

### Step 2: Gate Criteria Re-Read
Re-read the workset gate criteria. All pass criteria must be met. Any unmet criterion = gate open.

### Step 3: Silent Scope Reduction Check
Compare the original scope declaration with what was actually delivered. Any unreported narrowing of scope is a doctrine violation. If scope was intentionally narrowed, it must be explicitly documented with rationale.

### Step 4: "Looks Done But Is Not" Pattern Check
Check for these patterns:
- compile-only strata or package splits reported as real runtime support,
- capability levels claimed without conformance evidence,
- normalization logic inferring lane semantics instead of consuming adapter declarations,
- replay artifacts that cannot be reproduced from tracked inputs.

### Step 5: Include Result
Include the self-audit result in the completion report with explicit pass/fail for each step.

## 10. Report-Back Completeness Contract

Every completion report (status updates, workset closure notes, handoff summaries) must include:

1. `execution_state`: `planned` | `in_progress` | `blocked` | `complete`
2. `scope_completeness`: `scope_complete` | `scope_partial`
3. `target_completeness`: `target_complete` | `target_partial`
4. `integration_completeness`: `integrated` | `partial`
5. `open_lanes`: explicit list when any completeness axis is partial
6. `capability_impact`: `none` or a short explicit statement
7. `pack_impact`: `none` or a short explicit statement

Normative wording rules:
1. Use `complete for declared scope` only when the declared scope already represents full known shared-runtime semantics and only integration or external limits remain partial.
2. Do not use `complete for declared scope` for semantically bounded subsets that still carry known gaps; report those as `scope_partial`.
3. Do not claim `fully complete` unless all three completeness axes are complete and evidence links are present.

## 11. Carried-Forward Operating Lessons

These lessons are binding operating guidance derived from earlier execution failures across the wider program and from the replay-bootstrap design itself.

### Lesson 1: Scaffold Determinism Is a Gate
Scaffolding (stubs, empty adapters, compile-only code) must produce deterministic outputs or be explicitly marked non-functional. Non-deterministic scaffolding that silently passes tests is a gate failure.

### Lesson 2: Spec Drift Checks Run Alongside Implementation
Do not defer spec-vs-implementation consistency checks to a separate phase. Run them as part of each workset execution. Spec drift discovered late is expensive to reconcile.

### Lesson 3: Final Validation Must Not Mutate Tracked Evidence
Validation runs must not modify the artifacts they are validating. Evidence mutation during validation invalidates the evidence chain.

### Lesson 4: Provenance Loss Is A Semantic Failure
If normalization drops source lane, schema lineage, registry pinning, or lifecycle state in a way that prevents later explanation, the runtime is incorrect even if replay appears to work.

### Lesson 5: Capability Strings Without Evidence Are Noise
Capability levels are evidence-bearing claims. If they are not exercised, validated, and retained, they are not part of the runtime truth.

### Lesson 6: Distillation Must Stay Offline And Predicate-Bound
Any attempt to move witness reduction logic into the hot-path capture or replay mechanics is a design failure unless explicitly reauthorized by Foundation doctrine.

### Lesson 7: Shared Convenience Must Not Become Semantic Reinterpretation
If a helper makes lane-local meaning "easier" by encoding it in shared runtime code, the helper is in the wrong repo or the adapter boundary is underspecified.

## 12. Upstream Observation Ledger Protocol

### 12.1 Purpose
Repos that use or integrate with `OxReplay` discover interface and design constraints through their own implementation work. Those observations must flow back to `OxReplay` through a structured channel so they inform design before shared runtime contracts solidify.

This is distinct from handoff packets (Section 5), which propose specific normative text changes. Observation ledgers are standing documents that accumulate design feedback over time.

### 12.2 Inbound Observation Sources
`OxReplay` must check for inbound observation ledgers from sibling repos at the start of any adapter, runtime, or host interface workset. Known source locations are:

| Source repo | Ledger location | Relationship |
|-------------|----------------|--------------|
| OxCalc | `../OxCalc/docs/upstream/NOTES_FOR_OXREPLAY.md` | Coordinator-facing replay constraints |
| OxFml | `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md` | Formula and seam replay constraints |
| OxFunc | `../OxFunc/docs/upstream/NOTES_FOR_OXREPLAY.md` | Function-oracle replay constraints |
| OxVba | `../OxVba/docs/upstream/NOTES_FOR_OXREPLAY.md` | VBA replay and host-integration constraints |

### 12.3 Outbound Observations
When `OxReplay` implementation work reveals design constraints that affect a sibling repo or Foundation, write them under `docs/upstream/` using this structure:

1. **Purpose**: what the receiving repo needs to know and why.
2. **Core message**: the essential design constraint in 2-3 sentences.
3. **Current evidence**: specific examples with concrete scenarios.
4. **Interface implications**: what the receiving repo must preserve, avoid, or expose.
5. **Minimum invariants**: binary testable statements.
6. **Open questions**: explicit questions the receiving repo should answer.

### 12.4 Lifecycle
1. Observation ledgers are living documents and are updated as new evidence accumulates.
2. Entries are never silently removed; outdated observations are marked superseded with rationale.
3. When an observation is addressed by the receiving repo, the originating entry is updated with a resolution reference.
4. Observation ledgers are not completion artifacts and do not close worksets or satisfy gate criteria.

### 12.5 Agent Obligation
Agents starting work on adapter, runtime, or host interface design must:
1. check all listed inbound observation sources (Section 12.2),
2. note any unresolved observations that are relevant to current scope,
3. include a "reviewed inbound observations" line in the workset status report,
4. when a design decision addresses an inbound observation, reference the observation entry explicitly.

## 13. Emitted Artifact Protocol

### 13.1 Canonical Artifact Root Required
Any execution packet that expects emitted evidence must declare a canonical artifact root before implementation begins.

That declaration must state:
1. the canonical root path,
2. whether the artifacts are checked in or ephemeral,
3. whether emitted artifacts are bundle, conformance, explanation, distillation, or pack-export artifacts.

### 13.2 Path Normalization Rule
Tracked artifacts must use repo-relative paths only.

Absolute paths are allowed only in transient local diagnostics that are not tracked.

### 13.3 Validation Non-Mutation Rule
Validation runs must not mutate tracked evidence in place.

If a checked-in baseline run exists:
1. re-validation should run into a separate transient run id, or
2. the tracked baseline should be regenerated intentionally as a new evidence act, not accidentally during validation.

### 13.4 Artifact Root Reporting Rule
Completion and status reports for any emitted-evidence workset must name:
1. the canonical artifact root,
2. the checked-in baseline run if one exists,
3. the commands used to generate or validate it.

### 13.5 Suggested Baseline Roots
Initial retained roots should prefer:
1. `docs/test-corpus/` for canonical retained bundles and witnesses,
2. `docs/test-runs/` for human-readable retained run summaries,
3. `states/registry/` for checked-in registry or lifecycle state snapshots when needed.

## 14. Execution Packet Minimums

Any workset that acts as an execution packet must include the following sections explicitly.

### 14.1 Environment Preconditions
1. required tools on PATH,
2. optional tools and their role,
3. fallback evidence rules if optional tools are unavailable.

### 14.2 Evidence Layout
1. canonical emitted artifact root,
2. checked-in versus ephemeral policy,
3. stable naming policy for baseline runs.

### 14.3 Replay-Corpus Readiness
If replay classes are part of the gate model, the packet must state:
1. which replay classes require corpus scenarios before implementation begins,
2. which scenario ids satisfy them,
3. which replay classes remain reserve or later lanes.

### 14.4 Pack-Evidence Traceability
Execution packets that mention packs must identify:
1. pack name,
2. replay classes,
3. scenario ids or artifact paths once they exist.

### 14.5 Workset Versus Feature-Area Rule
Execution packets and feature-register items have different closure semantics.

Rule:
1. a workset may reach `complete` for its declared scope while the broader feature area remains `in_progress`,
2. later widening must use a successor workset or explicit extension lane rather than silently reopening a completed workset,
3. completion reports should state this distinction whenever a broader feature area remains active.

## 15. Local Doctrine Reference
`OxReplay`-local execution lessons live at `docs/LOCAL_EXECUTION_DOCTRINE.md`.

That file is additive to the carried-forward lessons in Section 11.
It should be updated when later execution waves reveal stronger repo-local operating practices.
