mod halfedge;
mod trimesh;

pub use halfedge::*;
pub use trimesh::*;

use std::{borrow::Cow, collections::HashMap};

#[derive(Debug)]
pub enum Property {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
    F64(f64),
    I8List(Vec<i8>),
    U8List(Vec<u8>),
    I16List(Vec<i16>),
    U16List(Vec<u16>),
    I32List(Vec<i32>),
    U32List(Vec<u32>),
    F32List(Vec<f32>),
    F64List(Vec<f64>),
}

#[derive(Default)]
pub struct PropertyMap {
    pub map: HashMap<Cow<'static, str>, Property>,
}

impl ply_rs::ply::PropertyAccess for PropertyMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set_property(&mut self, key: String, property: ply_rs::ply::Property) {
        let prop = match property {
            ply_rs::ply::Property::Char(val) => Property::I8(val),
            ply_rs::ply::Property::UChar(val) => Property::U8(val),
            ply_rs::ply::Property::Short(val) => Property::I16(val),
            ply_rs::ply::Property::UShort(val) => Property::U16(val),
            ply_rs::ply::Property::Int(val) => Property::I32(val),
            ply_rs::ply::Property::UInt(val) => Property::U32(val),
            ply_rs::ply::Property::Float(val) => Property::F32(val),
            ply_rs::ply::Property::Double(val) => Property::F64(val),
            ply_rs::ply::Property::ListChar(val) => Property::I8List(val),
            ply_rs::ply::Property::ListUChar(val) => Property::U8List(val),
            ply_rs::ply::Property::ListShort(val) => Property::I16List(val),
            ply_rs::ply::Property::ListUShort(val) => Property::U16List(val),
            ply_rs::ply::Property::ListInt(val) => Property::I32List(val),
            ply_rs::ply::Property::ListUInt(val) => Property::U32List(val),
            ply_rs::ply::Property::ListFloat(val) => Property::F32List(val),
            ply_rs::ply::Property::ListDouble(val) => Property::F64List(val),
        };
        self.map.insert(key.into(), prop);
    }
}

fn get_property_type(prop: &Property) -> ply_rs::ply::PropertyType {
    match prop {
        Property::I8(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Char),
        Property::U8(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::UChar),
        Property::I16(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Short),
        Property::U16(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::UShort),
        Property::I32(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Int),
        Property::U32(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::UInt),
        Property::F32(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Float),
        Property::F64(_) => ply_rs::ply::PropertyType::Scalar(ply_rs::ply::ScalarType::Double),
        Property::I8List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::Char,
        ),
        Property::U8List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::UChar,
        ),
        Property::I16List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::Short,
        ),
        Property::U16List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::UShort,
        ),
        Property::I32List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::Int,
        ),
        Property::U32List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::UInt,
        ),
        Property::F32List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::Float,
        ),
        Property::F64List(_) => ply_rs::ply::PropertyType::List(
            ply_rs::ply::ScalarType::UChar,
            ply_rs::ply::ScalarType::Double,
        ),
    }
}

fn get_ply_property(prop: &Property) -> ply_rs::ply::Property {
    match prop {
        Property::I8(val) => ply_rs::ply::Property::Char(*val),
        Property::U8(val) => ply_rs::ply::Property::UChar(*val),
        Property::I16(val) => ply_rs::ply::Property::Short(*val),
        Property::U16(val) => ply_rs::ply::Property::UShort(*val),
        Property::I32(val) => ply_rs::ply::Property::Int(*val),
        Property::U32(val) => ply_rs::ply::Property::UInt(*val),
        Property::F32(val) => ply_rs::ply::Property::Float(*val),
        Property::F64(val) => ply_rs::ply::Property::Double(*val),
        Property::I8List(val) => ply_rs::ply::Property::ListChar(val.clone()),
        Property::U8List(val) => ply_rs::ply::Property::ListUChar(val.clone()),
        Property::I16List(val) => ply_rs::ply::Property::ListShort(val.clone()),
        Property::U16List(val) => ply_rs::ply::Property::ListUShort(val.clone()),
        Property::I32List(val) => ply_rs::ply::Property::ListInt(val.clone()),
        Property::U32List(val) => ply_rs::ply::Property::ListUInt(val.clone()),
        Property::F32List(val) => ply_rs::ply::Property::ListFloat(val.clone()),
        Property::F64List(val) => ply_rs::ply::Property::ListDouble(val.clone()),
    }
}
