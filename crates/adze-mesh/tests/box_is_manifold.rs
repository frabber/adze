//! The first property test (D13): whatever box we build, it is a closed
//! manifold with the vertex, edge and face counts the parameters imply.
//!
//! A cube is the `segments: [1, 1, 1]` case, and it is checked on its own
//! below so a failure says plainly whether the cube broke or only a subdivided
//! box did.

use adze_mesh::primitives::{self, BoxParams};
use adze_mesh::{CheckLevel, Mesh, Scalar, check};
use proptest::prelude::*;

/// Every level of the checker, so a failure names the strongest thing that
/// still holds.
fn assert_closed_manifold(mesh: &Mesh) {
    for level in [
        CheckLevel::Structural,
        CheckLevel::Manifold,
        CheckLevel::ClosedManifold,
    ] {
        let violations = check(mesh, level);
        assert!(
            violations.is_empty(),
            "{level:?} check failed: {violations:#?}"
        );
    }
}

#[test]
fn a_cube_is_a_closed_manifold() {
    let cube = primitives::cube(2.0);

    assert_closed_manifold(&cube);
    assert_eq!(cube.vertex_count(), 8);
    assert_eq!(cube.edge_count(), 12);
    assert_eq!(cube.face_count(), 6);
    assert_eq!(cube.euler_characteristic(), 2);
    assert!((primitives::signed_volume(&cube) - 8.0).abs() < 1e-9);
}

#[test]
fn a_cube_has_a_three_way_fan_at_every_corner() {
    let cube = primitives::cube(1.0);
    for (_, edge) in cube.edges() {
        assert_eq!(
            edge.faces.len(),
            2,
            "every cube edge joins exactly two faces"
        );
    }
}

fn box_params() -> impl Strategy<Value = BoxParams> {
    (
        prop::array::uniform3(0.01f64..100.0),
        prop::array::uniform3(1u32..6),
    )
        .prop_map(|(size, segments)| BoxParams { size, segments })
}

proptest! {
    #[test]
    fn every_box_is_a_closed_manifold(params in box_params()) {
        let mesh = primitives::box_mesh(params);
        assert_closed_manifold(&mesh);
    }

    #[test]
    fn a_box_has_the_element_counts_its_parameters_imply(params in box_params()) {
        let mesh = primitives::box_mesh(params);

        prop_assert_eq!(mesh.vertex_count(), params.expected_vertex_count());
        prop_assert_eq!(mesh.face_count(), params.expected_face_count());
        // Euler holds for any closed surface of genus zero, so it pins the edge
        // count without a second formula to get wrong.
        prop_assert_eq!(mesh.euler_characteristic(), 2);
    }

    #[test]
    fn a_box_winds_outward_and_encloses_its_size(params in box_params()) {
        let mesh = primitives::box_mesh(params);

        let expected: Scalar = params.size.iter().product();
        let volume = primitives::signed_volume(&mesh);
        prop_assert!(
            (volume - expected).abs() <= expected * 1e-9,
            "volume {volume} is not the expected {expected}"
        );
    }

    /// Two builds of the same parameters agree element for element, IDs
    /// included (D7). The real determinism harness is M0.4; this is the
    /// same-process floor it builds on.
    #[test]
    fn building_a_box_twice_gives_the_same_mesh(params in box_params()) {
        let first = primitives::box_mesh(params);
        let second = primitives::box_mesh(params);

        let positions = |mesh: &Mesh| {
            mesh.vertices().map(|(id, v)| (id, v.position)).collect::<Vec<_>>()
        };
        let faces = |mesh: &Mesh| {
            mesh.faces().map(|(id, f)| (id, f.verts.clone())).collect::<Vec<_>>()
        };
        prop_assert_eq!(positions(&first), positions(&second));
        prop_assert_eq!(faces(&first), faces(&second));
    }
}
