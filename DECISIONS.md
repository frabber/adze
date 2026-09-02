# Adze decisions log

One entry per architectural choice. Newer entries at the bottom. An entry is never
edited after it is made; if a decision changes, add a new entry that supersedes it.
Status is one of: **decided**, **leaning** (needs a prototype or spike before it is
final), **pending** (must be decided before a named milestone), **superseded by Dn**.

Format: context, decision, alternatives rejected, consequences.

---

## D1. Language and core stack — decided (2026-09-02)

**Context.** Modo is discontinued and was unstable. Blender needs paid addons for
hard-surface work. Stability is the headline feature of Adze.

**Decision.** Rust throughout. wgpu for the viewport, egui for the UI shell, winit
for windowing. No game engine.

**Rejected.** Bevy: its ECS is a poor fit for one large mutable mesh, and we want
full control of the render pipeline for overlays, selection, and subdivision.
C++: throws away the memory-safety argument that is the point of the project.

**Consequences.** egui is a pragmatic choice, not a final one; a custom UI layer
may replace it once the tools are proven. Everything compiles to wasm and native
from one codebase.

## D2. The op log is the document — decided (2026-09-02)

**Context.** Modifier stacks cannot reference elements; feature timelines break on
topological naming; Plasticity chose no history and users still ask for it.

**Decision.** Every modeling action is a pure, serializable operation. The saved
file is the ordered log of operations plus periodic snapshots. Undo is replay from
the nearest snapshot. History is always recorded; an op is *live* (re-evaluated
when its parameters change) only when the user promotes it. A range of history can
be squashed into one baked op while the original log is retained.

**Rejected.** Mutable mesh with an undo stack of deltas (no history, no replay,
no crash repro). Fully parametric everything (history hell, slow rebuilds).

**Consequences.** Every crash report is a replayable test case. Multiplayer and
branching become feasible later. The kernel must be deterministic (see D7).
Hand-tweaks after a live op are stored as delta layers keyed by stable IDs, not as
replayed move ops.

## D3. Command system — decided (2026-09-02)

**Decision.** Every UI action is a named command with typed, serializable
arguments (as in Modo's command system and Blender's operators). The UI only ever
invokes commands. Macros, scripting, the CLI, the command palette, tests, and the
AI layer all sit on the same command surface.

**Consequences.** No feature is done until it has a command. UI code contains no
modeling logic.

## D4. Web-first product, native as the power tier — decided (2026-09-02)

**Context.** Precedent: Onshape, Figma, Womp, Spline. Web removes signing, installers,
updates, and piracy. Artifact-hosted demo gives a feedback loop Claude can read.

**Decision.** The primary product is a self-hosted progressive web app. Native
builds arrive later from the same code. The kernel and shaders are designed to
web constraints as the baseline: wasm32 memory budgets, the WebGPU feature subset,
threads optional, file model built on File System Access handles and the origin
private file system with autosave of the op log.

**Rejected.** Native-first with a web port later (a rewrite, historically).

**Consequences.** The artifact-hosted demo is for feedback only; the product needs
its own domain for silent saves and cross-origin isolation headers. Default keymap
must avoid browser-owned shortcuts. A native download is a milestone gated on
stability, not a date (see ROADMAP M8).

## D5. Mesh structure — leaning (2026-09-02)

**Context.** Half-edge is fragile with non-manifold geometry. BMesh (radial edge)
handles n-gons and non-manifold. No shipped modeler uses a persistent
(structurally shared, immutable) mesh.

**Leaning.** BMesh-like topology (n-gons, non-manifold tolerant), stable
generational element IDs as primary keys (never raw indices), attributes stored as
typed columns (SoA), and *persistent* storage with structural sharing so each op
returns a new version cheaply.

**Spike required.** Prototype persistent chunked storage with adjacency and measure
query cost versus a plain mutable BMesh on a 1M-face mesh. Decide by end of M0.

**Rejected.** Textbook half-edge (non-manifold fragility). Raw index references
(break under every topology change, cause the topological naming problem).

## D6. Kernel coordinates — leaning (2026-09-02)

**Leaning.** Positions on a 64-bit integer lattice in the kernel, converted to f32
only for the GPU. Geometric predicates become exact, snap rounding is free,
cross-platform determinism follows.

