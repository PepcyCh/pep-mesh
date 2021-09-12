use super::{FaceRef, HalfEdgeMesh, HalfEdgeRef};

pub(crate) struct Vertex {
    pub(crate) id: usize,
    pub(crate) halfedge: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexRef {
    pub(crate) id: usize,
    pub(crate) token: u128,
}

impl VertexRef {
    pub fn data<'a, VData, EData, FData>(
        &self,
        mesh: &'a HalfEdgeMesh<VData, EData, FData>,
    ) -> &'a VData {
        assert!(mesh.is_vertex_ref_valid(self));
        mesh.vertex_data(self)
    }

    pub fn data_mut<'a, VData, EData, FData>(
        &self,
        mesh: &'a mut HalfEdgeMesh<VData, EData, FData>,
    ) -> &'a mut VData {
        assert!(mesh.is_vertex_ref_valid(self));
        mesh.vertex_data_mut(self)
    }

    pub fn halfedge<VData, EData, FData>(
        &self,
        mesh: &HalfEdgeMesh<VData, EData, FData>,
    ) -> HalfEdgeRef {
        assert!(mesh.is_vertex_ref_valid(self));
        let halfedge = mesh.vertices[self.id].halfedge;
        assert!(halfedge < mesh.halfedges.len());
        HalfEdgeRef {
            id: halfedge,
            token: self.token,
        }
    }

    pub fn face<'a, VData, EData, FData>(
        &self,
        mesh: &HalfEdgeMesh<VData, EData, FData>,
    ) -> FaceRef {
        assert!(mesh.is_vertex_ref_valid(self));
        self.halfedge(mesh).face(mesh)
    }

    pub fn on_boundary<VData, EData, FData>(
        &self,
        mesh: &HalfEdgeMesh<VData, EData, FData>,
    ) -> bool {
        assert!(mesh.is_vertex_ref_valid(self));
        let mut he = self.halfedge(mesh);
        loop {
            if he.on_boundary(mesh) {
                return true;
            }
            he = he.twin(mesh).next(mesh);
            if he == self.halfedge(mesh) {
                break;
            }
        }
        false
    }

    pub fn degree<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> u32 {
        assert!(mesh.is_vertex_ref_valid(self));
        let mut degree = 0;
        let mut he = self.halfedge(mesh);
        loop {
            degree += 1;
            he = he.twin(mesh).next(mesh);
            if he == self.halfedge(mesh) {
                break;
            }
        }
        degree
    }

    pub fn set_halfedge<VData, EData, FData>(
        &self,
        mesh: &mut HalfEdgeMesh<VData, EData, FData>,
        halfedge: &HalfEdgeRef,
    ) {
        assert!(mesh.is_vertex_ref_valid(self) && mesh.is_halfedge_ref_valid(halfedge));
        mesh.vertices[self.id].halfedge = halfedge.id;
    }
}
