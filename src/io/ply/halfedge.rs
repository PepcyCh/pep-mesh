use std::collections::HashMap;

use ply_rs::{
    parser,
    ply::{self, Addable},
    writer,
};

use crate::{
    halfedge::HalfEdgeMesh,
    io::{
        ply::{Property, PropertyMap},
        LoadError, SaveError,
    },
};

pub trait FromPropertyMap {
    fn from_proprety_map(props: PropertyMap) -> Self;
}

impl<T: From<PropertyMap>> FromPropertyMap for T {
    fn from_proprety_map(props: PropertyMap) -> Self {
        props.into()
    }
}

impl FromPropertyMap for () {
    fn from_proprety_map(_: PropertyMap) -> Self {
        ()
    }
}

pub trait ToPropertyMap {
    fn to_proprety_map(&self) -> PropertyMap;
}

impl ToPropertyMap for () {
    fn to_proprety_map(&self) -> PropertyMap {
        PropertyMap::default()
    }
}

pub fn load_to_halfedge<Path, VData, EData, FData>(
    path: Path,
) -> Result<HalfEdgeMesh<VData, EData, FData>, LoadError>
where
    Path: AsRef<std::path::Path>,
    VData: Default + FromPropertyMap,
    EData: Default + FromPropertyMap,
    FData: Default + FromPropertyMap,
{
    let file = std::fs::File::open(path.as_ref()).map_err(|err| LoadError::new(err.to_string()))?;
    let mut reader = std::io::BufReader::new(file);

    let vertex_parser = parser::Parser::<PropertyMap>::new();
    let face_parser = parser::Parser::<PropertyMap>::new();
    let edge_parser = parser::Parser::<PropertyMap>::new();

    let header = vertex_parser
        .read_header(&mut reader)
        .map_err(|err| LoadError::new(err.to_string()))?;

    let mut vertices_list = vec![];
    let mut faces_list = vec![];
    let mut edges_list = vec![];

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
            "edge" => {
                edges_list = edge_parser
                    .read_payload_for_element(&mut reader, &element, &header)
                    .map_err(|err| LoadError::new(err.to_string()))?
            }
            _ => {}
        }
    }

    let mut faces = Vec::with_capacity(faces_list.len());
    let mut faces_data = HashMap::with_capacity(faces_list.len());
    for mut f in faces_list {
        let vertex_index = if f.map.contains_key("vertex_index") {
            f.map.remove("vertex_index".into())
        } else if f.map.contains_key("vertex_indices") {
            f.map.remove("vertex_indices".into())
        } else {
            None
        };
        if let Some(vertex_index) = vertex_index {
            if let Property::I32List(vertex_index) = vertex_index {
                let vertex_index = vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>();
                faces_data.insert(faces.len(), FData::from_proprety_map(f));
                faces.push(vertex_index);
            } else if let Property::U32List(vertex_index) = vertex_index {
                let vertex_index = vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>();
                faces_data.insert(faces.len(), FData::from_proprety_map(f));
                faces.push(vertex_index);
            } else if let Property::I16List(vertex_index) = vertex_index {
                let vertex_index = vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>();
                faces_data.insert(faces.len(), FData::from_proprety_map(f));
                faces.push(vertex_index);
            } else if let Property::U16List(vertex_index) = vertex_index {
                let vertex_index = vertex_index.iter().map(|i| *i as usize).collect::<Vec<_>>();
                faces_data.insert(faces.len(), FData::from_proprety_map(f));
                faces.push(vertex_index);
            }
        }
    }

    let vertices_data = vertices_list
        .into_iter()
        .enumerate()
        .map(|(vid, prop)| (vid, VData::from_proprety_map(prop)))
        .collect::<HashMap<_, _>>();

    let mut edges_data = HashMap::with_capacity(edges_list.len());
    for mut e in edges_list {
        if let Some(vertex1) = e.map.remove("vertex1".into()) {
            if let Some(vertex2) = e.map.remove("vertex2".into()) {
                let pair = match (vertex1, vertex2) {
                    (Property::I32(v1), Property::I32(v2)) => Some((v1 as usize, v2 as usize)),
                    (Property::U32(v1), Property::U32(v2)) => Some((v1 as usize, v2 as usize)),
                    (Property::I16(v1), Property::I16(v2)) => Some((v1 as usize, v2 as usize)),
                    (Property::U16(v1), Property::U16(v2)) => Some((v1 as usize, v2 as usize)),
                    _ => None,
                };
                if let Some((v1, v2)) = pair {
                    let key = (v1.min(v2), v1.max(v2));
                    edges_data.insert(key, EData::from_proprety_map(e));
                }
            }
        }
    }

    Ok(HalfEdgeMesh::new(
        faces,
        vertices_data,
        edges_data,
        faces_data,
    ))
}

