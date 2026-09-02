//! Mesh storage.
//!
//! Faces are n-gons stored as an ordered vertex loop. Edges are derived: adding
//! a face creates the edges it needs and records itself on each of them, which
//! is what makes the radial (BMesh-style) queries in [`crate::check`] possible
//! without a half-edge structure and its non-manifold fragility (D5).
//!
//! Nothing here is the final structure. The persistent-storage spike (M0.2)
//! decides what sits behind this surface.

use std::collections::BTreeMap;

use crate::arena::Arena;
use crate::id::{EdgeId, FaceId, VertexId};

/// Kernel coordinate scalar.
///
/// D6 is still leaning: if the integer-lattice spike (M0.3) wins, this becomes
/// a lattice integer and [`Point`] a lattice coordinate. Everything in the
/// kernel goes through these two aliases so that swap stays local.
pub type Scalar = f64;

/// A position in kernel space. See [`Scalar`].
pub type Point = [Scalar; 3];

#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: Point,
}

/// An undirected edge. `verts` is sorted, so an edge has one canonical key
/// regardless of which face created it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub verts: [VertexId; 2],
    /// Faces using this edge, in creation order. Two for a closed manifold
    /// edge, one on a boundary, more when the mesh is non-manifold — which the
    /// structure tolerates and the checker reports (D5).
    pub faces: Vec<FaceId>,
}

/// An n-gon, stored as its ordered vertex loop. Winding is counter-clockwise
/// seen from the front of the face.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Face {
    pub verts: Vec<VertexId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeshError {
    /// A face needs at least three corners.
    FaceTooSmall { corners: usize },
    /// The same vertex appears twice in one face loop.
    RepeatedVertexInFace { vertex: VertexId },
    /// A face referenced a vertex that is not live in this mesh.
    UnknownVertex { vertex: VertexId },
}

impl core::fmt::Display for MeshError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FaceTooSmall { corners } => {
                write!(f, "a face needs at least 3 corners, got {corners}")
            }
            Self::RepeatedVertexInFace { vertex } => {
                write!(f, "vertex {vertex:?} appears twice in one face loop")
            }
            Self::UnknownVertex { vertex } => write!(f, "vertex {vertex:?} is not in this mesh"),
        }
    }
}

impl std::error::Error for MeshError {}