**Spike required.** One-week prototype: lattice resolution, unit handling,
transforms (rotation on a lattice needs rounding policy), and whether artists could
ever notice. Fallback if it fails: f64 in the kernel. Decide by end of M0.

## D7. Bit-exact determinism — decided (2026-09-02)

**Decision.** The kernel produces identical results on Linux, Windows, macOS, and
wasm. No hash-map iteration order anywhere in the kernel; ordered containers or
sorted iteration only. Float operations, where they exist, are controlled
(no fast-math, no platform intrinsics). Randomness is seeded and recorded in the op.

**Consequences.** Op logs replay anywhere. Crash repros are exact. Required for
multiplayer and shared test corpora. Cheap now, impossible to retrofit.

## D8. Exchange formats — decided (2026-09-02)

**Decision.** USD's data model (polygons with n-gons, creases, subdivision scheme,
attributes, materials, hierarchy) is the reference for what a mesh may carry; the
kernel stores nothing it cannot export. OBJ import/export in M1. glTF export as the
game-engine target (triangles only, never an exchange format). USD read/write by
M6, binding the C++ library or a written subset. FBX read via ufbx when needed;
FBX write is deferred indefinitely.

**Renderers.** Never integrated directly. USD plus Hydra delegates cover V-Ray,
Arnold, Cycles, Redshift, Karma.

**Live links.** Save-to-watched-folder (engines auto-reimport). A tiny Blender
addon that reloads on hotkey. No sockets.

## D9. Subdivision semantics — decided (2026-09-02)

**Decision.** Catmull-Clark matching OpenSubdiv exactly: crease weights, boundary
rules, semi-sharp creases. Evaluated on the GPU as a compute shader within the
WebGPU subset. No custom schemes.

## D10. Product scope — decided (2026-09-02)

**In.** Polygon modeling, subdivision, UVs, material slots with basic PBR
assignment, vertex colors, export. Sculpting is a possible year-two addition.

**Out, permanently.** Rigging, animation, rendering beyond the viewport, texture
painting.

**Rejected.** A "Unix toolbox" of many small 3D apps (fragments a solo effort;
artists will not stitch apps together). The Unix idea applies underneath instead:
the kernel is a crate, a CLI ships alongside the app (remesh, validate, convert,
replay), and the op-log format is the ecosystem.

## D11. Plugins — decided (2026-09-02)

**Decision.** No plugin API for the first two years. Extension is via the kernel
crate and the command system. When plugins arrive they are WASM sandboxed via
wasmtime: they cannot crash the host or corrupt the mesh.

## D12. AI integration — decided (2026-09-02)

**Decision.** AI sits on top of the command system and the textual op-log DSL:
natural-language commands, macro synthesis, selection assistance, tool discovery.
Every model output is a command sequence validated before execution. No
text-to-mesh generation. Local models on the NVIDIA box for experiments; the
artifact `sample` capability for the web-demo prototype.

## D13. Testing strategy — decided (2026-09-02)

**Decision.** The kernel is headless and tested without a UI. Property tests
(proptest) apply random op sequences and assert manifold invariants, attribute
preservation, and export round-trips. Differential tests run Blender headless as an
oracle for bevel, subdivision, and booleans. Every panic in the app captures the op
log; every captured log becomes a regression test. Snapshot tests on mesh output.

## D14. Governance — decided (2026-09-02)

**Decision.** Cathedral core, bazaar edges. Design authority is Faizal and Claude.
Build in public: public repo, devlog, web demo. Community contributes bug repros as
op logs, test meshes, platform testing, and later WASM plugins. No feature voting on
the core. Signal from the crowd comes from usage in op logs, not from votes.

## D15. Licence — superseded by D19

**Options.** (a) Open source under MIT/Apache. (b) Source-available with a paid
hosted/native build, Plasticity-style. (c) Closed. Reading Blender code to learn is
fine under any option; porting Blender code forces GPL and removes (b) and (c).
Rust crates are almost all MIT/Apache, which keeps every option open.

## D16. Booleans — pending research (target M5)

**Direction.** Hybrid: a robust implicit/SDF cut, then a seam-aware local remesh
guided by surrounding edge flow, with holding edges derived from crease weights.
Goal: SubD-ready output along the seam. This is the flagship research item.

