use super::{HalfEdgeMesh, HalfEdgeRef, VertexRef};

#[derive(PartialEq, Eq, Hash)]
pub struct FaceRef {
    pub(crate) id: usize,
    pub(crate) halfedge: usize,
    pub(crate) is_boundary: bool,
    pub(crate) token: u128,
}

impl FaceRef {
    pub fn is_boundary(&self) -> bool {
        self.is_boundary
    }
    
    pub fn data<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a FData
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_face_ref_valid(self));
        mesh.face_data(self)
    }

    pub fn halfedge<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a HalfEdgeRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_face_ref_valid(self));
        &mesh.halfedges[self.halfedge]
    }
    
    pub fn vertex<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a VertexRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_face_ref_valid(self));
        self.halfedge(mesh).vertex(mesh)
    }

    pub fn degree<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> u32
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
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
}