# Contributing to Adze

Thank you for looking. Please read the first two sections before spending any
time — they will probably save you some.

## Where the project actually is

Adze is at milestone M0. There is no application: no viewport, no tools, no file
format, nothing to run but a test suite. What exists is a mesh kernel that can
build a box and prove it is manifold.

`ROADMAP.md` carries a **CURRENT STEP** marker that is the single source of truth
for what is being worked on. `DECISIONS.md` explains why the project is built the
way it is; most questions about the architecture are answered there.

The honest summary is that **there is very little to contribute to right now**,
and saying so is better than accepting effort that goes nowhere.

## How this project is run

Adze is a cathedral core with bazaar edges (D14). Design authority sits with the
maintainer. This is deliberate: the project's headline feature is stability, and
a small, opinionated core with a coherent architecture is how a solo effort gets
there. Concretely:

- **The core is not open to feature voting.** Requests to reprioritise the
  roadmap will be read and will not usually change it.
- **Decided entries in `DECISIONS.md` are not re-litigated.** If you have
  *evidence* that contradicts one — a measurement, a counterexample, a paper —
  that is genuinely valuable. Open an issue presenting the evidence. Decisions
  change by a new entry that supersedes the old one, never by editing it.
- **Signal comes from usage, not votes.** Once op logs exist, they are the
  feedback channel that carries the most weight.

## What actually helps

In rough order of usefulness today:

1. **Platform testing.** `D17` names Linux/Intel Arc, Windows/RTX 5070 and macOS
   M4 as the test matrix. If you have hardware outside it — AMD, Windows on ARM,
   an older macOS, a BSD — running `cargo test --workspace --all-targets` and
   reporting what happens is useful *now*, before there is anything else to do.
2. **Determinism reports.** From M0.4 there is a harness that hashes the result
   of an op sequence. The kernel is required to be bit-exact everywhere (D7). A
   platform where the hash differs is the single most valuable bug you can file.
3. **Test meshes.** From M1 the kernel needs a corpus of real production
   topology: n-gons, poles, mixed quads and triangles, boundaries, and the
   non-manifold junk that real files actually contain. See the licensing note
   below — contributed meshes must be CC0.
4. **Bug reports.** Once there is something to run, and later once op logs exist,
   a recorded op log is the ideal bug report: it replays exactly (D2).

## What does not help right now

- Feature requests for milestones beyond the current one.
- Large unsolicited pull requests. Open an issue first; a big PR against a moving
  architecture is usually wasted work for both of us.
- Drive-by refactors and style changes.
- New dependencies. Every one is a wasm binary-size and audit cost (D4), and the
  bar is high.

## If you do send code

The project's own rules apply to contributions:

- **Everything is headless-testable.** No modeling logic in UI code (D3).
- **The kernel is deterministic.** No hash-map iteration anywhere in
  `adze-mesh`, `adze-ops` or `adze-doc`; use ordered containers or sort before
  iterating (D7).
- **Dependencies flow downward** through the crate order in D18. `adze-mesh`
  depends on nothing and never on wgpu or egui.
- **Prefer a property test over an example test** for any operation (D13).
- **Design to web constraints first** (D4): wasm32 memory budgets, the WebGPU
  feature subset.
- `cargo fmt --all`, `cargo clippy --workspace --all-targets` and
  `cargo test --workspace --all-targets` must all be clean. CI runs them on
  Linux, Windows and macOS and checks that the kernel still builds for wasm32.

## The Contributor Licence Agreement

**Code contributions require agreement to the CLA in [`CLA.md`](CLA.md).**

Adze is AGPL-3.0-or-later. The copyright is held by one person, which is what
makes it possible to also sell a commercial licence to anyone who needs to embed
Adze without the AGPL's obligations. That revenue is what pays for the project to
continue. If contributed code arrived under the AGPL alone, that option would be
gone permanently the first time a patch was merged, because relicensing would
require tracking down every past contributor.

Being straight about the asymmetry: the CLA gives the maintainer rights over your
contribution that you do not get over the rest of the project. Some people object
to CLAs for exactly that reason, and that is a reasonable position. What the CLA
does **not** do is take your copyright away — you keep it, and you can do anything
you like with your own code elsewhere.

**How to accept.** In your first code pull request, add a line for yourself to
`CONTRIBUTORS.md` in the same PR. That line is your acceptance, and the commit is
the dated record of it.

**What does not need a CLA:** issues, bug reports, discussion, and documentation
corrections.

### Contributed meshes, op logs and other assets

A 3D model is a creative work and carries copyright like anything else, so test
meshes and recorded op logs need a licence too. To keep the test corpus usable
without tracking permissions, **contributed meshes and op logs must be dedicated
to the public domain under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/)**.

Say so in the pull request, and only contribute assets you made yourself or that
were already CC0 or public domain. Please do not contribute meshes from a client,
an employer, or an asset store, however incidental they seem.

## Licence

By contributing you agree that your contributions are licensed under
AGPL-3.0-or-later (see `LICENSE`), and, for code, under the additional terms of
`CLA.md`.
