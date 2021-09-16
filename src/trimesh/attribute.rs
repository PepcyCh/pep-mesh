#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttributeFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
}

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

    pub fn format(&self) -> VertexAttributeFormat {
        match self {
            VertexAttribute::Float(_) => VertexAttributeFormat::Float,
            VertexAttribute::Float2(_) => VertexAttributeFormat::Float2,
            VertexAttribute::Float3(_) => VertexAttributeFormat::Float3,
            VertexAttribute::Float4(_) => VertexAttributeFormat::Float4,
            VertexAttribute::Int(_) => VertexAttributeFormat::Int,
            VertexAttribute::Int2(_) => VertexAttributeFormat::Int2,
            VertexAttribute::Int3(_) => VertexAttributeFormat::Int3,
            VertexAttribute::Int4(_) => VertexAttributeFormat::Int4,
        }
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

macro_rules! impl_vertex_attributes_ctor {
    ( $( ( $name:expr, $enum:ident ) ),+ $(,)? ) => {
        $(
            paste::paste! {
                pub fn [<$name _with_capacity>](capacity: usize) -> Self {
                    VertexAttribute::$enum(Vec::with_capacity(capacity))
                }
            }
        )+
    };
}

impl VertexAttribute {
    impl_vertex_attributes_ctor! {
        (float, Float),
        (float2, Float2),
        (float3, Float3),
        (float4, Float4),
        (int, Int),
        (int2, Int2),
        (int3, Int3),
        (int4, Int4),
    }
}

macro_rules! impl_vertex_attributes_getter_setter {
    ( $( ( $name:expr, $type:ty, $enum:ident ) ),+ $(,)? ) => {
        $(
            paste::paste! {
                pub fn [<get_ $name>](&self, index: usize) -> Option<$type> {
                    if let VertexAttribute::$enum(val) = self {
                        Some(val[index])
                    } else {
                        None
                    }
                }

                pub fn [<set_ $name>](&mut self, index: usize, val: $type) {
                    if let VertexAttribute::$enum(self_val) = self {
                        self_val[index] = val;
                    }
                }

                pub fn [<push_ $name>](&mut self, val: $type) {
                    if let VertexAttribute::$enum(self_val) = self {
                        self_val.push(val);
                    }
                }
            }
        )+
    };
}

impl VertexAttribute {
    impl_vertex_attributes_getter_setter! {
        (float, f32, Float),
        (float2, [f32; 2], Float2),
        (float3, [f32; 3], Float3),
        (float4, [f32; 4], Float4),
        (int, i32, Int),
        (int2, [i32; 2], Int2),
        (int3, [i32; 3], Int3),
        (int4, [i32; 4], Int4),
    }
}
