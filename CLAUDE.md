# Adze — session instructions

Adze is a Rust polygon modeler, web-first, built by Claude with Faizal as sparring
partner. Read these before doing anything else in a session:

1. `ROADMAP.md` — find the **CURRENT STEP** marker. That is the session's job unless
   Faizal says otherwise. Move the marker when the step is done.
2. `DECISIONS.md` — the architectural decisions. Do not re-litigate a *decided*
   entry; if new evidence contradicts one, add a superseding entry rather than
   editing it. Decisions marked *leaning* or *pending* name the milestone by which
   they must be settled.
3. `docs/research/README.md` — index of research notes. Read the note for a topic
   before reading papers on it; write the note (conclusion first, then links)
   when a research task finishes so nothing is re-derived.

## Session ritual
**Start:** Faizal types `/continue` (or just "continue"). Claude follows the
`continue` skill: read the marker, check the model tag, state the step, begin.

**End:** the last message of every session finishes with this handoff block,
filled in, and nothing after it:

```
--- handoff ---
Done: <one line, what was completed and recorded>
Marker: <the new CURRENT STEP id and title>
Uncommitted: <none | list of files>
Safe to clear: <yes | no, because ...>
Next session: /clear  →  /model <opus|fable>  →  /continue
```

`/model` in Claude Code also saves that model as the default for new sessions,
so the next session opens on the right model automatically.

## Working rules
- One goal per session. Finish it, record it, stop. End every session by moving
  the ROADMAP marker and stating whether it is "safe to clear" the session.
- Model choice per step is tagged in ROADMAP.md ([Fable]/[Opus]); say at the start
  of a session if the current step is tagged for the other model.
- Every op and command is headless-testable. No modeling logic in UI code (D3).
- The kernel is deterministic (D7): no HashMap iteration in `adze-mesh`/`adze-ops`/
  `adze-doc`; use ordered containers or sort before iterating.
- Design to web constraints first (D4): wasm32 memory, WebGPU feature subset.
- Dependencies flow downward through the crates in D18. `adze-mesh` never depends
  on wgpu or egui.
- Prefer a property test over an example test for any op.
- Claude makes design calls and says when a call is about feel rather than
  architecture; feel is validated by tester op logs, not by argument.
- Commits: only when Faizal asks, always GPG-signed with the YubiKey (repo-local
  `commit.gpgsign=true`, gmail identity). The PIN dialog is on the desktop; if it
  times out because Faizal is away, report it and retry later. A decision or roadmap change is
  committed separately from code.

## Machines
Linux laptop (Intel Arc, primary dev), Windows NUC with RTX 5070 (SSH, DX12,
performance), Mac mini M4 (Metal). See D17.
