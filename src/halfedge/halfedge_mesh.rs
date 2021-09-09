use std::collections::HashMap;

use super::{FaceRef, HalfEdgeRef, VertexRef};

pub struct HalfEdgeMesh<VData, EData, FData> {
    pub(crate) vertices: Vec<VertexRef>,
    pub(crate) halfedges: Vec<HalfEdgeRef>,
    pub(crate) faces: Vec<FaceRef>,
    vertices_data: Vec<VData>,
    edges_data: Vec<EData>,
    faces_data: Vec<FData>,
    token: u128,
}

impl<VData, EData, FData> HalfEdgeMesh<VData, EData, FData>
where
    VData: Default,
    EData: Default,
    FData: Default,
{
    /// create a halfedge mesh from topology (`in_faces`) and data
    ///
    /// Notice:
    /// * if `(u, v)` is key of `in_edges_data`, `u < v` must be hold
    pub fn new(
        in_faces: Vec<Vec<usize>>,
        mut in_vertices_data: HashMap<usize, VData>,
        mut in_edges_data: HashMap<(usize, usize), EData>,
        mut in_faces_data: HashMap<usize, FData>,
    ) -> Self {
        let token = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

        let num_vertices_input = *in_faces.iter().flatten().max().unwrap();

        let mut vertices = Vec::with_capacity(num_vertices_input);
        let mut vertices_map = HashMap::with_capacity(num_vertices_input);
        let mut vertices_data = Vec::with_capacity(num_vertices_input);
        let mut faces = Vec::with_capacity(in_faces.len());
        let mut faces_data = Vec::with_capacity(in_faces.len());
        let mut num_halfedges = 0;

        for (fid, face) in in_faces.iter().enumerate() {
            let he_face = FaceRef {
                id: fid,
                halfedge: usize::MAX,
                is_boundary: false,
                token,
            };
            faces.push(he_face);
            faces_data.push(in_faces_data.remove(&fid).unwrap_or(FData::default()));

            for vid_input in face {
                if !vertices_map.contains_key(vid_input) {
                    let vert = VertexRef {
                        id: vertices.len(),
                        halfedge: usize::MAX,
                        token,
                    };
                    vertices_map.insert(vid_input, vert.id);
                    vertices_data.push(in_vertices_data.remove(&vert.id).unwrap_or(VData::default()));
                    vertices.push(vert);
                }
            }

            num_halfedges += face.len();
        }

        let mut edges_data = Vec::with_capacity(num_halfedges / 2);
        let mut edges_map = HashMap::with_capacity(num_halfedges);
        for face in &in_faces {
            for i in 0..face.len() {
                let a = face[i];
                let b = if i + 1 == face.len() {
                    face[0]
                } else {
                    face[i + 1]
                };

                let key = (a.min(b), a.max(b));
                if !edges_map.contains_key(&key) {
                    edges_map.insert(key, edges_data.len());
                    edges_data.push(in_edges_data.remove(&key).unwrap_or(EData::default()));
                }
            }
        }

        let mut halfedges: Vec<HalfEdgeRef> = Vec::with_capacity(num_halfedges);
        let mut halfedges_map = HashMap::with_capacity(num_halfedges);

        for (fid, face) in in_faces.iter().enumerate() {
            let mut face_halfedges = Vec::with_capacity(face.len());

            for i in 0..face.len() {
                let a = face[i];
                let b = if i + 1 == face.len() {
                    face[0]
                } else {
                    face[i + 1]
                };

                let va = *vertices_map.get(&a).unwrap();

                let edge_key = (a.min(b), a.max(b));
                let mut he = HalfEdgeRef {
                    id: halfedges.len(),
                    edge: *edges_map.get(&edge_key).unwrap(),
                    next: usize::MAX,
                    twin: usize::MAX,
                    vertex: va,
                    face: fid,
                    token,
                };
                halfedges_map.insert((a, b), he.id);

                vertices[va].halfedge = he.id;
                faces[fid].halfedge = he.id;

                if let Some(twin_he_id) = halfedges_map.get(&(b, a)) {
                    he.twin = *twin_he_id;
                    halfedges[*twin_he_id].twin = he.id;
                }

                face_halfedges.push(he.id);
                halfedges.push(he);
            }

            for i in 0..face.len() {
                let he_id = face_halfedges[i];
                let next_he_id = if i + 1 == face.len() {
                    face_halfedges[0]
                } else {
                    face_halfedges[i + 1]
                };
                halfedges[he_id].next = next_he_id;
            }
        }

        for v in &mut vertices {
            let mut he = v.halfedge;
            loop {
                if halfedges[he].twin == usize::MAX {
                    v.halfedge = he;
                    break;
                }

                he = halfedges[halfedges[he].twin].next;
                if he == v.halfedge {
                    break;
                }
            }
        }

        for he_id in 0..num_halfedges {
            if halfedges[he_id].twin == usize::MAX {
                let fake_face = FaceRef {
                    id: faces.len(),
                    halfedge: halfedges.len(),
                    is_boundary: true,
                    token,
                };

                let mut boundary_edges = vec![];
                let mut it = he_id;
                loop {
                    let he = HalfEdgeRef {
                        id: halfedges.len(),
                        edge: halfedges[it].edge,
                        next: usize::MAX,
                        twin: it,
                        vertex: halfedges[halfedges[it].next].vertex,
                        face: fake_face.id,
                        token,
                    };
                    halfedges[it].twin = he.id;
                    boundary_edges.push(he.id);
                    halfedges.push(he);

                    it = halfedges[it].next;
                    while it != he_id && halfedges[it].twin < halfedges.len() {
                        it = halfedges[halfedges[it].twin].next;
                    }

                    if it == he_id {
                        break;
                    }
                }

                for i in 0..boundary_edges.len() {
                    let he = boundary_edges[i];
                    halfedges[he].next = if i + 1 == boundary_edges.len() {
                        boundary_edges[0]
                    } else {
                        boundary_edges[i + 1]
                    };
                }

                faces.push(fake_face);
            } 
        }

        for v in &mut vertices {
            v.halfedge = halfedges[halfedges[v.halfedge].twin].next;
        }

        Self {
            vertices,
            halfedges,
            faces,
            vertices_data,
            edges_data,
            faces_data,
            token,
        }
    }

    pub fn is_vertex_ref_valid(&self, vref: &VertexRef) -> bool {
        self.token == vref.token && vref.id < self.vertices.len()
    }

    pub fn vertex_data(&self, vref: &VertexRef) -> &VData {
        assert!(self.is_vertex_ref_valid(vref));
        &self.vertices_data[vref.id]
    }

    pub fn vertices(&self) -> &[VertexRef] {
        &self.vertices
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_halfedge_ref_valid(&self, heref: &HalfEdgeRef) -> bool {
        self.token == heref.token && heref.id < self.halfedges.len()
    }

    pub fn edge_data(&self, heref: &HalfEdgeRef) -> &EData {
        assert!(self.is_halfedge_ref_valid(heref));
        &self.edges_data[heref.edge]
    }

    pub fn halfedges(&self) -> &[HalfEdgeRef] {
        &self.halfedges
    }

    pub fn num_edges(&self) -> usize {
        self.halfedges.len() / 2
    }

    pub fn is_face_ref_valid(&self, fref: &FaceRef) -> bool {
        self.token == fref.token && fref.id < self.faces.len()
    }

    pub fn face_data(&self, fref: &FaceRef) -> &FData {
        assert!(self.is_face_ref_valid(fref));
        &self.faces_data[fref.id]
    }

    pub fn faces(&self) -> &[FaceRef] {
        &self.faces
    }

    /// Notice: this method is not O(1)
    pub fn num_faces(&self) -> usize {
        self.faces.iter().filter(|f| !f.is_boundary).count()
    }

    pub fn num_faces_with_boundary(&self) -> usize {
        self.faces.len()
    }
}
