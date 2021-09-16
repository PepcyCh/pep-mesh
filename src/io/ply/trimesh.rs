use std::{borrow::Cow, collections::HashMap};

use ply_rs::{
    parser,
    ply::{self, Addable},
    writer,
};

use crate::{
    io::{
        ply::{Property, PropertyMap},
        LoadError, SaveError,
    },
    trimesh::{MeshIndices, TriMesh, VertexAttribute},
};

pub fn load_to_trimesh<Path, VertexAttributesInitializer, VertexPropertiesHandler>(
    path: Path,
    vertex_attributes_initializer: VertexAttributesInitializer,
    vertex_properties_handler: VertexPropertiesHandler,
) -> Result<TriMesh, LoadError>
where
    Path: AsRef<std::path::Path>,
    VertexAttributesInitializer: Fn(usize) -> HashMap<Cow<'static, str>, VertexAttribute>,
    VertexPropertiesHandler: Fn(&mut HashMap<Cow<'static, str>, VertexAttribute>, PropertyMap),
{
    let file = std::fs::File::open(path.as_ref()).map_err(|err| LoadError::new(err.to_string()))?;
    let mut reader = std::io::BufReader::new(file);

    let vertex_parser = parser::Parser::<PropertyMap>::new();
    let face_parser = parser::Parser::<PropertyMap>::new();

    let header = vertex_parser
        .read_header(&mut reader)
        .map_err(|err| LoadError::new(err.to_string()))?;

    let mut vertices_list = vec![];
    let mut faces_list = vec![];

    for (_, element) in &header.elements {
        match element.name.as_ref() {
            "vertex" => {
                vertices_list = vertex_parser
                    .read_payload_for_element(&mut reader, &element, &header)
                    .map_err(|err| LoadError::new(err.to_string()))?
            }
            "face" => {
                faces_list = face_parser
                    .read_payload_for_element(&mut reader, &element, &header)
                    .map_err(|err| LoadError::new(err.to_string()))?
            }
            _ => {}
        }
    }

    let mut vertex_attributes = vertex_attributes_initializer(vertices_list.len());
    for vert in vertices_list {
        vertex_properties_handler(&mut vertex_attributes, vert);
    }

    let mut indices = Vec::with_capacity(faces_list.len() * 3);
    for mut f in faces_list {
        let vertex_index = if f.map.contains_key("vertex_index") {
            f.map.remove("vertex_index".into())
        } else if f.map.contains_key("vertex_indices") {
            f.map.remove("vertex_indices".into())
        } else {
            None
        };
        let vertex_index = if let Some(vertex_index) = vertex_index {
            if let Property::I32List(vertex_index) = vertex_index {
                Some(vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>())
            } else if let Property::U32List(vertex_index) = vertex_index {
                Some(vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>())
            } else if let Property::I16List(vertex_index) = vertex_index {
                Some(vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>())
            } else if let Property::U16List(vertex_index) = vertex_index {
                Some(vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>())
            } else {
                None
            }
        } else {
            None
        };
        if let Some(vertex_index) = vertex_index {
            for i in 1..vertex_index.len() - 1 {
                indices.push(vertex_index[0] as u32);
                indices.push(vertex_index[i] as u32);
                indices.push(vertex_index[i + 1] as u32);
            }
        }
    }

    Ok(TriMesh::new(
        vertex_attributes,
        Some(MeshIndices::U32(indices)),
    ))
}

pub fn save_trimesh<Path, VertexAttributesConverter>(
    path: Path,
    mesh: &TriMesh,
    vertex_attributes_converter: VertexAttributesConverter,
) -> Result<(), SaveError>
where
    Path: AsRef<std::path::Path>,
    VertexAttributesConverter:
        Fn(&HashMap<Cow<'static, str>, VertexAttribute>, usize) -> PropertyMap,
{
    let mut ply = ply::Ply::<ply::DefaultElement>::new();
    ply.header.encoding = ply::Encoding::Ascii;

    let num_vertices = mesh.num_vertices();
    if num_vertices > 0 {
        let mut vertex_element = ply::ElementDef::new("vertex".to_owned());
        let vertex_data = (0..num_vertices)
            .map(|index| vertex_attributes_converter(&mesh.vertex_attributes, index))
            .collect::<Vec<_>>();
        for (k, v) in &vertex_data[0].map {
            let prop = ply::PropertyDef::new(k.to_string(), super::get_property_type(v));
            vertex_element.properties.add(prop);
        }
        ply.header.elements.add(vertex_element);

        let mut vertices = Vec::with_capacity(mesh.num_vertices());
        for data in vertex_data {
            let mut vertex = ply::DefaultElement::new();
            for (k, v) in data.map {
                vertex.insert(k.to_string(), super::get_ply_property(&v));
            }
            vertices.push(vertex);
        }
        ply.payload.insert("vertex".to_owned(), vertices);
    }

    if let Some(num_indcies) = mesh.num_indices() {
        if num_indcies > 0 {
            let mut face_element = ply::ElementDef::new("face".to_owned());
            let prop = ply::PropertyDef::new(
                "vertex_index".to_owned(),
                ply::PropertyType::List(ply::ScalarType::UChar, ply::ScalarType::Int),
            );
            face_element.properties.add(prop);
            ply.header.elements.add(face_element);

            let mut faces = Vec::with_capacity(num_indcies / 3);
            for i in 0..num_indcies / 3 {
                let i3 = i * 3;
                let vertex_index = vec![
                    mesh.indices().unwrap().get(i3) as i32,
                    mesh.indices().unwrap().get(i3 + 1) as i32,
                    mesh.indices().unwrap().get(i3 + 2) as i32,
                ];
                let mut face = ply::DefaultElement::new();
                face.insert(
                    "vertex_index".to_owned(),
                    ply::Property::ListInt(vertex_index),
                );
                faces.push(face);
            }
            ply.payload.insert("face".to_owned(), faces);
        }
    }

    ply.make_consistent()
        .map_err(|err| SaveError::new(err.to_string()))?;

    let mut file =
        std::fs::File::create(path.as_ref()).map_err(|err| SaveError::new(err.to_string()))?;
    let writer = writer::Writer::new();
    writer
        .write_ply(&mut file, &mut ply)
        .map_err(|err| SaveError::new(err.to_string()))?;

    Ok(())
}