pub fn save_halfedge<Path, VData, EData, FData>(
    path: Path,
    mesh: &HalfEdgeMesh<VData, EData, FData>,
) -> Result<(), SaveError>
where
    Path: AsRef<std::path::Path>,
    VData: Default + ToPropertyMap,
    EData: Default + ToPropertyMap,
    FData: Default + ToPropertyMap,
{
    let mut ply = ply::Ply::<ply::DefaultElement>::new();
    ply.header.encoding = ply::Encoding::Ascii;

    if mesh.num_vertices() > 0 {
        let mut vertex_element = ply::ElementDef::new("vertex".to_owned());
        let vdata = mesh.vertices().next().unwrap().data(mesh).to_proprety_map();
        for (k, v) in &vdata.map {
            let prop = ply::PropertyDef::new(k.to_string(), super::get_property_type(v));
            vertex_element.properties.add(prop);
        }
        ply.header.elements.add(vertex_element);

        let mut vertices = Vec::with_capacity(mesh.num_vertices());
        for vref in mesh.vertices() {
            let mut vertex = ply::DefaultElement::new();
            let vdata = mesh.vertex_data(&vref).to_proprety_map();
            for (k, v) in &vdata.map {
                vertex.insert(k.to_string(), super::get_ply_property(v));
            }
            vertices.push(vertex);
        }
        ply.payload.insert("vertex".to_owned(), vertices);
    }

    let num_faces = mesh.num_faces();
    if num_faces > 0 {
        let mut face_element = ply::ElementDef::new("face".to_owned());
        let prop = ply::PropertyDef::new(
            "vertex_index".to_owned(),
            ply::PropertyType::List(ply::ScalarType::UChar, ply::ScalarType::Int),
        );
        face_element.properties.add(prop);
        let fdata = mesh.faces().next().unwrap().data(mesh).to_proprety_map();
        for (k, v) in &fdata.map {
            let prop = ply::PropertyDef::new(k.to_string(), super::get_property_type(v));
            face_element.properties.add(prop);
        }
        ply.header.elements.add(face_element);

        let mut faces = Vec::with_capacity(num_faces);
        for fref in mesh.faces() {
            if fref.is_boundary(mesh) {
                continue;
            }

            let mut he = fref.halfedge(mesh);
            let mut vertex_index = vec![];
            loop {
                vertex_index.push(he.vertex(mesh).id as i32);
                he = he.next(mesh);
                if he == fref.halfedge(mesh) {
                    break;
                }
            }

            let mut face = ply::DefaultElement::new();
            face.insert(
                "vertex_index".to_owned(),
                ply::Property::ListInt(vertex_index),
            );
            let fdata = mesh.face_data(&fref).to_proprety_map();
            for (k, v) in &fdata.map {
                face.insert(k.to_string(), super::get_ply_property(v));
            }
            faces.push(face);
        }
        ply.payload.insert("face".to_owned(), faces);
    }

    if mesh.num_edges() > 0 {
        let mut edge_element = ply::ElementDef::new("edge".to_owned());
        let prop = ply::PropertyDef::new(
            "vertex1".to_owned(),
            ply::PropertyType::Scalar(ply::ScalarType::Int),
        );
        edge_element.properties.add(prop);
        let prop = ply::PropertyDef::new(
            "vertex2".to_owned(),
            ply::PropertyType::Scalar(ply::ScalarType::Int),
        );
        edge_element.properties.add(prop);
        let edata = mesh
            .halfedges()
            .next()
            .unwrap()
            .data(mesh)
            .to_proprety_map();
        for (k, v) in &edata.map {
            let prop = ply::PropertyDef::new(k.to_string(), super::get_property_type(v));
            edge_element.properties.add(prop);
        }
        ply.header.elements.add(edge_element);

        let mut edges = Vec::with_capacity(mesh.num_edges());
        for heref in mesh.halfedges() {
            let twin = heref.twin(mesh);
            let v1 = mesh.halfedges[heref.id].vertex;
            let v2 = mesh.halfedges[twin.id].vertex;
            if v1 > v2 {
                continue;
            }

            let mut edge = ply::DefaultElement::new();
            edge.insert("vertex1".to_owned(), ply::Property::Int(v1 as i32));
            edge.insert("vertex2".to_owned(), ply::Property::Int(v2 as i32));
            let edata = mesh.edge_data(&heref).to_proprety_map();
            for (k, v) in &edata.map {
                edge.insert(k.to_string(), super::get_ply_property(v));
            }
            edges.push(edge);
        }
        ply.payload.insert("edge".to_owned(), edges);
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
