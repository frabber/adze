//! Constructed meshes.
//!
//! Only the box lives here for now: the primitive set proper (plane, cube,
//! cylinder, sphere, torus) is M1 work, and until then this exists to give the
//! invariant checker something real to chew on. Faces wind counter-clockwise
//! seen from outside, so the enclosed volume is positive.

use std::collections::BTreeMap;

use crate::mesh::{Mesh, Point, Scalar};

/// Parameters of an axis-aligned box centred on the origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxParams {
    /// Full extent along x, y, z.
    pub size: [Scalar; 3],
    /// Quad divisions along each axis. Every entry is clamped to at least one,
    /// so the default is a plain six-quad cube.
    pub segments: [u32; 3],
}

impl Default for BoxParams {
    fn default() -> Self {
        Self {
            size: [1.0, 1.0, 1.0],
            segments: [1, 1, 1],
        }
    }
}

impl BoxParams {
    pub fn cube(size: Scalar) -> Self {
        Self {
            size: [size; 3],
            ..Self::default()
        }
    }

    pub fn with_segments(self, segments: [u32; 3]) -> Self {
        Self { segments, ..self }
    }

    fn divisions(&self) -> [u32; 3] {
        self.segments.map(|n| n.max(1))
    }

    /// Vertices a box with these parameters will have: the grid points on the
    /// surface, that is, all lattice points minus the interior ones.
    pub fn expected_vertex_count(&self) -> usize {
        let [nx, ny, nz] = self.divisions().map(u64::from);
        let all = (nx + 1) * (ny + 1) * (nz + 1);
        let interior = nx.saturating_sub(1) * ny.saturating_sub(1) * nz.saturating_sub(1);
        (all - interior) as usize
    }

    /// Quads a box with these parameters will have.
    pub fn expected_face_count(&self) -> usize {
        let [nx, ny, nz] = self.divisions().map(u64::from);
        (2 * (nx * ny + ny * nz + nz * nx)) as usize
    }
}

/// A box as a closed, consistently wound quad mesh.
pub fn box_mesh(params: BoxParams) -> Mesh {
    let divisions = params.divisions();
    let mut mesh = Mesh::new();

    // Lattice coordinate to vertex, so the six sides share their seams instead
    // of each minting its own rim. Ordered, never a hash map (D7).
    let mut lattice: BTreeMap<[u32; 3], _> = BTreeMap::new();
    let mut vertex_at = |mesh: &mut Mesh, cell: [u32; 3]| {
        *lattice.entry(cell).or_insert_with(|| {
            let position: Point = std::array::from_fn(|axis| {
                let t = Scalar::from(cell[axis]) / Scalar::from(divisions[axis]);
                params.size[axis] * (t - 0.5)
            });
            mesh.add_vertex(position)
        })
    };

    // For each of the three axes, the two sides perpendicular to it. `axis` is
    // the side normal; `u` and `v` span it, ordered so that u x v points along
    // +axis, which makes the u,v quad loop wind outward on the far side.
    for axis in 0..3 {
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;
        for (layer, outward) in [(0, false), (divisions[axis], true)] {
            for i in 0..divisions[u] {
                for j in 0..divisions[v] {
                    let cell = |du: u32, dv: u32| {
                        let mut cell = [0u32; 3];
                        cell[axis] = layer;
                        cell[u] = i + du;
                        cell[v] = j + dv;
                        cell
                    };
                    let mut quad = [
                        vertex_at(&mut mesh, cell(0, 0)),
                        vertex_at(&mut mesh, cell(1, 0)),
                        vertex_at(&mut mesh, cell(1, 1)),
                        vertex_at(&mut mesh, cell(0, 1)),
                    ];
                    if !outward {
                        quad.reverse();
                    }
                    mesh.add_face(&quad).expect("box quads are well formed");
                }
            }
        }
    }

    mesh
}

/// A cube of the given edge length, centred on the origin.
pub fn cube(size: Scalar) -> Mesh {
    box_mesh(BoxParams::cube(size))
}

/// Signed volume enclosed by the mesh, by the divergence theorem over a fan
/// triangulation of each face. Positive when the faces wind outward, which is
/// how a test tells an inside-out box from a correct one.
pub fn signed_volume(mesh: &Mesh) -> Scalar {
    let mut total = 0.0;
    for (_, face) in mesh.faces() {
        let Some(&origin) = face.verts.first() else {
            continue;
        };
        let Some(a) = mesh.vertex(origin).map(|v| v.position) else {
            continue;
        };
        for corner in 1..face.verts.len().saturating_sub(1) {
            let (Some(b), Some(c)) = (
                mesh.vertex(face.verts[corner]).map(|v| v.position),
                mesh.vertex(face.verts[corner + 1]).map(|v| v.position),
            ) else {
                continue;
            };
            total += triple_product(a, b, c) / 6.0;
        }
    }
    total
}

fn triple_product(a: Point, b: Point, c: Point) -> Scalar {
    a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0])
}
