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
PR: <url and state, or none>
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
- Commits on a step branch are how the work is recorded and do not need a
  separate ask; anything landing on `main` does, and normally lands by Faizal
  merging the PR. Always GPG-signed with the YubiKey (repo-local
  `commit.gpgsign=true`, gmail identity). The PIN dialog is on the desktop; if it
  times out because Faizal is away, report it and retry later. A decision or
  roadmap change is committed separately from code.
- Keep a step small enough that its diff can be read in one sitting. If a step
  would land much more than a few hundred lines, split it and say so.

## Code review
Faizal reads far more of this code than he writes, and the point is to
understand the Rust in detail, not only the architecture. The review surface is
RustRover's Pull Requests window, not the GitHub web UI.

- **One branch and one PR per roadmap step.** CI must be green before the PR is
  ready. The ROADMAP marker moves when the PR merges, not when the code is
  written.
- **The PR description is written for a reader learning Rust.** Doc comments
  explain the API to someone using it; the PR description explains why the
  implementation is shaped the way it is. Name the language decisions in the
  diff and the reasoning behind them — variance and marker types, what the
  borrow checker forced, why a macro instead of a generic, edition-2024
  constructs — and say which parts deserve close attention and which are
  mechanical. Do not move this material into source comments; it would bury the
  code.
- **Unanswered review comments are picked up at the start of the next session**,
  before new work.
- Faizal asks for walkthroughs of existing code whenever he wants one; that is
  ordinary session work and does not need a roadmap entry.

## Machines
Linux laptop (Intel Arc, primary dev), Windows NUC with RTX 5070 (SSH, DX12,
performance), Mac mini M4 (Metal). See D17.
