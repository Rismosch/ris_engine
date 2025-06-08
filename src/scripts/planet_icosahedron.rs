use std::f32::consts::PI;
use std::hash::Hash;
use std::usize;

use ris_asset::assets::ris_mesh;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::CpuMesh;
use ris_asset_data::mesh::Indices;
use ris_data::ecs::script_prelude::*;
use ris_error::prelude::*;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_math::color::Rgb;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[derive(Debug)]
pub struct PlanetScript {
    log_mod: usize,
    subdivisions: usize,
    distort: bool,
    rng: Rng,
}

impl Default for PlanetScript {
    fn default() -> Self {
        let seed = ris_error::unwrap!(
           Seed::new(),
           "failed to generate seed",
        );
        let rng = Rng::new(seed);

        Self{
            log_mod: 100_000,
            subdivisions: 9, // 11 is the new record, but it fails to load due to gpu allocation
                              // error
            distort: true,
            rng,
        }
    }
}

struct HashableVertex(Vec3);

impl PartialEq for HashableVertex {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(other.0).all()
    }
}

impl Eq for HashableVertex {}

impl std::hash::Hash for HashableVertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let x = u32::from_ne_bytes(self.0.x().to_ne_bytes());
        let y = u32::from_ne_bytes(self.0.y().to_ne_bytes());
        let z = u32::from_ne_bytes(self.0.x().to_ne_bytes());
        x.hash(state);
        y.hash(state);
        z.hash(state);
    }
}

impl Script for PlanetScript {
    fn start(&mut self, data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        Ok(())
    }

