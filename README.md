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
