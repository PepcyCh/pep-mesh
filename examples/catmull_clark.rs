use std::collections::HashMap;

use cgmath::{ElementWise, EuclideanSpace, Point3};
use pep_mesh::{halfedge::{HalfEdgeMesh, VertexRef}, io::{self, ply::{Property, PropertyMap, ToPropertyMap}}};

struct VData {
    pos: Point3<f32>,
    new_pos: Option<Point3<f32>>,
}

impl Default for VData {
    fn default() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            new_pos: None,
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
        let pos = Point3::new(x, y, z);

        Self {
            pos,
            ..Default::default()
        }
    }
}

impl ToPropertyMap for VData {
    fn to_proprety_map(&self) -> PropertyMap {
        let mut map = HashMap::new();

        map.insert("x".into(), Property::F32(self.pos.x));
        map.insert("y".into(), Property::F32(self.pos.y));
        map.insert("z".into(), Property::F32(self.pos.z));

        PropertyMap { map }
    }
}

#[derive(Default)]
struct EData {
    new_pos: Option<Point3<f32>>,
    new_vert: Option<VertexRef>,
}

impl From<PropertyMap> for EData {
    fn from(_: PropertyMap) -> Self {
        Self::default()
    }
}

impl ToPropertyMap for EData {
    fn to_proprety_map(&self) -> PropertyMap {
        PropertyMap::default()
    }
}

#[derive(Default)]
struct FData {
    new_pos: Option<Point3<f32>>,
}

impl From<PropertyMap> for FData {
    fn from(_: PropertyMap) -> Self {
        Self::default()
    }
}

impl ToPropertyMap for FData {
    fn to_proprety_map(&self) -> PropertyMap {
        PropertyMap::default()
    }
}