    fn end(&mut self, data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn serialize(&mut self, stream: &mut SceneWriter) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(&mut self, stream: &mut SceneReader) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData {
            id,
            ui,
            game_object,
            frame,
            state,
        } = data;

        let a = 1.0;
        let b = (1.0 + f32::sqrt(5.0)) / 2.0;
        let a = a / 2.0;
        let b = b / 2.0;

        // rectangle 1
        let v0 = Vec3(a, b, 0.0);
        let v1 = Vec3(-a, b, 0.0);
        let v2 = Vec3(-a, -b, 0.0);
        let v3 = Vec3(a, -b, 0.0);

        // rectangle 2
        let v4 = Vec3(b, 0.0, a);
        let v5 = Vec3(-b, 0.0, a);
        let v6 = Vec3(-b, 0.0, -a);
        let v7 = Vec3(b, 0.0, -a);

        // rectangle 3
        let v8 = Vec3(0.0, a, b);
        let v9 = Vec3(0.0, -a, b);
        let v10 = Vec3(0.0, -a, -b);
        let v11 = Vec3(0.0, a, -b);

        ris_debug::gizmo::point(v0, None)?;
        ris_debug::gizmo::point(v1, None)?;
        ris_debug::gizmo::point(v2, None)?;
        ris_debug::gizmo::point(v3, None)?;
        ris_debug::gizmo::point(v4, None)?;
        ris_debug::gizmo::point(v5, None)?;
        ris_debug::gizmo::point(v6, None)?;
        ris_debug::gizmo::point(v7, None)?;
        ris_debug::gizmo::point(v8, None)?;
        ris_debug::gizmo::point(v9, None)?;
        ris_debug::gizmo::point(v10, None)?;
        ris_debug::gizmo::point(v11, None)?;

        if ui.button("generate distorted mesh") {
            ris_log::trace!("generate prototype...");

            // vertices
            let mut vertices = vec![
                v9, v3, v2,
                v9, v4, v3,
                v4, v7, v3,
                v4, v0, v7,
                v0, v11, v7,
                v0, v1, v11,
                v1, v6, v11,
                v1, v5, v6,
                v5, v2, v6,
                v5, v9, v2,
                v8, v4, v9,
                v8, v0, v4, 
                v8, v1, v0,
                v8, v5, v1,
                v8, v9, v5,
                v10, v3, v7,
                v10, v7, v11,
                v10, v11, v6,
                v10, v6, v2,
                v10, v2, v3,
            ];

            // indices
            let mut unique_vertices = Vec::<Vec3>::new();
            let mut indices = Vec::new();
            for (i, v) in vertices.iter().enumerate() {
                if i % self.log_mod == 0 {
                    ris_log::trace!("generate indices... {}/{}", i, vertices.len());
                }

                let position = unique_vertices
                    .iter()
                    .position(|x| x.equal(*v).all());

                match position {
                    Some(position) => {
                        indices.push(position as u32);
                    },
                    None => {
                        let index = unique_vertices.len();
                        indices.push(index.try_into()?);
                        unique_vertices.push(*v);
                    }
                }
            }

            // subdivisions
            ris_log::trace!("generate subdivisions...");
            let mut subdivision = self.subdivisions;
            while subdivision > 0 {
                let actual_subdivision = self.subdivisions - subdivision + 1;
                ris_log::trace!("subdivision {}...", actual_subdivision);

                let old_vertices = &unique_vertices;
                let old_indices = &indices;
                let mut new_vertices = Vec::with_capacity(vertices.len() * 4);
                let mut new_indices = Vec::with_capacity(indices.len() * 4);

                let mut index_lookup = std::collections::HashMap::<HashableVertex, u32>::new();
                
                let mut add_vertex = |v: Vec3| {
                    let key = HashableVertex(v);
                    match index_lookup.get(&key) {
                        Some(index) => new_indices.push(*index),
                        None => {
                            let index = new_vertices.len() as u32;
                            index_lookup.insert(key, index);
                            new_vertices.push(v);
                            new_indices.push(index);
                        },
                    }
                };

                for (round, old_index) in old_indices.chunks(3).enumerate() {
                    if round % self.log_mod == 0 
                    {
                        let actual_round = round * 3;
                        ris_log::trace!(
                            "subdevide... {} {}/{}",
                            actual_subdivision,
                            actual_round,
                            old_indices.len(),
                        );
                    }

                    let i0 = *old_index.get(0).into_ris_error()? as usize;
                    let i1 = *old_index.get(1).into_ris_error()? as usize;
                    let i2 = *old_index.get(2).into_ris_error()? as usize;

                    let v0 = *old_vertices.get(i0).into_ris_error()?;
                    let v1 = *old_vertices.get(i1).into_ris_error()?;
                    let v2 = *old_vertices.get(i2).into_ris_error()?;
                    let v3 = Vec3::mix(v0, v1, Vec3::init(0.5));
                    let v4 = Vec3::mix(v1, v2, Vec3::init(0.5));
                    let v5 = Vec3::mix(v0, v2, Vec3::init(0.5));

                    let v0 = v0.normalize();
                    let v1 = v1.normalize();
                    let v2 = v2.normalize();
                    let v3 = v3.normalize();
                    let v4 = v4.normalize();
                    let v5 = v5.normalize();

                    add_vertex(v0);
                    add_vertex(v3);
                    add_vertex(v5);
                    add_vertex(v3);
                    add_vertex(v1);
                    add_vertex(v4);
                    add_vertex(v5);
                    add_vertex(v3);
                    add_vertex(v4);
                    add_vertex(v5);
                    add_vertex(v4);
                    add_vertex(v2);
                }

                unique_vertices = new_vertices;
                indices = new_indices;

                //let old_vertices = &vertices;
                //let mut new_vertices = Vec::with_capacity(vertices.len() * 4);
                //for old_vertices in old_vertices.chunks(3) {
                //    let v0 = *old_vertices.get(0).into_ris_error()?;
                //    let v1 = *old_vertices.get(1).into_ris_error()?;
                //    let v2 = *old_vertices.get(2).into_ris_error()?;
                //    let v3 = Vec3::mix(v0, v1, Vec3::init(0.5));
                //    let v4 = Vec3::mix(v1, v2, Vec3::init(0.5));
                //    let v5 = Vec3::mix(v0, v2, Vec3::init(0.5));

                //    let v0 = v0.normalize();
                //    let v1 = v1.normalize();
                //    let v2 = v2.normalize();
                //    let v3 = v3.normalize();
                //    let v4 = v4.normalize();
                //    let v5 = v5.normalize();

                //    new_vertices.push(v0);
                //    new_vertices.push(v3);
                //    new_vertices.push(v5);
                //    new_vertices.push(v3);
                //    new_vertices.push(v1);
                //    new_vertices.push(v4);
                //    new_vertices.push(v5);
                //    new_vertices.push(v3);
                //    new_vertices.push(v4);
                //    new_vertices.push(v5);
                //    new_vertices.push(v4);
                //    new_vertices.push(v2);
                //}
                //vertices = new_vertices;

                subdivision -= 1;
            }

            ris_log::debug!("vertices: {} indices: {}", unique_vertices.len(), indices.len());
            //ris_log::debug!("indices: {:#?}", indices);

            // find hull
            //let vertices = self.find_hull_02(vertices);

            // calculate edge distance
            let i0 = indices[0] as usize;
            let i1 = indices[1] as usize;
            let v0 = unique_vertices[i0];
            let v1 = unique_vertices[i1];
            let edge_distance = v0.distance(v1);
            let edge_distance_scaled = edge_distance * 42000.0;
            ris_log::info!("edge distance: {} scaled: {}", edge_distance, edge_distance_scaled);

            ris_log::trace!(
                "vertices: {} unique: {}, indices: {}",
                vertices.len(),
                unique_vertices.len(),
                indices.len(),
            );
            let mut vertices = unique_vertices;

            // distort vertices
            if self.distort {
                ris_log::trace!("find distortion magnitude...");
                let v0 = vertices[0];
                let mut i1 = usize::MAX;
                let mut min = f32::MAX;
                for (i, &v1) in vertices.iter().enumerate() {
                    if v0.equal(v1).all() {
                        continue;
                    }

                    let d = v0.distance(v1);
                    if d < min {
                        min = d;
                        i1 = i;
                    }
                }

                let v1 = vertices[i1];
                let d = v0.distance(v1);
                let distortion_magnitude = d / 3.0;
                ris_log::trace!("distortion magnitude: {}", distortion_magnitude);

                let vertices_len = vertices.len();
                for (i, v) in vertices.iter_mut().enumerate() {
                    if i % self.log_mod == 0 {
                        ris_log::trace!("distort vertices... {}/{}", i, vertices_len);
                    }

                    let mut dir = self.rng.next_dir_3();
                    loop {
                        let dot = v.normalize().dot(dir);
                        if dot.abs() > 0.5 {
                            dir = self.rng.next_dir_3();
                        } else {
                            break;
                        }
                    }
                    let dir = v.normalize().cross(dir);
                    let distorted_vertex = *v + dir * distortion_magnitude;
                    *v = distorted_vertex;
                }
            }


            // normals
            let mut normals = Vec::with_capacity(vertices.len());
            //for vertices in vertices.chunks(3) {
            //    let v0 = vertices[0];
            //    let v1 = vertices[1];
            //    let v2 = vertices[2];

            //    let v01 = v1 - v0;
            //    let v02 = v2 - v0;
            //    let normal = v01.cross(v02);
            //    normals.push(normal);
            //    normals.push(normal);
            //    normals.push(normal);
            //}
            for v in vertices.iter() {
                let n = v.normalize();
                normals.push(n);
            }

            // uvs
            let mut uvs = Vec::with_capacity(vertices.len());
            //for _ in vertices.chunks(3) {
            //    let mut available_uvs = vec![
            //        Vec2(0.0, 0.0),
            //        Vec2(1.0, 0.0),
            //        Vec2(0.0, 1.0),
            //    ];

            //    let i = self.rng.next_i32_between(0, 2) as usize;
            //    let uv0 = available_uvs.swap_remove(i);
            //    let i = self.rng.next_i32_between(0, 1) as usize;
            //    let uv1 = available_uvs.swap_remove(i);
            //    let uv2 = available_uvs[0];

            //    uvs.push(uv0);
            //    uvs.push(uv1);
            //    uvs.push(uv2);
            //}
            for _ in vertices.iter() {
                let uv = self.rng.next_pos_2().abs();

                uvs.push(uv);
            }
            
            // generate mesh
            ris_log::trace!("generate mesh... vertices: {} indices: {}", vertices.len(), indices.len());
            let indices = Indices::U32(indices);
            let prototype = MeshPrototype{
                vertices,
                normals,
                uvs,
                indices,
            };

            let cpu_mesh = CpuMesh::try_from(prototype)?;
            ris_log::trace!("serialize...");
            let bytes = ris_mesh::serialize(&cpu_mesh)?;

            ris_log::trace!("write file...");
            let filename = format!("assets/in_use/meshes/planet_new.ris_mesh");
            if std::fs::exists(&filename)? {
                std::fs::remove_file(&filename)?;
            }

            let mut file = std::fs::File::create_new(filename)?;
            let f = &mut file;
            ris_io::write(f, &bytes)?;
            ris_log::trace!("done!");
        }

        Ok(())
    }
}

