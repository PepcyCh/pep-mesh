pub enum VertexAttribute {
    Float(Vec<f32>),
    Float2(Vec<[f32; 2]>),
    Float3(Vec<[f32; 3]>),
    Float4(Vec<[f32; 4]>),
    Int(Vec<i32>),
    Int2(Vec<[i32; 2]>),
    Int3(Vec<[i32; 3]>),
    Int4(Vec<[i32; 4]>),
}

impl VertexAttribute {
    pub fn len(&self) -> usize {
        match self {
            VertexAttribute::Float(val) => val.len(),
            VertexAttribute::Float2(val) => val.len(),
            VertexAttribute::Float3(val) => val.len(),
            VertexAttribute::Float4(val) => val.len(),
            VertexAttribute::Int(val) => val.len(),
            VertexAttribute::Int2(val) => val.len(),
            VertexAttribute::Int3(val) => val.len(),
            VertexAttribute::Int4(val) => val.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(feature = "bytemuck")]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            VertexAttribute::Float(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Float2(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Float3(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Float4(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Int(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Int2(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Int3(val) => bytemuck::cast_slice(&val),
            VertexAttribute::Int4(val) => bytemuck::cast_slice(&val),
        }
    }
}