## D17. Hardware and test matrix — decided (2026-09-02)

Linux laptop (Arch, Intel Arc iGPU, high RAM): primary development, Vulkan/ANV.
Windows NUC (RTX 5070): DX12 backend, NVIDIA shader validation, performance
ceiling, local AI experiments, reachable over SSH. Mac mini M4: Metal backend,
Apple-silicon testing. GitHub Actions for builds and headless tests on all three.
Never develop GPU code inside WSL2.

## D18. Workspace layout — decided (2026-09-02)

Cargo workspace: `adze-mesh` (kernel: structure, attributes, IDs, invariants),
`adze-ops` (operations and the tool pipe: action × center × falloff × symmetry ×
snapping), `adze-doc` (op log, snapshots, replay, live ops, delta layers),
`adze-cmd` (command system, palette, macros), `adze-io` (formats),
`adze-render` (wgpu viewport, subdivision shaders, picking),
`adze-app` (egui shell, native and wasm targets), `adze-cli` (headless utility).
Dependencies flow strictly downward in that order; `adze-mesh` depends on nothing
above it and never on wgpu or egui.

## D19. Licence: AGPL-3.0-or-later, public from the start — decided (2026-09-02)

Supersedes D15, which parked this until M7. It is settled early because the
answer turned out to be knowable now and the repository cannot go public until
it is.

**Context.** D15 kept three options open: permissive open source, source-available
with a paid tier, or closed. Adze will never be closed source, which removes (c).
That leaves the question of which open licence protects a product whose primary
form is a hosted web application (D4). It does not have to wait for M7: nothing
learned between here and there changes the answer, and staying private until then
costs CI minutes and the build-in-public loop of D14.

**Decision.** AGPL-3.0-or-later, copyright held solely by Faizal Abdoelrahman.
The repository is public from now, not from M7. Code contributions require a CLA;
the contributions D14 actually invites — bug repros as op logs, test meshes,
platform testing — do not, as they are not copyrightable core work.

**Rejected.**
- *MIT/Apache.* Adze is a web app. Permissive licensing means a competitor can
  host it unmodified and owe nothing, which gives away the product itself rather
  than the source.
- *GPL-3.0.* Copyleft triggers on distribution, and running a service is not
  distribution. For a hosted product it is close to no protection at all. AGPL
  section 13 is the clause that matters here, and it is the only reason to prefer
  AGPL over GPL.
- *BUSL 1.1 with a timed conversion to Apache-2.0.* Encodes "never unavailable"
  as a date and protects hosting revenue harder, but it is not open source, and
  the Rust and graphics communities treat that as a meaningful difference. Kept
  in mind as the fallback if hosted revenue ever needs stronger protection; note
  that relicensing later is only possible while the copyright stays undivided,
  which is what the CLA requirement protects.

**Consequences.**
- The commercial tier D15 wanted from option (b) survives without a
  source-available licence: sole copyright means a non-AGPL licence can be sold
  to anyone who needs to embed Adze. Accepting a code PR without a CLA destroys
  that, permanently and quietly.
- Porting Blender code is now permitted. Blender is GPLv2-or-later, usable as
  GPLv3, and GPLv3 and AGPLv3 are explicitly cross-compatible, so D15's warning
  that porting "removes (b) and (c)" no longer applies. This is worth most to
  M4 bevel and M5 booleans. Tempered: Blender's bevel does not produce the
  deterministic output topology and stable derived IDs the roadmap requires, so
  the value is in consulting a battle-tested implementation, not in wholesale
  porting.
- Some studios and large companies refuse AGPL software outright. That is a real
  limit on adoption in exactly the professional market Adze aims at, and the
  answer to those users is a commercial licence, not a licence change.
- GitHub Actions is unmetered on public repositories, so the full three-platform
  matrix of D17 runs on every push at no cost. Docs-only pushes are filtered out
  of CI to keep the signal clean, not to save minutes.
- The M0-M5 development history is public as it happens, which is the D14
  intent, brought forward.
- Rust crates are almost entirely MIT/Apache and combine into an AGPL work
  without friction, so nothing in the dependency tree is affected.
