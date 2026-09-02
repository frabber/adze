//! The headless utility: validate, convert, replay (D10).
//!
//! A stub with one real job so far, so the binary target is exercised by CI
//! from the start.

use adze_mesh::primitives;
use adze_mesh::{CheckLevel, check};

fn main() {
    let cube = primitives::cube(1.0);
    let violations = check(&cube, CheckLevel::ClosedManifold);
    println!(
        "adze {} — cube: {} verts, {} edges, {} faces, {} violations",
        env!("CARGO_PKG_VERSION"),
        cube.vertex_count(),
        cube.edge_count(),
        cube.face_count(),
        violations.len(),
    );
}
