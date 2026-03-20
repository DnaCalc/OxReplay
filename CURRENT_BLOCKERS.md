# CURRENT_BLOCKERS.md — OxReplay

Status: active blockers present.

Last reviewed: 2026-03-18.

---

## Active Blockers

### BLK-REPLAY-002: OxCalc manifest C4 lifecycle gap

- **Status**: active
- **Impact**: `W003` sibling-manifest acceptance, `W004` honest capability intake, and any local acceptance of `OxCalc` `cap.C4.distill_valid`
- **Current state**: the retained `OxReplay` W003 conformance baseline rejects `../OxCalc/docs/spec/core-engine/CORE_ENGINE_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json` because it claims `cap.C4.distill_valid` without declaring lifecycle states
- **Exact unblock steps**: either add explicit lifecycle states to the `OxCalc` manifest and keep the `C4` claim, or downgrade the current claim to `C3` and keep `C4` as target or scaffolded until lifecycle evidence is exposed
- **Recommendation**: escalate
- **Opened**: 2026-03-18

---

## Resolved Blockers

### BLK-REPLAY-001: First implementation stack undeclared

- **Status**: resolved
- **Impact**: `W002` through `W006` activation packet quality
- **Current state**: Rust-first implementation direction is now declared, the active Cargo workspace root is `src/`, workspace checks are explicit, and the workset packets now reference the chosen stack
- **Exact unblock steps**: none; resolved by the Rust-first baseline update and execution-packet expansion
- **Recommendation**: workaround
- **Opened**: 2026-03-16
- **Resolved**: 2026-03-16

---

## Entry Template

```text
### BLK-REPLAY-NNN: <title>

- **Status**: active | resolved | closed
- **Impact**: <which worksets/features are blocked>
- **Current state**: <what has been attempted, what failed>
- **Exact unblock steps**: <specific actions needed>
- **Recommendation**: wait | escalate | workaround
- **Opened**: YYYY-MM-DD
- **Resolved**: YYYY-MM-DD (if applicable)
```
