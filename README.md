# Adze

A polygon modeler in Rust, web-first. Stability is the feature: every modeling
action is a pure, serializable operation, so a crash is a replayable test case
and the kernel behaves identically on every platform.

Not usable yet. See `ROADMAP.md` for where it is, `DECISIONS.md` for why it is
built this way, and `docs/research/` for what has been read.

## Workspace

Dependencies flow strictly downward through this list (D18):

| Crate | Holds |
|---|---|
| `adze-mesh` | The kernel: topology, stable element IDs, attributes, the invariant checker. Depends on nothing. |
| `adze-ops` | Operations and the tool pipe: action x centre x falloff x symmetry x snapping. |
| `adze-doc` | The document: op log, snapshots, replay, live ops, delta layers. |
| `adze-cmd` | The command system: named commands with typed arguments, macros, the palette. |
| `adze-io` | Exchange formats: OBJ, later USD and glTF. |
| `adze-render` | The wgpu viewport, subdivision shaders, picking. The only crate that may use wgpu. |
| `adze-app` | The egui shell, native and wasm. No modeling logic. |
| `adze-cli` | The headless utility: validate, convert, replay. |

## Building

```sh
cargo test --workspace --all-targets   # everything is headless-testable (D13)
cargo run -p adze-cli
```

CI runs the same tests on Linux, Windows and macOS, and checks that everything
below the UI still compiles for `wasm32-unknown-unknown` (D4, D17).

## Licence

AGPL-3.0-or-later. Copyright (C) 2026 Faizal Abdoelrahman. See `LICENSE`.

Adze is primarily a hosted web application, and the AGPL's section 13 is the
only copyleft that reaches that form: run a modified Adze as a service and you
owe your users the source. Self-host it, fork it, sell what you model with it —
all fine.

The copyright is held by one person, so a commercial licence for anyone who
needs to embed Adze without the AGPL's obligations is available on request. Code
contributions therefore need a CLA; bug repros, test meshes and platform testing
do not (D14, D19).
