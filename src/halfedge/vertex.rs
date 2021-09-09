use super::{FaceRef, HalfEdgeMesh, HalfEdgeRef};

#[derive(PartialEq, Eq, Hash)]
pub struct VertexRef {
    pub(crate) id: usize,
    pub(crate) halfedge: usize,
    pub(crate) token: u128,
}

impl VertexRef {
    pub fn data<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a VData
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_vertex_ref_valid(self));
        mesh.vertex_data(self)
    }

    pub fn halfedge<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a HalfEdgeRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_vertex_ref_valid(self));
        &mesh.halfedges[self.halfedge]
    }
    
    pub fn face<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a FaceRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_vertex_ref_valid(self));
        self.halfedge(mesh).face(mesh)
    }

    pub fn on_boundary<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> bool
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_vertex_ref_valid(self));
        let mut he = self.halfedge(mesh);
        loop {
            if he.on_boundary(mesh) {
                return true;
            }
            he = he.next(mesh);
            if he == self.halfedge(mesh) {
                break;
            }
        }
        false
    }

    pub fn degree<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> u32
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
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
}