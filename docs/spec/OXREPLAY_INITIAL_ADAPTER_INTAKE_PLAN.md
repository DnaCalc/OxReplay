# OXREPLAY_INITIAL_ADAPTER_INTAKE_PLAN.md

## 1. Position
This document defines the first adapter-intake plan for `OxCalc` and `OxFml`.

It is a local planning companion for `W004`.
It does not redefine lane adapter semantics.

## 2. Intake sequence
1. intake `OxCalc` first because Foundation already treats it as the first expected lane toward higher replay capability growth
2. intake `OxFml` second in the same workset, with initial emphasis on ingest/replay/diff/explain rather than distillation
3. defer `OxFunc` and `OxVba` to later narrower lanes after the first two lanes have stabilized inside `OxReplay`

## 3. OxCalc intake expectations
`OxReplay` expects these source surfaces from `OxCalc`:
1. replay-facing scenario ids and replay-class bindings
2. source labels plus normalized event-family projection
3. retained views, reject sets, counters, and diff surfaces
4. adapter capability manifest and registry pin

Current planning scenario:
1. `oxcalc_tracecalc_accept_publish_001`

Current retained source and observations:
1. source anchor: `../OxCalc/docs/test-corpus/core-engine/tracecalc/hand-auditable/tc_accept_publish_001.json`
2. current alias mapping: `tc_accept_publish_001` -> `oxcalc_tracecalc_accept_publish_001`
3. current conformance result in `OxReplay`: manifest loads, but current `cap.C4.distill_valid` claim is rejected locally until lifecycle states are declared

Questions to answer during activation:
1. which retained `TraceCalc` artifacts should be mirrored locally versus referenced remotely
2. whether `engine_diff` is already sufficient for shared diff intake or requires a narrower projection pass
3. which `OxCalc` capability level is honestly claimable at intake time

## 4. OxFml intake expectations
`OxReplay` expects these source surfaces from `OxFml`:
1. typed session, candidate, commit, and reject projections
2. fixture-family import discipline with stable scenario ids
3. adapter capability manifest and registry pin
4. explicit current limits on replay-safe transforms and distillation claims

Current planning scenario:
1. `oxfml_fec_accept_publication_001`

Current retained source and observations:
1. source anchor: `../OxFml/crates/oxfml_core/tests/fixtures/fec_commit_replay_cases.json` with case id `fec_001_accept`
2. current alias mapping: `fec_001_accept` -> `oxfml_fec_accept_publication_001`
3. current conformance result in `OxReplay`: manifest passes the local `C0` through `C3` validator floor and keeps `C4` scaffolded

Questions to answer during activation:
1. which fixture family should act as the first retained shared replay import
2. what the first shared explain surface should consume
3. what remains local-only evidence versus retained shared replay evidence

## 5. Shared intake invariants
1. `OxReplay` consumes declared adapter meaning; it does not reinterpret lane semantics
2. source scenario ids remain authoritative
3. local retained imports must preserve source-lane identity and schema lineage
4. any normative pressure discovered during intake must route back through outbound notes or handoff packets

## 6. Planned outbound observation triggers
Write or update outbound notes if intake discovers:
1. missing adapter manifest fields needed by shared validation
2. missing source identity fields needed by shared replay/explain
3. lane-local naming drift that requires explicit adapter normalization
4. lifecycle or registry requirements that are not yet exposed by the lane repo

Current triggered outbound notes:
1. `NOTES_FOR_OXCALC.md` for the current `C4` lifecycle-state gap and retained alias mapping
2. `NOTES_FOR_OXFML.md` for the retained case-id alias mapping and first shared replay intake expectations
