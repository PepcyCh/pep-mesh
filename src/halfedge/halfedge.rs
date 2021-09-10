use super::{FaceRef, HalfEdgeMesh, VertexRef};

pub(crate) struct HalfEdge {
    pub(crate) id: usize,
    pub(crate) edge: usize,
    pub(crate) next: usize,
    pub(crate) twin: usize,
    pub(crate) vertex: usize,
    pub(crate) face: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalfEdgeRef {
    pub(crate) id: usize,
    pub(crate) token: u128,
}

impl HalfEdgeRef {
    pub fn data<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a EData {
        assert!(mesh.is_halfedge_ref_valid(self));
        mesh.edge_data(self)
    }

    pub fn data_mut<'a, VData, EData, FData>(&self, mesh: &'a mut HalfEdgeMesh<VData, EData, FData>) -> &'a mut EData {
        assert!(mesh.is_halfedge_ref_valid(self));
        mesh.edge_data_mut(self)
    }

    pub fn vertex<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> VertexRef {
        assert!(mesh.is_halfedge_ref_valid(self));
        let vertex = mesh.halfedges[self.id].vertex;
        assert!(vertex < mesh.vertices.len());
        VertexRef { id: vertex, token: self.token }
    }
    
    pub fn face<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> FaceRef {
        assert!(mesh.is_halfedge_ref_valid(self));
        let face = mesh.halfedges[self.id].face;
        assert!(face < mesh.faces.len());
        FaceRef { id: face, token: self.token }
    }

    pub fn next<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> HalfEdgeRef {
        assert!(mesh.is_halfedge_ref_valid(self));
        let next = mesh.halfedges[self.id].next;
        assert!(next < mesh.halfedges.len());
        HalfEdgeRef { id: next, token: self.token }
    }

    /// Notice: not O(1)
    pub fn last<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> HalfEdgeRef {
        let mut he = self.twin(mesh);
        loop {
            if he.next(mesh) == *self {
                return he;
            }
            he = he.next(mesh).twin(mesh);
        }
    }

    pub fn twin<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> HalfEdgeRef {
        assert!(mesh.is_halfedge_ref_valid(self));
        let twin = mesh.halfedges[self.id].twin;
        assert!(twin < mesh.halfedges.len());
        HalfEdgeRef { id: twin, token: self.token }
    }

    pub fn on_boundary<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> bool {
        self.face(mesh).is_boundary(mesh)
    }

    pub fn set_vertex<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, vertex: &VertexRef) {
        assert!(mesh.is_halfedge_ref_valid(self) && mesh.is_vertex_ref_valid(vertex));
        mesh.halfedges[self.id].vertex = vertex.id;
    }

    pub fn set_next<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, next: &HalfEdgeRef) {
        assert!(mesh.is_halfedge_ref_valid(self) && mesh.is_halfedge_ref_valid(next));
        mesh.halfedges[self.id].next = next.id;
    }

    /// set twin of this halfedge to be `twin`, set associated edge data to be that of `twin`
    pub fn set_twin<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, twin: &HalfEdgeRef) {
        assert!(mesh.is_halfedge_ref_valid(self) && mesh.is_halfedge_ref_valid(twin));
        mesh.halfedges[self.id].twin = twin.id;
        mesh.halfedges[self.id].edge = mesh.halfedges[twin.id].edge;
    }

    /// set twin of this halfedge to be `twin`, `self.data(mesh)` and `self.twin(mesh).data(mesh)` may be different after this
    pub fn set_twin_weak<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, twin: &HalfEdgeRef) {
        assert!(mesh.is_halfedge_ref_valid(self) && mesh.is_halfedge_ref_valid(twin));
        mesh.halfedges[self.id].twin = twin.id;
    }

    pub fn set_face<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, face: &FaceRef) {
        assert!(mesh.is_halfedge_ref_valid(self) && mesh.is_face_ref_valid(face));
        mesh.halfedges[self.id].face = face.id;
    }
}