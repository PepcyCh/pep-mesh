use std::{borrow::Cow, collections::HashMap};

use super::VertexAttribute;

pub enum MeshIndices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

pub struct TriMesh {
    pub(crate) vertex_attributes: HashMap<Cow<'static, str>, VertexAttribute>,
    pub(crate) indices: Option<MeshIndices>,
}

impl TriMesh {
    pub const POSITION: &'static str = "position";
    pub const NORMAL: &'static str = "normal";
    pub const TANGENT: &'static str = "tangent";
    pub const BITANGENT: &'static str = "bitangent";
    pub const COLOR: &'static str = "color";
    pub const TEXCOORD: &'static str = "texcoord";

    pub fn new(
        vertex_attributes: HashMap<Cow<'static, str>, VertexAttribute>,
        indices: Option<MeshIndices>,
    ) -> Self {
        Self {
            vertex_attributes,
            indices,
        }
    }

    pub fn attribute<Str: Into<Cow<'static, str>>>(&self, name: Str) -> Option<&VertexAttribute> {
        self.vertex_attributes.get(&name.into())
    }

    pub fn attribute_mut<Str: Into<Cow<'static, str>>>(
        &mut self,
        name: Str,
    ) -> Option<&mut VertexAttribute> {
        self.vertex_attributes.get_mut(&name.into())
    }

    pub fn num_vertices(&self) -> usize {
        self.vertex_attributes
            .values()
            .next()
            .map_or(0, |attrib| attrib.len())
    }

    pub fn indices(&self) -> Option<&MeshIndices> {
        self.indices.as_ref()
    }

    pub fn indices_mut(&mut self) -> Option<&mut MeshIndices> {
        self.indices.as_mut()
    }

    pub fn num_indices(&self) -> Option<usize> {
        self.indices.as_ref().map(|indices| indices.len())
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

impl MeshIndices {
    pub fn len(&self) -> usize {
        match self {
            MeshIndices::U16(val) => val.len(),
            MeshIndices::U32(val) => val.len(),
        }
    }

    pub fn get(&self, index: usize) -> usize {
        match self {
            MeshIndices::U16(val) => val[index] as usize,
            MeshIndices::U32(val) => val[index] as usize,
        }
    }

    pub fn set(&mut self, index: usize, val: usize) {
        match self {
            MeshIndices::U16(self_val) => self_val[index] = val as u16,
            MeshIndices::U32(self_val) => self_val[index] = val as u32,
        }
    }

    #[cfg(feature = "bytemuck")]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            MeshIndices::U16(val) => bytemuck::cast_slice(&val),
            MeshIndices::U32(val) => bytemuck::cast_slice(&val),
        }
    }
}
