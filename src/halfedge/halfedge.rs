use super::{FaceRef, HalfEdgeMesh, VertexRef};

#[derive(PartialEq, Eq, Hash)]
pub struct HalfEdgeRef {
    pub(crate) id: usize,
    pub(crate) edge: usize,
    pub(crate) next: usize,
    pub(crate) twin: usize,
    pub(crate) vertex: usize,
    pub(crate) face: usize,
    pub(crate) token: u128,
}

impl HalfEdgeRef {
    pub fn data<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a EData
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        mesh.edge_data(self)
    }

    pub fn vertex<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a VertexRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        &mesh.vertices[self.vertex]
    }
    
    pub fn face<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a FaceRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        &mesh.faces[self.face]
    }

    pub fn next<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a HalfEdgeRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        &mesh.halfedges[self.next]
    }

    pub fn twin<'a, VData, EData, FData>(&self, mesh: &'a HalfEdgeMesh<VData, EData, FData>) -> &'a HalfEdgeRef
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        &mesh.halfedges[self.twin]
    }

    pub fn on_boundary<VData, EData, FData>(&self, mesh: &HalfEdgeMesh<VData, EData, FData>) -> bool
    where
        VData: Default,
        EData: Default,
        FData: Default,
    {
        assert!(mesh.is_halfedge_ref_valid(self));
        self.face(mesh).is_boundary
    }
}