fn edges_match(a: (Vec3, Vec3, Vec3), b: (Vec3, Vec3, Vec3)) -> bool {
    let matches_case_1 = a.0.equal(b.0).all() && a.1.equal(a.1).all();
    let matches_case_2 = a.0.equal(b.1).all() && a.1.equal(b.0).all();
    let matches = matches_case_1 || matches_case_2;
    matches
}

impl PlanetScript {
    fn find_hull_02(&mut self, mut vertices: Vec<Vec3>) -> Vec<Vec3> {
        ris_log::trace!("find distortion magnitude...");
        let v0 = vertices[0];
        let mut i1 = usize::MAX;
        let mut min = f32::MAX;
        for (i, &v1) in vertices.iter().enumerate() {
            if v0.equal(v1).all() {
                continue;
            }

            let d = v0.distance(v1);
            if d < min {
                min = d;
                i1 = i;
            }
        }

        let v1 = vertices[i1];
        let d = v0.distance(v1);
        let distortion_magnitude = d / 3.0;
        ris_log::trace!("distortion magnitude: {}", distortion_magnitude);

        let mut rectified = vertices.clone();

        ris_log::trace!("rectify and distort...");
        for i in 0..rectified.len() {
            let left = rectified[i];

            let mut dir = self.rng.next_dir_3();
            loop {
                let dot = left.normalize().dot(dir);
                if dot.abs() > 0.5 {
                    dir = self.rng.next_dir_3();
                } else {
                    break;
                }
            }
            let dir = left.normalize().cross(dir);
            let distorted_vertex = left + dir * distortion_magnitude;

            if i % 100 == 0 {
                ris_log::trace!("rectify... {}/{}", i, rectified.len());
            }

            for (j, right) in vertices.iter_mut().enumerate() {
                if left.fequal(*right, 0.001).all() {
                    rectified[j] = distorted_vertex;
                    *right = Vec3::init(f32::NAN);
                }
            }
        }

        rectified
    }