#[derive(Default)]
pub struct Mesh {
    verts: Arena<VertexId, Vertex>,
    edges: Arena<EdgeId, Edge>,
    faces: Arena<FaceId, Face>,
    /// Canonical (sorted) vertex pair to edge. A `BTreeMap`, not a hash map:
    /// nothing in the kernel may iterate in hash order (D7).
    edge_index: BTreeMap<[VertexId; 2], EdgeId>,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, position: Point) -> VertexId {
        self.verts.insert(Vertex { position })
    }

    /// Adds an n-gon over an ordered vertex loop, creating any edges it needs.
    pub fn add_face(&mut self, loop_verts: &[VertexId]) -> Result<FaceId, MeshError> {
        if loop_verts.len() < 3 {
            return Err(MeshError::FaceTooSmall {
                corners: loop_verts.len(),
            });
        }
        for (position, &vertex) in loop_verts.iter().enumerate() {
            if !self.verts.contains(vertex) {
                return Err(MeshError::UnknownVertex { vertex });
            }
            if loop_verts[..position].contains(&vertex) {
                return Err(MeshError::RepeatedVertexInFace { vertex });
            }
        }

        let face = self.faces.insert(Face {
            verts: loop_verts.to_vec(),
        });
        for (&from, &to) in loop_verts.iter().zip(loop_verts.iter().cycle().skip(1)) {
            let edge = self.edge_between_or_insert(from, to);
            self.edges
                .get_mut(edge)
                .expect("edge was just resolved")
                .faces
                .push(face);
        }
        Ok(face)
    }

    /// Canonical key for the undirected edge between two vertices.
    fn edge_key(a: VertexId, b: VertexId) -> [VertexId; 2] {
        if a <= b { [a, b] } else { [b, a] }
    }

    /// The edge between two vertices, if one exists.
    pub fn edge_between(&self, a: VertexId, b: VertexId) -> Option<EdgeId> {
        self.edge_index.get(&Self::edge_key(a, b)).copied()
    }

    fn edge_between_or_insert(&mut self, a: VertexId, b: VertexId) -> EdgeId {
        let key = Self::edge_key(a, b);
        if let Some(&edge) = self.edge_index.get(&key) {
            return edge;
        }
        let edge = self.edges.insert(Edge {
            verts: key,
            faces: Vec::new(),
        });
        self.edge_index.insert(key, edge);
        edge
    }

    pub fn vertex(&self, id: VertexId) -> Option<&Vertex> {
        self.verts.get(id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn face(&self, id: FaceId) -> Option<&Face> {
        self.faces.get(id)
    }

    pub fn vertex_count(&self) -> usize {
        self.verts.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Live vertices in a stable order.
    pub fn vertices(&self) -> impl Iterator<Item = (VertexId, &Vertex)> {
        self.verts.iter()
    }

    /// Live edges in a stable order.
    pub fn edges(&self) -> impl Iterator<Item = (EdgeId, &Edge)> {
        self.edges.iter()
    }

    /// Live faces in a stable order.
    pub fn faces(&self) -> impl Iterator<Item = (FaceId, &Face)> {
        self.faces.iter()
    }

    /// Euler characteristic V - E + F. Two for a closed surface of genus zero.
    pub fn euler_characteristic(&self) -> i64 {
        self.vertex_count() as i64 - self.edge_count() as i64 + self.face_count() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle(mesh: &mut Mesh) -> [VertexId; 3] {
        [
            mesh.add_vertex([0.0, 0.0, 0.0]),
            mesh.add_vertex([1.0, 0.0, 0.0]),
            mesh.add_vertex([0.0, 1.0, 0.0]),
        ]
    }

    #[test]
    fn a_face_creates_its_edges_once() {
        let mut mesh = Mesh::new();
        let v = triangle(&mut mesh);
        let d = mesh.add_vertex([1.0, 1.0, 0.0]);

        mesh.add_face(&[v[0], v[1], v[2]]).unwrap();
        assert_eq!(mesh.edge_count(), 3);

        // Shares the v1-v2 edge with the first face.
        mesh.add_face(&[v[1], d, v[2]]).unwrap();
        assert_eq!(mesh.edge_count(), 5);

        let shared = mesh.edge_between(v[1], v[2]).unwrap();
        assert_eq!(mesh.edge(shared).unwrap().faces.len(), 2);
    }

    #[test]
    fn edges_are_undirected() {
        let mut mesh = Mesh::new();
        let v = triangle(&mut mesh);
        mesh.add_face(&[v[0], v[1], v[2]]).unwrap();
        assert_eq!(mesh.edge_between(v[0], v[1]), mesh.edge_between(v[1], v[0]));
    }

    #[test]
    fn malformed_faces_are_rejected() {
        let mut mesh = Mesh::new();
        let v = triangle(&mut mesh);

        assert_eq!(
            mesh.add_face(&[v[0], v[1]]),
            Err(MeshError::FaceTooSmall { corners: 2 })
        );
        assert_eq!(
            mesh.add_face(&[v[0], v[1], v[0]]),
            Err(MeshError::RepeatedVertexInFace { vertex: v[0] })
        );

        // Far enough along in its own mesh that its slot does not exist here.
        let mut other = Mesh::new();
        let stranger = (0..5)
            .map(|n| other.add_vertex([n as Scalar, 0.0, 0.0]))
            .last()
            .unwrap();
        assert_eq!(
            mesh.add_face(&[v[0], v[1], stranger]),
            Err(MeshError::UnknownVertex { vertex: stranger })
        );
        assert_eq!(mesh.face_count(), 0, "no face survives a rejected add");
        assert_eq!(mesh.edge_count(), 0);
    }
}