fn catmull_clark(mesh: &mut HalfEdgeMesh<VData, EData, FData>, iter_times: u32) {
    for _ in 0..iter_times {
        // calc position of face point
        for face in mesh.faces() {
            if face.is_boundary(mesh) {
                continue;
            }
            let mut count = 0.0;
            let mut sum = Point3::new(0.0, 0.0, 0.0);
            let mut he = face.halfedge(mesh);
            loop {
                count += 1.0;
                sum.add_assign_element_wise(he.vertex(mesh).data(mesh).pos);
                he = he.next(mesh);
                if he == face.halfedge(mesh) {
                    break;
                }
            }
            face.data_mut(mesh).new_pos = Some(sum / count);
        }
        // calc position of edge pointoint
        for face in mesh.faces() {
            if face.is_boundary(mesh) {
                continue;
            }
            let mut he = face.halfedge(mesh);
            loop {
                if he.data(mesh).new_pos.is_none() {
                    let twin = he.twin(mesh);

                    let pos_v1 = he.vertex(mesh).data(mesh).pos;
                    let pos_v2 = twin.vertex(mesh).data(mesh).pos;

                    if he.on_boundary(mesh) || twin.on_boundary(mesh) {
                        he.data_mut(mesh).new_pos = Some(pos_v1.midpoint(pos_v2));
                    } else {
                        let pos_f1 = he.face(mesh).data(mesh).new_pos;
                        let pos_f2 = twin.face(mesh).data(mesh).new_pos;
                        if pos_f1.is_some() && pos_f2.is_some() {
                            let pos_f1 = pos_f1.unwrap();
                            let pos_f2 = pos_f2.unwrap();
                            he.data_mut(mesh).new_pos = Some(pos_v1.add_element_wise(pos_v2).add_element_wise(pos_f1).add_element_wise(pos_f2) / 4.0);
                        } else {
                            he.data_mut(mesh).new_pos = Some(pos_v1.midpoint(pos_v2));
                        }
                    }
                }
                he = he.next(mesh);
                if he == face.halfedge(mesh) {
                    break;
                }
            }
        }
        // calc position of vertex pointoint
        for face in mesh.faces() {
            if face.is_boundary(mesh) {
                continue;
            }
            let mut he = face.halfedge(mesh);
            loop {
                let v = he.vertex(mesh);
                if v.data(mesh).new_pos.is_none() {
                    let pos_v = v.data(mesh).pos;
                    if v.on_boundary(mesh) {
                        let mut he = v.halfedge(mesh);
                        loop {
                            he = he.twin(mesh);
                            if he.on_boundary(mesh) {
                                break;
                            }
                            he = he.next(mesh);
                        }
                        let pos_e1 = he.vertex(mesh).data(mesh).pos;
                        let pos_e2 = he.next(mesh).next(mesh).vertex(mesh).data(mesh).pos;
                        v.data_mut(mesh).new_pos = Some((0.75 * pos_v).add_element_wise(0.125 * pos_e1).add_element_wise(0.125 * pos_e2));
                    } else {
                        let mut count = 0.0;
                        let mut sum = Point3::new(0.0, 0.0, 0.0);
                        let mut vhe = he;
                        loop {
                            let twin = vhe.twin(mesh);
                            count += 1.0;
                            sum.add_assign_element_wise(twin.vertex(mesh).data(mesh).pos);
                            if let Some(pos_f) = vhe.face(mesh).data(mesh).new_pos {
                                sum.add_assign_element_wise(pos_f);
                            } else {
                                sum.add_assign_element_wise(pos_v);
                            }
                            vhe = twin.next(mesh);
                            if vhe == he {
                                break;
                            }
                        }
                        v.data_mut(mesh).new_pos = Some(((count - 2.0) * pos_v).add_element_wise(sum / count) / count);
                    }
                }
                he = he.next(mesh);
                if he == face.halfedge(mesh) {
                    break;
                }
            }
        }
        // split edges
        let mut halfedges = vec![];
        for face in mesh.faces() {
            if face.is_boundary(mesh) {
                continue;
            }
            let mut he = face.halfedge(mesh);
            loop {
                if he.data(mesh).new_vert.is_none() {
                    he.data_mut(mesh).new_vert = Some(mesh.create_vertex(VData { pos: he.data(mesh).new_pos.unwrap(), new_pos: None }));
                    halfedges.push(he);
                }
                he = he.next(mesh);
                if he == face.halfedge(mesh) {
                    break;
                }
            }
        }
        for he in halfedges {
            let ev = he.data(mesh).new_vert.unwrap();
            *he.data_mut(mesh) = EData::default();
            let new_edge = mesh.create_edge(&he.vertex(mesh), &ev, EData::default());
            let twin = he.twin(mesh);
            new_edge.0.set_next(mesh, &he);
            he.last(mesh).set_next(mesh, &new_edge.0);
            he.vertex(mesh).set_halfedge(mesh, &new_edge.0);
            he.set_vertex(mesh, &ev);
            new_edge.0.set_face(mesh, &he.face(mesh));
            he.face(mesh).set_halfedge(mesh, &new_edge.0);
            new_edge.1.set_next(mesh, &twin.next(mesh));
            twin.set_next(mesh, &new_edge.1);
            new_edge.1.set_face(mesh, &twin.face(mesh));
            twin.face(mesh).set_halfedge(mesh, &twin);
            ev.set_halfedge(mesh, &he);
        }
        // update topology and process_faces
        for face in mesh.faces() {
            if face.is_boundary(mesh) {
                continue;
            }
            let fv = mesh.create_vertex(VData { pos: face.data(mesh).new_pos.unwrap(), new_pos: None });
            *face.data_mut(mesh) = FData::default();
            let mut new_edges = vec![];
            let mut new_faces = vec![];
            let mut count = 0;

            let mut he = face.halfedge(mesh);
            loop {
                let he_last = he;
                he = he.next(mesh);
                let ev = he.vertex(mesh);
                let he_next = he;

                let new_edge = mesh.create_edge(&ev, &fv, EData::default());
                fv.set_halfedge(mesh, &new_edge.1);
                new_edge.1.set_next(mesh, &he_next);
                he_last.set_next(mesh, &new_edge.0);

                new_edges.push(new_edge.0);
                new_edges.push(new_edge.1);

                if count == 0 {
                    new_faces.push(face);
                } else {
                    let new_face = mesh.create_face(FData::default(), face.is_boundary(mesh));
                    new_faces.push(new_face);
                }
                count += 1;

                let vert = he_last.vertex(mesh);
                if let Some(v_pos) = vert.data(mesh).new_pos {
                    vert.data_mut(mesh).pos = v_pos;
                    vert.data_mut(mesh).new_pos = None;
                }

                he = he.next(mesh);
                if he == face.halfedge(mesh) {
                    break;
                }
            }

            for i in 0..count {
                let j = if i == 0 {
                    2 * count - 1
                } else {
                    2 * i - 1
                };
                let edge_j = new_edges[j];
                new_edges[2 * i].set_next(mesh, &edge_j);
                new_faces[i].set_halfedge(mesh, &edge_j);
                let mut he = new_edges[2 * i];
                loop {
                    he.set_face(mesh, &new_faces[i]);
                    he = he.next(mesh);
                    if he == new_edges[2 * i] {
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    // let path = "examples/cube.ply";
    let path = "examples/boundary.ply";
    let mut mesh: HalfEdgeMesh<VData, EData, FData> = io::ply::load_to_halfedge(path).expect("Failed to load ply mesh");
    catmull_clark(&mut mesh, 4);

    let path = "subdivided.ply";
    io::ply::save_halfedge(path, &mesh).expect("Failed to save ply mesh");
}