    fn find_hull_01(&mut self, vertices: Vec<Vec3>) -> Vec<Vec3> {
        // find unique vertices
        ris_log::trace!("find unique vertices...");
        let mut unique_vertices = Vec::<Vec3>::new();
        for (i, &v0) in vertices.iter().enumerate() {

            if i % 5000 == 0 {
                ris_log::trace!("find unique vertices... {}/{}", i, vertices.len());
            }

            let exists = unique_vertices
                .iter()
                .find(|&&x| v0.fequal(x, 0.001).all())
                .is_some();

            if !exists {
                unique_vertices.push(v0);
            }
        }

        ris_log::trace!("unique vertex count: {}", unique_vertices.len());

        // triangulate
        ris_log::trace!("triangulate...");
        let mut vertices = Vec::<Vec3>::new();
        let all_vertices = unique_vertices.clone();

        // find a small triangle
        ris_log::trace!("find small triangle...");
        ris_log::trace!("find v0...");
        let mut v0 = all_vertices[0];
        let mut i1 = usize::MAX;
        let mut i2 = usize::MAX;

        ris_log::trace!("find v1...");
        let mut min = f32::MAX;
        for i in 0..all_vertices.len() {
            let v1 = all_vertices[i];
            if v1.equal(v0).all() {
                continue;
            }

            let d = v0.distance(v1);
            if d < min {
                min = d;
                i1 = i;
            }
        }
        let mut v1 = all_vertices[i1];

        ris_log::trace!("find v2...");
        let mut min = f32::MAX;
        for i in 0..all_vertices.len() {
            let v2 = all_vertices[i];
            if v2.equal(v0).all() || v2.equal(v1).all() {
                continue;
            }

            let d = v2.distance(v0) + v2.distance(v1);
            if d < min {
                min = d;
                i2 = i;
            }
        }
        let v2 = all_vertices[i2];

        let v01 = v1 - v0;
        let v02 = v2 - v0;
        let angle = Vec3::signed_angle(v01, v02, v0);
        if angle.is_sign_negative() {
            ris_log::trace!("swapped {} {:?} {:?} {:?}", angle, v0, v1, v2);
            std::mem::swap(&mut v0, &mut v1);
        }

        ris_log::trace!(
            "smallest triangle found: {:?} {:?} {:?}",
            v0,
            v1,
            v2,
        );
        vertices.push(v0);
        vertices.push(v1);
        vertices.push(v2);

        // the first two define the edge, the third defines the vertex that describes a
        // triangle with that edge
        let mut loose_edges = vec![
            (v0, v1, v2),
            (v1, v2, v0),
            (v2, v0, v1),
        ];

        // connect loose vertices
        let mut count = 0;
        while !loose_edges.is_empty() {
            if count % 100 == 0{
                ris_log::trace!("connecting loose vertices... round: {} edges: {}", count, loose_edges.len());
            }

            count += 1;

            // find third vertex
            // swap v0 and v1, to ensure all other vertices are in front of the plane
            let (v1, v0, mut v2) = loose_edges.remove(0);
            let mut count = 0;
            loop {
                if count % self.log_mod == 0{
                    ris_log::trace!("find better candidate... {}", count);
                }
                count += 1;

                let mut better_candidate_found = false;
                for &v in all_vertices.iter() {
                    let v02 = v0 - v2;
                    let v12 = v1 - v2;
                    let n = v02.cross(v12);

                    let p = v;
                    let o = v2;
                    let d = n.dot(p - o);
                    if d == 0.0 {
                        continue; // indicates a point in the same plane, usually itself
                    }

                    if d.is_sign_negative() {
                        // point is behind the plane. we only want to find candidates in front
                        // of the plane
                        continue;
                    }

                    if v.equal(v0).all() || v.equal(v1).all() {
                        // v2 may not be equal to v0 or v1, or we get an infinitely thin
                        // triangle
                        continue;
                    }

                    //ris_log::trace!("better candidate: {} {} {:?}", d, i, v2);
                    v2 = v;
                    better_candidate_found = true;
                }

                if !better_candidate_found {
                    break;
                }
            }

            // vertex found
            vertices.push(v0);
            vertices.push(v1);
            vertices.push(v2);

            // handle edges
            let edge0 = (v1, v2, v0);
            let edge1 = (v2, v0, v1);

            let edge0_position = loose_edges.iter().position(|&existing_edge| {
                edges_match(edge0, existing_edge)
            });
            match edge0_position {
                Some(i) => _ = loose_edges.remove(i),
                None => loose_edges.push(edge0),
            }

            let edge1_position = loose_edges.iter().position(|&existing_edge| {
                edges_match(edge1, existing_edge)
            });
            match edge1_position {
                Some(i) => _ = loose_edges.remove(i),
                None => loose_edges.push(edge1),
            }
        }

        ris_log::trace!("triangulated vertices! count: {}", vertices.len());

        vertices
    }
}

