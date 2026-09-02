//! The Adze mesh kernel.
//!
//! This crate owns mesh topology, stable element identity, attributes, and the
//! invariant checker. It depends on nothing else in the workspace and never on
//! `wgpu` or `egui` (D18).
//!
//! Two decisions are still open and are deliberately isolated here:
//!
//! * **D5 (mesh structure, leaning).** The storage below is a plain mutable
//!   arena mesh: enough to build a box and check it, not the final structure.
//!   The persistent-vs-mutable spike (roadmap M0.2) replaces the innards of
//!   [`Mesh`] without changing the ID or checker surface.
//! * **D6 (coordinates, leaning).** All positions go through [`Scalar`] and
//!   [`Point`]. If the integer-lattice spike (roadmap M0.3) wins, those two
//!   aliases change and the rest of the kernel follows.
//!
//! Determinism (D7) is a hard rule in this crate: no hash-map iteration. Ordered
//! containers only, and every public iterator yields elements in a stable order.
//!
//! ```
//! use adze_mesh::{CheckLevel, check, primitives};
//!
//! let cube = primitives::cube(2.0);
//! assert_eq!(cube.face_count(), 6);
//! assert_eq!(cube.euler_characteristic(), 2);
//! assert_eq!(check(&cube, CheckLevel::ClosedManifold), vec![]);
//! ```

pub mod arena;
pub mod check;
pub mod id;
pub mod mesh;
pub mod primitives;

pub use check::{CheckLevel, Violation, check};
pub use id::{EdgeId, ElementId, FaceId, VertexId};
pub use mesh::{Edge, Face, Mesh, MeshError, Point, Scalar, Vertex};
