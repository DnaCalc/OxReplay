# DEC-002 — Replay Plumbing For Undo/Redo And Macro Recording

## 1. Decision id
`DEC-002`

## 2. Date
`2026-03-23`

## 3. Scope
Forward-looking repo-local vision for `OxReplay`.

## 4. Vision
`OxReplay` should become the plumbing layer that makes two later system-wide abilities practical:
1. undo and redo over retained state-changing actions,
2. macro recording over captured user-intent and replayable action streams.

The core idea is that a system able to retain, normalize, replay, diff, and explain action-bearing evidence is also building the substrate needed for reversible action history and durable macro capture.

## 5. Boundary
This is not a claim that `OxReplay` itself becomes the end-user undo stack or macro product surface.

The intended split is:
1. `OxReplay` provides the retained action, provenance, replay, diff, and witness plumbing,
2. product or host layers provide the user-facing undo/redo UX and macro-recording UX,
3. lane repos remain the owners of semantic meaning for their actions and safe reversibility rules.

## 6. Why this matters
If the replay substrate is designed well enough, undo/redo and macro recording do not need to be separate ad hoc systems.

They can grow out of the same underlying capacities:
1. stable action capture,
2. retained provenance,
3. replay-safe normalization,
4. explicit lossiness markers,
5. explainable divergence and failure handling.

## 7. Impact
1. adapter boundaries should preserve enough intent and action identity for later reversibility work,
2. replay bundles should not assume they are only for forensic failure analysis,
3. future host and lane work may build undo/redo and macro features over this plumbing without moving semantic ownership into `OxReplay`.

## 8. Cautions
This is a good fit only if the ownership split stays explicit:
1. `OxReplay` is the shared action, history, provenance, and replay substrate,
2. hosts own undo/redo UX and macro-recording UX,
3. lane repos own semantic meaning, safe reversibility rules, and action interpretation.

Generic replay does not automatically imply semantic reversibility.

For this direction to remain sound:
1. lossiness and uncertainty must stay explicit,
2. not every captured action stream should be assumed safe for deterministic reversal,
3. not every captured action stream should be assumed faithful for macro playback without lane-declared rules.

The direction becomes unsound if `OxReplay` starts pretending that generic replay alone is enough to define undo semantics or macro semantics.

## 9. Handoff requirement
No immediate Foundation or sibling handoff is required.

This note is a local direction-setting statement only.
