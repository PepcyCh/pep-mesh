use std::collections::HashMap;

use cgmath::Point3;
use pep_mesh::{halfedge::HalfEdgeMesh, io::{self, ply::{Property, PropertyMap, ToPropertyMap}}};

struct VData {
    position: Point3<f32>,
    color: [f32; 3],
}

impl Default for VData {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            color: [0.0, 0.0, 0.0],
        }
    }
}

impl From<PropertyMap> for VData {
    fn from(props: PropertyMap) -> Self {
        let x = props.map.get("x").map_or(0.0, |prop| {
            match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            }
        });
        let y = props.map.get("y").map_or(0.0, |prop| {
            match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            }
        });
        let z = props.map.get("z").map_or(0.0, |prop| {
            match prop {
                Property::F32(val) => *val,
                Property::F64(val) => *val as f32,
                _ => 0.0,
            }
        });
        let position = Point3::new(x, y, z);

        let r = props.map.get("red").map_or(0.0, |prop| {
            match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            }
        });
        let g = props.map.get("green").map_or(0.0, |prop| {
            match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            }
        });
        let b = props.map.get("blue").map_or(0.0, |prop| {
            match prop {
                Property::U8(val) => *val as f32 / 255.0,
                _ => 0.0,
            }
        });
        let color = [r, g, b];

        Self {
            position,
            color,
        }
    }
}

impl ToPropertyMap for VData {
    fn to_proprety_map(&self) -> PropertyMap {
        let mut map = HashMap::new();

        map.insert("x".into(), Property::F32(self.position.x));
        map.insert("y".into(), Property::F32(self.position.y));
        map.insert("z".into(), Property::F32(self.position.z));

        map.insert("red".into(), Property::F32(self.color[0]));
        map.insert("green".into(), Property::F32(self.color[1]));
        map.insert("blue".into(), Property::F32(self.color[2]));

        PropertyMap { map }
    }
}

fn main() {
    let path = "examples/io_test_input.ply";
    let mesh: HalfEdgeMesh<VData, (), ()> = io::ply::load_to_halfedge(path).expect("Failed to load ply mesh");

    println!("# vertices: {}", mesh.num_vertices());
    println!("# edges: {}", mesh.num_edges());
    println!("# faces (w/ boundary faces): {}", mesh.num_faces_with_boundary());
    println!("# faces (w/o boundary faces): {}", mesh.num_faces());

    let path = "io_test_output.ply";
    io::ply::save_halfedge(path, &mesh).expect("Failed to save ply mesh");
}