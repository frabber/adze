//! Negative cases for the invariant checker. Without these, the manifold
//! property test would pass just as happily against a checker that never
//! reports anything.

use adze_mesh::{CheckLevel, Mesh, Violation, check};

/// Two quads that meet at one vertex only: manifold edges everywhere, but the
/// shared vertex has two fans.
#[test]
fn a_bowtie_vertex_is_non_manifold() {
    let mut mesh = Mesh::new();
    let waist = mesh.add_vertex([0.0, 0.0, 0.0]);
    let left = [
        mesh.add_vertex([-1.0, 0.0, 0.0]),
        mesh.add_vertex([-1.0, 1.0, 0.0]),
        mesh.add_vertex([0.0, 1.0, 0.0]),
    ];
    let right = [
        mesh.add_vertex([1.0, 0.0, 0.0]),
        mesh.add_vertex([1.0, -1.0, 0.0]),
        mesh.add_vertex([0.0, -1.0, 0.0]),
    ];
    mesh.add_face(&[waist, left[0], left[1], left[2]]).unwrap();
    mesh.add_face(&[waist, right[0], right[1], right[2]])
        .unwrap();

    assert_eq!(check(&mesh, CheckLevel::Structural), vec![]);
    assert_eq!(
        check(&mesh, CheckLevel::Manifold),
        vec![Violation::NonManifoldVertex {
            vertex: waist,
            fans: 2
        }]
    );
}

/// Three triangles hinged on one edge: the T-junction case a half-edge
/// structure cannot even represent (D5).
#[test]
fn three_faces_on_one_edge_is_non_manifold() {
    let mut mesh = Mesh::new();
    let hinge = [
        mesh.add_vertex([0.0, 0.0, 0.0]),
        mesh.add_vertex([0.0, 1.0, 0.0]),
    ];
    let wings = [
        mesh.add_vertex([1.0, 0.0, 0.0]),
        mesh.add_vertex([-1.0, 0.0, 0.0]),
        mesh.add_vertex([0.0, 0.0, 1.0]),
    ];
    for wing in wings {
        mesh.add_face(&[hinge[0], hinge[1], wing]).unwrap();
    }

    assert_eq!(check(&mesh, CheckLevel::Structural), vec![]);
    let edge = mesh.edge_between(hinge[0], hinge[1]).unwrap();
    let violations = check(&mesh, CheckLevel::Manifold);
    assert!(
        violations.contains(&Violation::NonManifoldEdge { edge, faces: 3 }),
        "expected a non-manifold edge, got {violations:#?}"
    );
}

/// Two triangles sharing an edge, the second wound the same way round it, so
/// their normals point opposite ways.
#[test]
fn a_flipped_neighbour_is_caught() {
    let mut mesh = Mesh::new();
    let shared = [
        mesh.add_vertex([0.0, 0.0, 0.0]),
        mesh.add_vertex([1.0, 0.0, 0.0]),
    ];
    let up = mesh.add_vertex([0.0, 1.0, 0.0]);
    let down = mesh.add_vertex([0.0, -1.0, 0.0]);

    let first = mesh.add_face(&[shared[0], shared[1], up]).unwrap();
    let second = mesh.add_face(&[shared[0], shared[1], down]).unwrap();

    let edge = mesh.edge_between(shared[0], shared[1]).unwrap();
    assert_eq!(
        check(&mesh, CheckLevel::Manifold),
        vec![Violation::InconsistentWinding {
            edge,
            faces: [first, second]
        }]
    );
}

/// A single triangle is a perfectly good manifold with boundary, and not a
/// closed one.
#[test]
fn a_lone_triangle_is_manifold_but_not_closed() {
    let mut mesh = Mesh::new();
    let corners = [
        mesh.add_vertex([0.0, 0.0, 0.0]),
        mesh.add_vertex([1.0, 0.0, 0.0]),
        mesh.add_vertex([0.0, 1.0, 0.0]),
    ];
    mesh.add_face(&corners).unwrap();
    let loose = mesh.add_vertex([5.0, 5.0, 5.0]);

    assert_eq!(check(&mesh, CheckLevel::Manifold), vec![]);

    let violations = check(&mesh, CheckLevel::ClosedManifold);
    assert!(violations.contains(&Violation::LooseVertex { vertex: loose }));
    assert_eq!(
        violations
            .iter()
            .filter(|v| matches!(v, Violation::BoundaryEdge { .. }))
            .count(),
        3
    );
}
