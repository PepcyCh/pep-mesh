use std::{borrow::Cow, collections::HashMap};

use super::VertexAttribute;

pub enum MeshIndices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

pub struct TriMesh {
    vertex_attributes: HashMap<Cow<'static, str>, VertexAttribute>,
    indices: Option<MeshIndices>,
}

impl TriMesh {
    pub fn attribute<Str: Into<Cow<'static, str>>>(
        &self,
        name: Str,
    ) -> Option<&VertexAttribute> {
        self.vertex_attributes.get(&name.into())
    }

    pub fn attribute_mut<Str: Into<Cow<'static, str>>>(
        &mut self,
        name: Str,
    ) -> Option<&mut VertexAttribute> {
        self.vertex_attributes.get_mut(&name.into())
    }

    pub fn indices(&self) -> Option<&MeshIndices> {
        self.indices.as_ref()
    }

    pub fn indices_mut(&mut self) -> Option<&mut MeshIndices> {
        self.indices.as_mut()
    }

    pub fn set_attribute<Str, Values>(&mut self, name: Str, values: Values)
    where
        Str: Into<Cow<'static, str>>,
        Values: Into<VertexAttribute>,
    {
        self.vertex_attributes.insert(name.into(), values.into());
    }

    pub fn set_indices(&mut self, indices: Option<MeshIndices>) {
        self.indices = indices;
    }
}