//! The invariant checker (D13).
//!
//! The kernel tolerates non-manifold geometry (D5); it is the checker, not the
//! structure, that says whether a mesh meets a stricter standard. Callers ask
//! for the level they need:
//!
//! * [`CheckLevel::Structural`] — the mesh is internally consistent. This must
//!   always hold; a violation here is a kernel bug, not user geometry.
//! * [`CheckLevel::Manifold`] — every edge is shared by at most two faces, the
//!   two agree on winding, and the faces around a vertex form a single fan.
//!   Boundaries are allowed.
//! * [`CheckLevel::ClosedManifold`] — additionally no boundary edges and no
//!   loose vertices: a watertight surface.
//!
//! Violations come back in a deterministic order (D7): the arena order of the
//! element each one is about.

use std::collections::{BTreeMap, BTreeSet};

use crate::id::{EdgeId, FaceId, VertexId};
use crate::mesh::{Face, Mesh};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckLevel {
    Structural,
    Manifold,
    ClosedManifold,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Violation {
    /// A face refers to a vertex that is not live.
    DeadVertexInFace { face: FaceId, vertex: VertexId },
    /// An edge refers to a face that is not live.
    DeadFaceOnEdge { edge: EdgeId, face: FaceId },
    /// An edge has a dead or repeated endpoint.
    MalformedEdge { edge: EdgeId },
    /// A face has fewer than three corners, or repeats one.
    MalformedFace { face: FaceId },
    /// A face loop uses a vertex pair with no edge, or an edge that does not
    /// list the face: the two sides of the edge/face relation disagree.
    MissingEdgeRecord {
        face: FaceId,
        from: VertexId,
        to: VertexId,
    },
    /// An edge that no face uses.
    WireEdge { edge: EdgeId },
    /// More than two faces meet at one edge.
    NonManifoldEdge { edge: EdgeId, faces: usize },
    /// Two faces traverse a shared edge in the same direction, so their normals
    /// disagree.
    InconsistentWinding { edge: EdgeId, faces: [FaceId; 2] },
    /// The faces around a vertex form more than one fan: two cones meeting at a
    /// point, or a bowtie.
    NonManifoldVertex { vertex: VertexId, fans: usize },
    /// An edge with only one face, where a closed surface was required.
    BoundaryEdge { edge: EdgeId },
    /// A vertex used by no face, where a closed surface was required.
    LooseVertex { vertex: VertexId },
}

/// Checks `mesh` against `level`. An empty result means the mesh passes.
pub fn check(mesh: &Mesh, level: CheckLevel) -> Vec<Violation> {
    let mut violations = Vec::new();
    check_structure(mesh, &mut violations);
    if level >= CheckLevel::Manifold {
        check_manifold(mesh, &mut violations);
    }
    if level >= CheckLevel::ClosedManifold {
        check_closed(mesh, &mut violations);
    }
    violations
}

fn check_structure(mesh: &Mesh, violations: &mut Vec<Violation>) {
    for (edge_id, edge) in mesh.edges() {
        if edge.verts[0] == edge.verts[1]
            || mesh.vertex(edge.verts[0]).is_none()
            || mesh.vertex(edge.verts[1]).is_none()
        {
            violations.push(Violation::MalformedEdge { edge: edge_id });
            continue;
        }
        for &face in &edge.faces {
            if mesh.face(face).is_none() {
                violations.push(Violation::DeadFaceOnEdge {
                    edge: edge_id,
                    face,
                });
            }
        }
        if edge.faces.is_empty() {
            violations.push(Violation::WireEdge { edge: edge_id });
        }
    }

    for (face_id, face) in mesh.faces() {
        let malformed = face.verts.len() < 3
            || face
                .verts
                .iter()
                .enumerate()
                .any(|(at, vertex)| face.verts[..at].contains(vertex));
        if malformed {
            violations.push(Violation::MalformedFace { face: face_id });
            continue;
        }
        for &vertex in &face.verts {
            if mesh.vertex(vertex).is_none() {
                violations.push(Violation::DeadVertexInFace {
                    face: face_id,
                    vertex,
                });
            }
        }
        for (from, to) in face_loop(&face.verts) {
            let recorded = mesh
                .edge_between(from, to)
                .and_then(|edge| mesh.edge(edge))
                .is_some_and(|edge| edge.faces.contains(&face_id));
            if !recorded {
                violations.push(Violation::MissingEdgeRecord {
                    face: face_id,
                    from,
                    to,
                });
            }
        }
    }
}

fn check_manifold(mesh: &Mesh, violations: &mut Vec<Violation>) {
    // Corner links: at each vertex, the pair of edges every incident face joins
    // there. A manifold vertex has all its incident edges in one chain.
    let mut corners: BTreeMap<VertexId, Vec<[EdgeId; 2]>> = BTreeMap::new();
    for (_, face) in mesh.faces() {
        let corner_count = face.verts.len();
        if corner_count < 3 {
            continue;
        }
        for (at, &vertex) in face.verts.iter().enumerate() {
            let previous = face.verts[(at + corner_count - 1) % corner_count];
            let next = face.verts[(at + 1) % corner_count];
            let (Some(before), Some(after)) = (
                mesh.edge_between(vertex, previous),
                mesh.edge_between(vertex, next),
            ) else {
                continue; // Already reported as a MissingEdgeRecord.
            };
            corners.entry(vertex).or_default().push([before, after]);
        }
    }

    for (vertex, links) in &corners {
        let fans = count_fans(links);
        if fans > 1 {
            violations.push(Violation::NonManifoldVertex {
                vertex: *vertex,
                fans,
            });
        }
    }

    for (edge_id, edge) in mesh.edges() {
        if edge.faces.len() > 2 {
            violations.push(Violation::NonManifoldEdge {
                edge: edge_id,
                faces: edge.faces.len(),
            });
            continue;
        }
        if let [first, second] = edge.faces[..]
            && let (Some(a), Some(b)) = (mesh.face(first), mesh.face(second))
            && traverses_forward(a, edge.verts) == traverses_forward(b, edge.verts)
        {
            violations.push(Violation::InconsistentWinding {
                edge: edge_id,
                faces: [first, second],
            });
        }
    }
}

fn check_closed(mesh: &Mesh, violations: &mut Vec<Violation>) {
    let used: BTreeSet<VertexId> = mesh
        .faces()
        .flat_map(|(_, face)| face.verts.iter().copied())
        .collect();
    for (vertex, _) in mesh.vertices() {
        if !used.contains(&vertex) {
            violations.push(Violation::LooseVertex { vertex });
        }
    }
    for (edge_id, edge) in mesh.edges() {
        if edge.faces.len() == 1 {
            violations.push(Violation::BoundaryEdge { edge: edge_id });
        }
    }
}

/// The directed pairs of a face loop, closing back to the first vertex.
fn face_loop(verts: &[VertexId]) -> impl Iterator<Item = (VertexId, VertexId)> + '_ {
    verts
        .iter()
        .zip(verts.iter().cycle().skip(1))
        .map(|(&from, &to)| (from, to))
}

/// Whether `face` traverses the edge from `verts[0]` to `verts[1]`.
fn traverses_forward(face: &Face, verts: [VertexId; 2]) -> bool {
    face_loop(&face.verts).any(|(from, to)| from == verts[0] && to == verts[1])
}

/// Number of connected chains formed by the corner links at one vertex. One
/// chain is a manifold fan (open or closed); more than one is a bowtie.
fn count_fans(links: &[[EdgeId; 2]]) -> usize {
    let mut edges: Vec<EdgeId> = links.iter().flatten().copied().collect();
    edges.sort_unstable();
    edges.dedup();

    fn find(parent: &mut [usize], mut node: usize) -> usize {
        while parent[node] != node {
            parent[node] = parent[parent[node]];
            node = parent[node];
        }
        node
    }

    let mut parent: Vec<usize> = (0..edges.len()).collect();
    let mut fans = edges.len();
    for link in links {
        let slot = |edge: &EdgeId| edges.binary_search(edge).expect("edge came from this list");
        let (a, b) = (
            find(&mut parent, slot(&link[0])),
            find(&mut parent, slot(&link[1])),
        );
        if a != b {
            parent[a] = b;
            fans -= 1;
        }
    }
    fans
}
