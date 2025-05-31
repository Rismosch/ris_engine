use std::usize;

use ris_asset::assets::ris_mesh;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::CpuMesh;
use ris_data::ecs::script_prelude::*;
use ris_error::prelude::*;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[derive(Debug)]
pub struct PlanetScript {
    subdivisions: usize,
    noise_magnitude: f32,
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
            subdivisions: 5,
            noise_magnitude: 0.01,
            rng,
        }
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

        if ui.button("generate") {
            ris_log::trace!("generate prototype...");

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

            // subdivisions
            ris_log::trace!("generate subdivisions...");
            let mut subdivision = self.subdivisions;
            while subdivision > 0 {
                ris_log::trace!("subdivision {}...", subdivision);

                // vertices
                let old_vertices = &vertices;
                let mut new_vertices = Vec::with_capacity(vertices.len() * 4);
                for old_vertices in old_vertices.chunks(3) {
                    let v0 = *old_vertices.get(0).into_ris_error()?;
                    let v1 = *old_vertices.get(1).into_ris_error()?;
                    let v2 = *old_vertices.get(2).into_ris_error()?;
                    let v3 = Vec3::mix(v0, v1, Vec3::init(0.5));
                    let v4 = Vec3::mix(v1, v2, Vec3::init(0.5));
                    let v5 = Vec3::mix(v0, v2, Vec3::init(0.5));

                    let v0 = v0.normalize();
                    let v1 = v1.normalize();
                    let v2 = v2.normalize();
                    let v3 = v3.normalize();
                    let v4 = v4.normalize();
                    let v5 = v5.normalize();

                    new_vertices.push(v0);
                    new_vertices.push(v3);
                    new_vertices.push(v5);
                    new_vertices.push(v3);
                    new_vertices.push(v1);
                    new_vertices.push(v4);
                    new_vertices.push(v5);
                    new_vertices.push(v3);
                    new_vertices.push(v4);
                    new_vertices.push(v5);
                    new_vertices.push(v4);
                    new_vertices.push(v2);
                }
                vertices = new_vertices;

                subdivision -= 1;
            }

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

            ris_log::trace!("distort vertices...");
            let mut distorted_vertices = unique_vertices.clone();
            for v in distorted_vertices.iter_mut() {
                let dir = self.rng.next_dir_3();
                let distortion = dir * self.noise_magnitude;
                let distorted = (*v + distortion).normalize();
                *v = distorted;
            }

            ris_log::trace!("triangulate...");
            let mut vertices = Vec::<Vec3>::new();
            let mut loose_vertices = distorted_vertices.clone();

            ris_log::trace!("find first triangle...");
            ris_log::trace!("find first vertex...");
            let v0 = loose_vertices.swap_remove(0);

            ris_log::trace!("find second vertex...");
            let mut i1 = 0;
            for (i, &v1b) in loose_vertices.iter().enumerate() {
                let v1a = loose_vertices[i1];
                let distance_a = v0.distance(v1a);
                let distance_b = v0.distance(v1b);

                if distance_b < distance_a {
                    i1 = i;
                }
            }
            let v1 = loose_vertices.swap_remove(i1);

            ris_log::trace!("find third vertex...");
            let mut i2 = 0;
            for (i, &v2b) in loose_vertices.iter().enumerate() {
                let v2a = loose_vertices[i2];
                let distance_a = Vec3::distance_to_point(v0, v1, v2a);
                let distance_b = Vec3::distance_to_point(v0, v1, v2b);

                if distance_b < distance_a {
                    i2 = i;
                }
            }
            let v2 = loose_vertices.swap_remove(i2);

            vertices.push(v0);
            vertices.push(v1);
            vertices.push(v2);
            ris_log::trace!("found triangle: {:?} {:?} {:?}", v0, v1, v2);

            let mut loose_edges = vec![
                (v0, v1),
                (v1, v2),
                (v2, v0),
            ];

            ris_log::trace!("connecting loose vertices...");
            while !loose_vertices.is_empty() {
                if loose_vertices.len() % 100 == 0{
                    ris_log::trace!("connecting loose vertices... {}", loose_vertices.len());
                }

                // find third vertex
                let (v0, v1) = loose_edges.swap_remove(0);
                let mut i2 = 0;
                for (i, &v2b) in loose_vertices.iter().enumerate() {
                    let v2a = loose_vertices[i2];

                    let distance_a = Vec3::distance_to_point(v0, v1, v2a);
                    let distance_b = Vec3::distance_to_point(v0, v1, v2b);

                    if distance_b < distance_a {
                        i2 = i;
                    }
                }
                let v2 = loose_vertices.swap_remove(i2);

                // vertex found
                vertices.push(v0);
                vertices.push(v1);
                vertices.push(v2);

                // handle edges
                let edge0 = (v2, v0);
                let edge1 = (v2, v1);

                let edge0_position = loose_edges.iter().position(|&existing_edge| {
                    edges_match(edge0, existing_edge)
                });
                let edge1_position = loose_edges.iter().position(|&existing_edge| {
                    edges_match(edge1, existing_edge)
                });

                match edge0_position {
                    Some(i) => _ = loose_edges.swap_remove(i),
                    None => loose_edges.push(edge0),
                }
                match edge1_position {
                    Some(i) => _ = loose_edges.swap_remove(i),
                    None => loose_edges.push(edge1),
                }
            }

            ris_log::trace!("triangulated vertices! count: {}", vertices.len());
            ris_log::trace!("loose count: {} {}", loose_vertices.len(), loose_edges.len());

            //ris_log::trace!("generate mesh... vertices: {}", vertices.len());
            //let prototype = MeshPrototype{
            //    vertices,
            //    normals,
            //    uvs,
            //    indices,
            //};

            //let cpu_mesh = CpuMesh::try_from(prototype)?;
            //ris_log::trace!("serialize...");
            //let bytes = ris_mesh::serialize(&cpu_mesh)?;

            //ris_log::trace!("write file...");
            //let filename = format!("assets/in_use/meshes/planet_convex_hull.ris_mesh");
            //if std::fs::exists(&filename)? {
            //    std::fs::remove_file(&filename)?;
            //}

            //let mut file = std::fs::File::create_new(filename)?;
            //let f = &mut file;
            //ris_io::write(f, &bytes)?;
            //ris_log::trace!("done!");
        }

        Ok(())
    }
}

fn edges_match(a: (Vec3, Vec3), b: (Vec3, Vec3)) -> bool {
    let matches_case_1 = a.0.equal(b.0).all() && a.1.equal(a.1).all();
    let matches_case_2 = a.0.equal(b.1).all() && a.1.equal(b.0).all();
    let matches = matches_case_1 || matches_case_2;
    matches
}

impl PlanetScript {
    fn distort_vertex(&mut self, v: Vec3) -> Vec3 {
        let dir = self.rng.next_dir_3();
        let distortion = dir * self.noise_magnitude;
        (v + distortion).normalize()
    }
}
