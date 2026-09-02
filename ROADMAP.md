# Adze roadmap

Milestones are ordered; each is done when its exit criteria hold, not when its
boxes are ticked. The **CURRENT STEP** marker below is the single source of truth
for what the next session does. Move it when the step is finished.

> **CURRENT STEP:** M0.2 — Spike: persistent (structurally shared) mesh storage
> versus a plain mutable BMesh. Build both behind the `adze-mesh` surface, measure
> adjacency query cost and memory on a 1M-face mesh, write
> `docs/research/persistent-mesh.md`, and settle D5.

## Session and model guidance
- **Clear the session when Claude says so.** A session ends when its step is
  recorded in the repo (marker moved, notes written). Claude's last message says
  "safe to clear" or names what is still unrecorded.
- **Work in bursts.** The prompt cache lasts about an hour of idle time. Turns
  within a burst are cheap; the first turn after a long gap re-reads everything
  cold. Long sessions resumed after hours are the worst case for budget.
- **Fable (2x Opus per token)** for: architecture decisions, research digestion,
  the M0 spikes, data-structure design, bevel, booleans, live-history semantics.
- **Opus** for: scaffolding, CI, IO formats, primitives, tests, UI wiring, and any
  step whose design is already written down. Steps below are tagged.
- Faizal tracks weekly limits and stops for the week when they get critical.

---

## M0. Foundations and spikes
Exit: D5 and D6 are decided with measurements written to `docs/research/`.
- [x] M0.1 Workspace skeleton (D18), CI on three platforms, first property test [Opus]
- [ ] M0.2 Spike: persistent mesh storage vs mutable BMesh, 1M faces, query cost (D5) [Fable]
- [ ] M0.3 Spike: integer-lattice coordinates, rotation rounding policy, unit model (D6) [Fable]
- [ ] M0.4 Determinism harness: same op sequence hashed on all platforms and wasm (D7) [Opus]
- [ ] M0.5 Research notes: BMesh/OpenMesh comparison, OpenSubdiv crease rules [Fable]

## M1. Mesh kernel
Exit: OBJ round-trips on a corpus of 50 real meshes; property tests green for 10k random op sequences.
- [ ] Topology: verts, edges, faces (n-gons), non-manifold tolerated, stable IDs [Fable]
- [ ] Typed attribute columns: position, normal, UV, crease, groups, custom [Opus]
- [ ] Invariant checker (manifoldness, orphan elements, attribute lengths) [Opus]
- [ ] OBJ import/export (D8) [Opus]
- [ ] Primitives: plane, cube, cylinder, sphere, torus [Opus]
- [ ] Headless viewer (native only) rendering wireframe + shaded, for eyeballing [Opus]

## M2. Selection and the tool pipe
Exit: move/rotate/scale with three action centers and three falloffs feel right to two ex-Modo testers.
- [ ] Selection modes: vertex, edge, face; loops, rings, grow/shrink, invert [Opus]
- [ ] Selection stored as rules + concrete IDs (survives topology change) [Fable]
- [ ] Tool pipe traits: action × action center × falloff × symmetry × snapping [Fable]
- [ ] Transform tools with in-viewport handles [Opus]
- [ ] Op log with snapshots; undo/redo via replay (D2) [Fable]
- [ ] Autosave of op log (native: file; web: origin private file system) [Opus]

## M3. Core ops
Exit: a hard-surface prop (e.g. a hinge) can be modeled start to finish.
- [ ] Extrude, inset, loop cut, knife, bridge, merge, dissolve, edge slide, connect [Opus]
- [ ] Differential tests against Blender headless for each op (D13) [Opus]
- [ ] Web build (wasm + WebGPU, WebGL2 fallback) with the same tools [Opus]
- [ ] Artifact-hosted demo writing op logs and comments to its database [Opus]

## M4. Bevel and subdivision
Exit: bevel handles the standard corner test set without artifacts; SubD preview interactive at 1M faces on the Intel iGPU.
- [ ] Bevel with deterministic output topology and stable derived IDs [Fable]
- [ ] Catmull-Clark on the GPU matching OpenSubdiv creases (D9) [Fable]
- [ ] Symmetry, snapping, workplane [Opus]
- [ ] Geodesic (heat-method) falloff on the GPU [Fable]
- [ ] First testers on the web demo; op-log review sessions [Opus]

## M5. Live history and booleans
Exit: change a bevel segment count 40 ops back and the model survives; a boolean seam is SubD-ready on the test set.
- [ ] Live ops with incremental re-evaluation (salsa or equivalent) [Fable]
- [ ] Fail-soft reference loss: evaluate with found elements, highlight missing, repick [Fable]
- [ ] Delta layers for post-op tweaks, re-projected on topology change [Fable]
- [ ] Squash history range into a baked op [Opus]
- [ ] Boolean research and implementation (D16) [Fable]
- [ ] Crease-to-holding-edge derivation [Fable]

## M6. Exchange and UVs
Exit: a model round-trips Adze → USD → Blender → USD → Adze with creases and groups intact.
- [ ] USD read/write (D8)
- [ ] glTF export
- [ ] UV editor with xatlas unwrapping, seams, basic layout tools
- [ ] Material slots, basic PBR assignment, vertex colors
- [ ] Blender reload addon; watched-folder export

## M7. Public web release
Exit: product on its own domain; 30 days without a data-loss report.
- [x] Licence decision (D15, settled early by D19: AGPL-3.0-or-later, public repo)
- [ ] Self-hosted PWA with cross-origin isolation headers, threads enabled
- [ ] File System Access save/export; download fallback for Firefox
- [ ] Keymap audited against browser-owned shortcuts
- [ ] Command palette
- [ ] Devlog and public repo

## M8. Native downloads
Exit: signed builds on three platforms with auto-update.
- [ ] Apple developer account, notarization; Windows signing (Azure Trusted Signing)
- [ ] GitHub release builds, self-update
- [ ] Opt-in crash report = op log upload

## M9. AI layer and extension
- [ ] Natural-language commands over the command system (D12)
- [ ] Macro synthesis from description
- [ ] Semantic selection (feature recognition: holes, fillets, panels)
- [ ] WASM plugin host (D11)
- [ ] Branching history; multiplayer experiment on the op log

## Parked ideas (not scheduled)
Sculpting. Constraint solver for polygons (coplanar, equal length). Sketch-driven
op inference. Topological symmetry detection. Local-model modeling copilot trained
on consented op logs.
