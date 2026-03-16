# AGENTS.md — OxReplay Agent Instructions

## 1. Context Loading Order

On session start, read in this order:

1. `README.md`
2. `CHARTER.md`
3. `OPERATIONS.md`
4. `CURRENT_BLOCKERS.md`
5. `docs/IN_PROGRESS_FEATURE_WORKLIST.md`
6. `docs/worksets/README.md`
7. `docs/spec/README.md`
8. `docs/spec/OXREPLAY_SCOPE_AND_BOUNDARY.md`
9. `docs/spec/OXREPLAY_RUNTIME_STRATA_AND_PACKAGE_MAP.md`
10. `docs/spec/OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md`
11. `docs/spec/OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md`
12. `docs/spec/DNA_RECALC_HOST.md`
13. `docs/spec/OXREPLAY_IMPLEMENTATION_BASELINE.md`
14. `docs/spec/OXREPLAY_REPLAY_CLASS_AND_SCENARIO_REGISTER.md`
15. `docs/spec/OXREPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md`
16. `docs/spec/OXREPLAY_INITIAL_ADAPTER_INTAKE_PLAN.md`
17. `docs/spec/DNA_RECALC_CLI_CONTRACT.md`
18. `docs/spec/OXREPLAY_WITNESS_LIFECYCLE_TRANSITIONS.md`
19. Inbound observation ledgers from sibling repos, if present (see `OPERATIONS.md` Section 12.2):
   - `../OxCalc/docs/upstream/NOTES_FOR_OXREPLAY.md`
   - `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md`
   - `../OxFunc/docs/upstream/NOTES_FOR_OXREPLAY.md`
   - `../OxVba/docs/upstream/NOTES_FOR_OXREPLAY.md`
20. Foundation doctrine docs (`../Foundation/CHARTER.md`, `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`, `../Foundation/OPERATIONS.md`, `../Foundation/REPLAY_APPLIANCE.md`)

## 2. Source-of-Truth Precedence

When guidance conflicts, precedence is:

1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`
5. this repo `CHARTER.md`
6. this repo `OPERATIONS.md`
7. this repo `docs/spec/*`

For `OxReplay`-local work, treat `CHARTER.md` in this directory as the working charter.
For cross-program doctrine and architectural constraints, treat Foundation docs as authoritative.
For mutable shared-runtime and host detail, use `docs/spec/*` in this repo.
Lane repos remain authoritative for lane-native semantic meaning even when `OxReplay` hosts the adapter runtime.

## 3. Anti-Premature-Completion Doctrine

This section is binding. Violations are doctrine failures, not style preferences.

### Rule 1: Restricted Completion Language
The words "implemented", "closed", "done", and "complete" are forbidden when describing:
- partial subsets of declared scope,
- scaffolding, stubs, or compile-only code,
- host shells without exercised adapter evidence,
- capability claims without conformance proof,
- spec text without replay-governed evidence.

Use "in-progress", "partial", or "scaffolded" instead.

### Rule 2: Self-Audit Required Before Completion Claims
Before ANY completion claim, the agent must:
1. Run the Pre-Closure Verification Checklist from `OPERATIONS.md` Section 7.
2. Run the Completion Claim Self-Audit from `OPERATIONS.md` Section 9.
3. Include the checklist and self-audit results in the completion report.

### Rule 3: Three-Axis Reporting Mandatory
Every status report must include:
- `scope_completeness` (`scope_complete` | `scope_partial`)
- `target_completeness` (`target_complete` | `target_partial`)
- `integration_completeness` (`integrated` | `partial`)
- explicit `open_lanes` list when any axis is partial

### Rule 4: Scaffolding Is Not Implementation
Stubs, empty adapters, compile-only package structure, and placeholder hosts are scaffolding.
Scaffolding is never reported as implementation. Report it as `scaffolded`.

### Rule 5: Spec Text Without Evidence Is Not Done
Spec or contract text without at least one deterministic replay-governed artifact proving intended behavior is not done. Report it as `spec_drafted`.

### Rule 6: Capability Claims Require Conformance Evidence
An adapter or host capability level is not complete unless the claimed level is exercised by conformance evidence and the resulting limitations are stated explicitly.

### Rule 7: Cross-Repo Handoff Is Not Completion
Filing a handoff packet to a lane repo or back to Foundation opens a dependency. It does not close the originating work.

### Rule 8: Default to In-Progress
When uncertain whether work meets completion criteria, report `in_progress`.

### Rule 9: Shared Mechanics Must Not Absorb Semantic Ownership
If a proposed convenience change causes `OxReplay` to infer, reinterpret, or hard-code lane-local meaning, the change is not complete and must be redirected into an adapter or upstream doctrine path.

## 4. Continuation Behavior

Mode: **checkpoint-at-gates** (conservative).

1. Agent must pause and report status at each workset gate boundary.
2. AutoRun is disabled by default.
3. AutoRun may only be enabled when explicitly requested by the user for a specific declared scope.
4. Between gates, the agent may proceed autonomously within the declared workset scope.

Rationale: `OxReplay` begins with cross-repo surface area and a high risk of ownership drift, so conservative gate-pausing is the default.

## 5. Blocker Handling

When a blocker is encountered:

1. Create or update `CURRENT_BLOCKERS.md` with a structured `BLK-REPLAY-NNN` entry.
2. Continue with other non-blocked work within scope.
3. If all paths are blocked, emit a structured summary:
   - blocked items with `BLK-*` identifiers,
   - current state of each,
   - exact unblock steps required,
   - recommendation (`wait` / `escalate` / `workaround`).

## 6. Public Attribution Doctrine

For any issue, pull request, email response, release note, discussion post, or any other external/public-facing message authored by an agent, the first line must be an italicized attribution line.

Required format:

*Posted by [Agent] agent on behalf of @govert*

Scope exclusions (do not add attribution by default):
- internal run artifacts,
- repository documentation drafts and working notes,
- local analysis files not being published externally.

## 7. Change Discipline

1. Keep changes minimal, explicit, and testable.
2. Keep `OxReplay` from absorbing lane-semantic ownership.
3. Treat adapter contracts as semantic boundaries, not convenience escape hatches.
4. Place new local policy in the most specific spec file practical.
5. Require explicit capability and pack impact notes for changes touching replay-governed surfaces.
6. When proposing changes that affect shared replay doctrine, route the normative delta back through Foundation rather than silently reassigning authority locally.
7. When a design decision affects a sibling repo's adapter contract, write an outbound observation note under `docs/upstream/` and file a handoff packet if normative text is implicated.
