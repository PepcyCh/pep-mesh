use super::{HalfEdgeMesh, HalfEdgeRef, VertexRef};

pub(crate) struct Face {
    pub(crate) id: usize,
    pub(crate) halfedge: usize,
    pub(crate) is_boundary: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceRef {
    pub(crate) id: usize,
    pub(crate) token: u128,
}

impl FaceRef {
    pub fn is_boundary<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> bool {
        assert!(mesh.is_face_ref_valid(self));
        mesh.faces[self.id].is_boundary
    }
    
    pub fn data<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a FData {
        assert!(mesh.is_face_ref_valid(self));
        mesh.face_data(self)
    }

    pub fn data_mut<'a, VData, EData, FData>(&self, mesh: &'a mut HalfEdgeMesh<VData, EData, FData>) -> &'a mut FData {
        assert!(mesh.is_face_ref_valid(self));
        mesh.face_data_mut(self)
    }

    pub fn halfedge<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> HalfEdgeRef {
        assert!(mesh.is_face_ref_valid(self));
        let halfedge = mesh.faces[self.id].halfedge;
        assert!(halfedge < mesh.halfedges.len());
        HalfEdgeRef {
            id: halfedge,
            token: self.token,
        }
    }
    
    pub fn vertex<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> VertexRef {
        self.halfedge(mesh).vertex(mesh)
    }

    pub fn degree<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> u32 {
        assert!(mesh.is_face_ref_valid(self));
        let mut degree = 0;
        let mut he = self.halfedge(mesh);
        loop {
            degree += 1;
            he = he.next(mesh);
            if he == self.halfedge(mesh) {
                break;
            }
        }
        degree
    }

    pub fn set_halfedge<VData, EData, FData>(&self, mesh: &mut HalfEdgeMesh<VData, EData, FData>, halfedge: &HalfEdgeRef) {
        assert!(mesh.is_face_ref_valid(self) && mesh.is_halfedge_ref_valid(halfedge));
        mesh.faces[self.id].halfedge = halfedge.id;
    }
}