use std::collections::HashMap;

use pep_mesh::{
    io::{
        self,
        ply::{Property, PropertyMap},
    },
    trimesh::{TriMesh, VertexAttribute},
};

fn main() {
    let path = "examples/color_cube.ply";
    let mesh = io::ply::load_to_trimesh(
        path,
        |len| {
            let mut vertex_attributes = HashMap::with_capacity(2);
            vertex_attributes.insert(
                TriMesh::POSITION.into(),
                VertexAttribute::float3_with_capacity(len),
            );
            vertex_attributes.insert(
                TriMesh::COLOR.into(),
                VertexAttribute::float3_with_capacity(len),
            );
            vertex_attributes
        },
        |vertex_attributes, props| {
            let x = props.map.get("x").map_or(0.0, |prop| match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            });
            let y = props.map.get("y").map_or(0.0, |prop| match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            });
            let z = props.map.get("z").map_or(0.0, |prop| match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            });
            let position = [x, y, z];
            vertex_attributes
                .get_mut(TriMesh::POSITION)
                .unwrap()
                .push_float3(position);

            let r = props.map.get("red").map_or(0.0, |prop| match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            });
            let g = props.map.get("green").map_or(0.0, |prop| match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            });
            let b = props.map.get("blue").map_or(0.0, |prop| match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            });
            let color = [r, g, b];
            vertex_attributes
                .get_mut(TriMesh::COLOR)
                .unwrap()
                .push_float3(color);
        },
    )
    .expect("Failed to load ply mesh");

    println!("# vertices: {}", mesh.num_vertices());
    println!("# indices: {:?}", mesh.num_indices());

    let path = "color_cube.ply";
    io::ply::save_trimesh(path, &mesh, |vertex_attributes, index| {
        let mut map = PropertyMap::default();

        let position = vertex_attributes
            .get(TriMesh::POSITION)
            .unwrap()
            .get_float3(index)
            .unwrap();
        map.map.insert("x".into(), Property::F32(position[0]));
        map.map.insert("y".into(), Property::F32(position[1]));
        map.map.insert("z".into(), Property::F32(position[2]));

        let color = vertex_attributes
            .get(TriMesh::COLOR)
            .unwrap()
            .get_float3(index)
            .unwrap();
        map.map.insert("red".into(), Property::F32(color[0]));
        map.map.insert("green".into(), Property::F32(color[1]));
        map.map.insert("blue".into(), Property::F32(color[2]));

        map
    })
    .expect("Failed to save ply mesh");
}
