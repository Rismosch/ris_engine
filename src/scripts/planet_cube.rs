use std::f32::consts::PI;

use std::path::PathBuf;

use ris_asset::assets::ris_mesh;
use ris_asset::assets::ris_terrain;
use ris_asset::codecs::qoi;
use ris_asset::codecs::qoi::Channels;
use ris_asset::codecs::qoi::ColorSpace;
use ris_asset::codecs::qoi::QoiDesc;
use ris_asset_data::mesh::CpuMesh;
use ris_asset_data::mesh::Indices;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::terrain_mesh::TerrainCpuMesh;
use ris_asset_data::terrain_mesh::TerrainMeshPrototype;
use ris_asset_data::terrain_mesh::TerrainVertex;
use ris_data::ecs::components::mesh_component::MeshComponent;
use ris_data::ecs::script_prelude::*;
use ris_math::color::ByteColor3;
use ris_math::color::Rgb;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
/*

notes

radius r = 42000
area = 2 * pi * r * r
umfang u = 2 * pi * r

sin alpha = (x / 2) / r
2 * r * sin alpha = x / (2 * r)

side_count s
resolution length l
l = (u / 4) / s
s = (u / 4) / l
s = ((2 * pi * r) / 4) / l

s = ((2 * pi * 42000) / 4) / 0.5
s ~= 131947 // resolution is acceptable, in a sense that 0..2_000_000 can be mapped in f32 between
            // 0.0..1.0 with no loss in precision. i.e. when i is between 0.0..1.0, we can get
            // quantized values by doing `(i * 2_000_000) as i32`;

vertices = s^3 ~= 2297193026690053

on my lenovo arch machine, generating 20000^2 hashes took 9.960982436s, (no mesh generation and upload to the gpu taken into account). this means on full resolution, with 0.5 meters between vertices, a 10km^2 can be generated in 10 seconds. This asumes no optimization, and every possible optimization can reduce that number.

assume a horizon distance of 10km when 2km high, what is the radius of the planet?

let distance = 10000
let height = 2000

distance / r = tan alpha
distance / (height + r) = cos alpha

alpha = atan distance / r
alpha = acos distance / (height + r)
atan distance / r = acos distance / (height + r)

shortest distance between points on different sides of the cube:
https://www.desmos.com/calculator/u5prhqrp1n

version 2:
https://www.desmos.com/calculator/ag21xuhioj

version 3:
https://www.desmos.com/calculator/kbkqho1ivt

versoin 4:
https://www.desmos.com/calculator/zgdphnszsp

----

(r + 2000) / r = sin alpha
alpha = asin (2000 + r) / r
distance / (2000 + r) = cos alpha
distance / (2000 + r) = cos asin (2000 + r) / r
distance = (2000 + r) * cos asin (2000 + r) / r
distance = (2000 + r) * cos(asin((2000 + r) / r))

*/

#[derive(Debug)]
pub struct PlanetScript {
    rng: Rng,
    subdivisions: usize,
    magnitude: f32,
}

impl Default for PlanetScript {
    fn default() -> Self {
        let seed = ris_error::unwrap!(Seed::new(), "failed to generate seed",);
        let rng = Rng::new(seed);

        Self {
            rng,
            subdivisions: 5,
            magnitude: 0.05,
        }
    }
}

impl Script for PlanetScript {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData {
            ui,
            game_object,
            state,
            ..
        } = data;

        if ui.button("benchmark") {
            let start = std::time::Instant::now();

            let view_distance = 13000;
            let count = std::hint::black_box(view_distance * view_distance);
            let mut actual_count = 0;
            for i in 0..count {
                // assuming chunkification. one chunkification reduces the vertices by 4
                let chunk_steps = 2;
                let modulo = chunk_steps * 4;
                if i % modulo != 0 {
                    continue;
                }

                for j in 0..count {
                    let i = std::hint::black_box(i);
                    let j = std::hint::black_box(j);
                    let x = i as f32 / count as f32;
                    let y = j as f32 / count as f32;
                    let p = Vec3(x, y, 0.0);
                    let hash = xxhash_vec3(p, 42);
                    std::hint::black_box(hash);
                    actual_count += 1;
                }
            }

            let end = std::time::Instant::now();
            let diff = end - start;
            ris_log::debug!(
                "generating {}^2 (actual {} ~= {}^2) hashes took {:?}",
                count,
                actual_count,
                (actual_count as f32).sqrt(),
                diff,
            );
        } // benchmark

        let v0 = Vec3(1.0, 1.0, 1.0);
        let v1 = Vec3(-1.0, 1.0, 1.0);
        let v2 = Vec3(1.0, -1.0, 1.0);
        let v3 = Vec3(-1.0, -1.0, 1.0);
        let v4 = Vec3(1.0, 1.0, -1.0);
        let v5 = Vec3(-1.0, 1.0, -1.0);
        let v6 = Vec3(1.0, -1.0, -1.0);
        let v7 = Vec3(-1.0, -1.0, -1.0);

        ris_debug::gizmo::point(v0, None)?;
        ris_debug::gizmo::point(v1, None)?;
        ris_debug::gizmo::point(v2, None)?;
        ris_debug::gizmo::point(v3, None)?;
        ris_debug::gizmo::point(v4, None)?;
        ris_debug::gizmo::point(v5, None)?;
        ris_debug::gizmo::point(v6, None)?;
        ris_debug::gizmo::point(v7, None)?;

        if ui.button("generate mesh") {
            let start = std::time::Instant::now();
            ris_log::trace!("generate prototype...");

            // vertices
            let mut vertices = vec![
                v0, v4, v2, v6, v2, v4, v0, v1, v4, v5, v4, v1, v0, v2, v1, v3, v1, v2, v7, v5, v3,
                v1, v3, v5, v7, v3, v6, v2, v6, v3, v7, v6, v5, v4, v5, v6,
            ];

            for subdivision in 0..self.subdivisions {
                ris_log::trace!("subdevide... {}/{}", subdivision, self.subdivisions);

                let mut new_vertices = Vec::with_capacity(vertices.len() * 4);
                for vertices in vertices.chunks(6) {
                    let v0 = vertices[0];
                    let v1 = vertices[1];
                    let v2 = vertices[2];
                    let v3 = vertices[3];
                    let v4 = (v0 + v1) / 2.0;
                    let v5 = (v1 + v3) / 2.0;
                    let v6 = (v3 + v2) / 2.0;
                    let v7 = (v2 + v0) / 2.0;
                    let v8 = (v0 + v3) / 2.0;

                    new_vertices.push(v0);
                    new_vertices.push(v4);
                    new_vertices.push(v7);
                    new_vertices.push(v8);
                    new_vertices.push(v7);
                    new_vertices.push(v4);

                    new_vertices.push(v4);
                    new_vertices.push(v1);
                    new_vertices.push(v8);
                    new_vertices.push(v5);
                    new_vertices.push(v8);
                    new_vertices.push(v1);

                    new_vertices.push(v8);
                    new_vertices.push(v5);
                    new_vertices.push(v6);
                    new_vertices.push(v3);
                    new_vertices.push(v6);
                    new_vertices.push(v5);

                    new_vertices.push(v7);
                    new_vertices.push(v8);
                    new_vertices.push(v2);
                    new_vertices.push(v6);
                    new_vertices.push(v2);
                    new_vertices.push(v8);
                }

                vertices = new_vertices;
            }

            // normalize vertices
            // https://catlikecoding.com/unity/tutorials/cube-sphere/
            for v in vertices.iter_mut() {
                let Vec3(x, y, z) = *v;
                let x2 = x * x;
                let y2 = y * y;
                let z2 = z * z;
                let sx = x * f32::sqrt(1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0);
                let sy = y * f32::sqrt(1.0 - x2 / 2.0 - z2 / 2.0 + x2 * z2 / 3.0);
                let sz = z * f32::sqrt(1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0);
                *v = Vec3(sx, sy, sz)
            }

            // indices
            let mut indices = Vec::with_capacity(vertices.len());
            for (i, _) in vertices.iter().enumerate() {
                indices.push(i as u32);
            }

            // normals
            let mut normals = Vec::with_capacity(vertices.len());
            for v in vertices.iter() {
                let n = v.normalize();
                normals.push(n);
            }

            // uvs
            let log = false;
            let mut hash_map = std::collections::HashMap::<u64, Vec<Vec3>>::new();
            let mut uvs = Vec::with_capacity(vertices.len());
            for v in vertices.iter_mut() {
                let hash = xxhash_vec3(*v, 0);

                if log {
                    match hash_map.get_mut(&hash) {
                        Some(values) => values.push(*v),
                        None => _ = hash_map.insert(hash, vec![*v]),
                    }
                }

                let x = Rng::hash_to_f32((hash >> 32) as u32);
                let y = Rng::hash_to_f32(hash as u32);
                let uv = Vec2(x, y);
                let value = (hash as f32 / u64::MAX as f32) * self.magnitude;
                *v = *v + *v * value;
                //let uv = Vec2::init(value);

                uvs.push(uv);
            }

            if log {
                let mut message = String::new();
                for (key, values) in hash_map.iter() {
                    for value in values.iter() {
                        message.push_str(&format!("\n{} {:?}", key, value));
                    }
                }
                ris_log::trace!("{}", message);
            }

            // generate mesh
            ris_log::trace!(
                "generate mesh... vertices: {} indices: {}",
                vertices.len(),
                indices.len()
            );
            let indices = Indices::U32(indices);
            let prototype = MeshPrototype {
                vertices,
                normals,
                uvs,
                indices,
            };

            let cpu_mesh = CpuMesh::try_from(prototype)?;
            ris_log::trace!("serialize...");
            let bytes = ris_mesh::serialize(&cpu_mesh)?;

            ris_log::trace!("write file...");
            let filepath = PathBuf::from("assets/in_use/meshes/planet_new.ris_mesh");

            if filepath.exists() {
                std::fs::remove_file(&filepath)?;
            }

            let mut file = std::fs::File::create_new(filepath)?;
            let f = &mut file;
            ris_io::write(f, &bytes)?;

            // add mesh component
            let handle = game_object.get_component::<MeshComponent>(&state.scene, GetFrom::This)?;
            let handle = match handle {
                Some(handle) => handle,
                None => game_object.add_component(&state.scene)?,
            };

            let mesh_component = state.scene.deref(handle)?;
            let asset_id = ris_asset_data::AssetId::Path("meshes/planet_new.ris_mesh".to_string());
            mesh_component.borrow_mut().set_asset_id(Some(asset_id));

            let total_duration = std::time::Instant::now() - start;
            let milliseconds = total_duration.as_secs_f32() * 1000.0;
            ris_log::trace!("done! duration: {}ms", milliseconds);
        } // generate mesh

        if ui.button("generate mesh for terrain renderer") {
            let start = std::time::Instant::now();
            ris_log::trace!("generate prototype...");

            // vertices
            ris_log::trace!("vertices...");
            let mut vertices = Vec::new();
            let tiles = 1 << 3;
            for i in 0..tiles {
                for j in 0..tiles {
                    let offset = tiles / 2;
                    let x = j - offset;
                    let y = i - offset;

                    let v0 = TerrainVertex(x, y);
                    let v1 = TerrainVertex(x, y + 1);
                    let v2 = TerrainVertex(x + 1, y);
                    let v3 = TerrainVertex(x + 1, y + 1);

                    vertices.push(v0);
                    vertices.push(v1);
                    vertices.push(v2);
                    vertices.push(v3);
                    vertices.push(v2);
                    vertices.push(v1);
                }
            }

            // indices
            ris_log::trace!("indices...");
            let mut indices = Vec::with_capacity(vertices.len());
            let mut unique_vertices = Vec::new();
            let mut lookup = std::collections::HashMap::<TerrainVertex, u32>::default();

            for vertex in vertices.iter() {
                match lookup.get(vertex) {
                    Some(index) => {
                        indices.push(*index);
                    }
                    None => {
                        let index = unique_vertices.len() as u32;
                        unique_vertices.push(*vertex);
                        indices.push(index);
                        lookup.insert(*vertex, index);
                    }
                }
            }

            ris_log::trace!(
                "vertices: {}, unique: {}",
                vertices.len(),
                unique_vertices.len(),
            );
            let vertices = unique_vertices;

            // generate mesh
            ris_log::trace!("generate mesh...");
            let prototype = TerrainMeshPrototype {
                vertices,
                indices: Indices::U32(indices),
            };
            let cpu_mesh = TerrainCpuMesh::try_from(prototype)?;

            // file
            ris_log::trace!("serialize...");
            let bytes = ris_terrain::serialize(&cpu_mesh)?;
            ris_log::trace!("bytes: {}", bytes.len());

            ris_log::trace!("write file...");
            let filepath = PathBuf::from("assets/in_use/terrain/demo.ris_terrain");

            if filepath.exists() {
                std::fs::remove_file(&filepath)?;
            }

            let mut file = std::fs::File::create_new(filepath)?;
            let f = &mut file;
            ris_io::write(f, &bytes)?;

            let total_duration = std::time::Instant::now() - start;
            let milliseconds = total_duration.as_secs_f32() * 1000.0;
            ris_log::trace!("done! duration: {}ms", milliseconds);
        } // generate mesh for terrain renderer
        
        if ui.button("make heightmaps") {

            //let width = 1 << 12;
            let width = 1 << 6;
            let height = width;
            let grid_width = 1 << 3;
            let grid_height = grid_width;
            //let max_height = 0xFFFF;
            let max_height = 0xFF;
            ris_log::trace!("resolution: {}x{}", width, height);


            let side_l = "l"; // -x left
            let side_r = "r"; // +x right
            let side_b = "b"; // -y back
            let side_f = "f"; // +y front
            let side_d = "d"; // -z down
            let side_u = "u"; // +z up

            let edge_lb = "lb";
            let edge_lf = "lf";
            let edge_ld = "ld";
            let edge_lu = "lu";
            let edge_rb = "rb";
            let edge_rf = "rf";
            let edge_rd = "rd";
            let edge_ru = "ru";
            let edge_bd = "bd";
            let edge_bu = "bu";
            let edge_fd = "fd";
            let edge_fu = "fu";

            let corn_lbd = "lbd";
            let corn_lfd = "lfd";
            let corn_lbu = "lbu";
            let corn_lfu = "lfu";
            let corn_rbd = "rbd";
            let corn_rfd = "rfd";
            let corn_rbu = "rbu";
            let corn_rfu = "rfu";

            struct Side<'a> {
                side: &'a str,
                edges: [&'a str; 4],
                corners: [&'a str; 4],
                perlin_sampler: PerlinSampler,
            }

            struct PerlinSampler {
                offset: (i32, i32),
                edge0: Option<Box<dyn Fn(i32) -> (i32, i32)>>,
                edge1: Option<Box<dyn Fn(i32) -> (i32, i32)>>,
                edge2: Option<Box<dyn Fn(i32) -> (i32, i32)>>,
                edge3: Option<Box<dyn Fn(i32) -> (i32, i32)>>,
                corn0: Option<(i32, i32)>,
                corn1: Option<(i32, i32)>,
                corn2: Option<(i32, i32)>,
                corn3: Option<(i32, i32)>,
            }

            //impl PerlinSampler {
            //    fn sample(

            //    ) {
            //        let coord = Vec2(x as f32, y as f32);
            //        let size = Vec2(width as f32, height as f32);
            //        let n = coord / size; // normalized
            //        let o = perlin_sampler.offset;
            //        let default = n + o;

            //        let sampled = if x == 0 {
            //            if y == 0 {
            //                perlin_sampler.corn0.unwrap_or(default)
            //            } else if y == height - 1 {
            //                perlin_sampler.corn2.unwrap_or(default)
            //            } else {
            //                perlin_sampler.edge0.as_ref().map(|s| s(n.y())).unwrap_or(default)
            //            }

            //        } else if x == width - 1 {
            //            if y == 0 {
            //                perlin_sampler.corn1.unwrap_or(default)
            //            } else if y == height -1 {
            //                perlin_sampler.corn3.unwrap_or(default)
            //            } else {
            //                perlin_sampler.edge1.as_ref().map(|s| s(n.y())).unwrap_or(default)
            //            }
            //        } else if y == 0 {
            //            perlin_sampler.edge2.as_ref().map(|s| s(n.x())).unwrap_or(default)
            //        } else if y == height - 1 {
            //            perlin_sampler.edge3.as_ref().map(|s| s(n.x())).unwrap_or(default)
            //        } else {
            //            default
            //        };

            //        let x = sampled.x() * grid_width as f32;
            //        let y = sampled.y() * grid_height as f32;

            //    }
            //}
            
            // (side, edges, corners, )
            let sides = vec![
                Side {
                    side: side_l,
                    edges: [
                        edge_lf, 
                        edge_lb, 
                        edge_lu, 
                        edge_ld,
                    ],
                    corners: [
                        corn_lfu, 
                        corn_lbu, 
                        corn_lfd, 
                        corn_lbd,
                    ],
                    perlin_sampler: PerlinSampler {
                        offset: (0, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                        corn0: None,
                        corn1: None,
                        corn2: None,
                        corn3: None,
                    },
                },
                Side {
                    side: side_r,
                    edges: [
                        edge_rb, 
                        edge_rf, 
                        edge_ru,
                        edge_rd,
                    ],
                    corners: [
                        corn_rbu, 
                        corn_rfu, 
                        corn_rbd, 
                        corn_rfd,
                    ],
                    perlin_sampler: PerlinSampler {
                        offset: (2, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                        corn0: None,
                        corn1: None,
                        corn2: None,
                        corn3: None,
                    },
                },
                Side {
                    side: side_b,
                    edges: [
                        edge_lb, 
                        edge_rb, 
                        edge_bu, 
                        edge_bd,
                    ],
                    corners: [
                        corn_lbu, 
                        corn_rbu, 
                        corn_lbd,
                        corn_rbd,
                    ],
                    perlin_sampler: PerlinSampler {
                        offset: (1, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                        corn0: None,
                        corn1: None,
                        corn2: None,
                        corn3: None,
                    },
                },
                Side {
                    side: side_f,
                    edges: [
                        edge_rf, 
                        edge_lf, 
                        edge_fu, 
                        edge_fd,
                    ],
                    corners: [
                        corn_rfu, 
                        corn_lfu, 
                        corn_rfd,
                        corn_lfd,
                    ],
                    perlin_sampler: PerlinSampler {
                        offset: (3, 0),
                        edge0: None,
                        edge1: Some(Box::new(|yi| (0, yi))),
                        edge2: None,
                        edge3: None,
                        corn0: None,
                        corn1: Some((0, 0)),
                        corn2: None,
                        corn3: Some((0, grid_height)),
                    },
                },
                //Side {
                //    side: side_d,
                //    edges: [
                //        edge_ld,
                //        edge_rd,
                //        edge_bd,
                //        edge_fd,
                //    ],
                //    corners: [
                //        corn_lbd,
                //        corn_rbd,
                //        corn_lfd,
                //        corn_rfd,
                //    ],
                //    perlin_sampler: PerlinSampler {
                //        offset: (1, 1),
                //        edge0: None,
                //        edge1: None,
                //        edge2: None,
                //        edge3: None,
                //        corn0: None,
                //        corn1: None,
                //        corn2: None,
                //        corn3: None,
                //    },
                //},
                Side {
                    side: side_u,
                    edges: [
                        edge_lu,
                        edge_ru,
                        edge_fu,
                        edge_bu,
                    ],
                    corners: [
                        corn_lfu,
                        corn_rfu,
                        corn_lbu, 
                        corn_rbu,
                    ],
                    perlin_sampler: PerlinSampler {
                        offset: (1, -1),
                        edge0: Some(Box::new(move |yi| (yi, 0))),
                        edge1: Some(Box::new(move |yi| (grid_width - yi + grid_width * 2, 0))),
                        edge2: Some(Box::new(move |xi| (grid_width - xi + grid_width * 3, 0))),
                        edge3: None,
                        corn0: Some((0, 0)),
                        corn1: Some((3 * grid_width, 0)),
                        corn2: None,
                        corn3: None,
                    },
                },
            ];

            let mut color_lookup = std::collections::HashMap::<&str, [u8; 3]>::new();

            color_lookup.insert(edge_lb, [64, 0, 0]);
            color_lookup.insert(edge_lf, [128, 0, 0]);
            color_lookup.insert(edge_ld, [0, 64, 64]);
            color_lookup.insert(edge_lu, [0, 128, 128]);
            color_lookup.insert(edge_rb, [0, 64, 0]);
            color_lookup.insert(edge_rf, [0, 128, 0]);
            color_lookup.insert(edge_rd, [64, 0, 64]);
            color_lookup.insert(edge_ru, [128, 0, 128]);
            color_lookup.insert(edge_bd, [0, 0, 64]);
            color_lookup.insert(edge_bu, [0, 0, 128]);
            color_lookup.insert(edge_fd, [64, 64, 0]);
            color_lookup.insert(edge_fu, [128, 128, 0]);

            color_lookup.insert(corn_lbd, [255, 0, 0]);
            color_lookup.insert(corn_lfd, [0, 255, 255]);
            color_lookup.insert(corn_lbu, [0, 255, 0]);
            color_lookup.insert(corn_lfu, [255, 0, 255]);
            color_lookup.insert(corn_rbd, [0, 0, 255]);
            color_lookup.insert(corn_rfd, [255, 255, 0]);
            color_lookup.insert(corn_rbu, [0, 0, 0]);
            color_lookup.insert(corn_rfu, [255, 255, 255]);

            for (i, side) in sides.into_iter().enumerate() {

                //let (side, [edge0, edge1, edge2, edge3], [corn0, corn1, corn2, corn3]) = side;
                let Side {
                    side,
                    edges,
                    corners,
                    perlin_sampler,
                } = side;

                ris_log::trace!("generating side... {} ({})", side, i);

                let mut bytes = Vec::with_capacity(width * height * 3);

                let mut min: u32 = u32::MAX;
                let mut max: u32 = u32::MIN;

                for y in 0..width {
                    if y % 100 == 0 {
                        ris_log::trace!("y... {}/{}", y, width);
                    }

                    for x in 0..height {
                        // height map format:
                        // u20 for height and u4 for material
                        //
                        //       material
                        //       vv
                        // 0xRRGGBB
                        //   ^--^
                        //    height
                        //   
                        // R = height middle byte
                        // G = height LSB
                        // B = height MSB + material
 
                        let coord = Vec2(x as f32, y as f32);
                        let size = Vec2(width as f32, height as f32);
                        let normalized = coord / size; // normalized
                        //let pos = n + perlin_sampler.offset; // position on cube/net
                        let grid = Vec2(grid_width as f32, grid_height as f32);
                        let grid_pos = normalized * grid;

                        // this closure connects the edges and corners of different sizes, to
                        // ensure that the perlin noise ist continuous over the whole cube
                        let apply_net = |xi: i32, yi: i32| {
                            let offset_x = perlin_sampler.offset.0 * grid_width;
                            let offset_y = perlin_sampler.offset.1 * grid_height;
                            let default_x = xi + offset_x;
                            let default_y = yi + offset_y;
                            let default = (default_x, default_y);

                            if xi == 0 {
                                if yi == 0 {
                                    perlin_sampler.corn0.unwrap_or(default)
                                } else if yi == grid_height {
                                    perlin_sampler.corn2.unwrap_or(default)
                                } else {
                                    perlin_sampler.edge0
                                        .as_ref()
                                        .map(|edge| edge(yi))
                                        .unwrap_or(default)
                                }
                            } else if xi == grid_width {
                                if yi == 0 {
                                    perlin_sampler.corn1.unwrap_or(default)
                                } else if yi == grid_height {
                                    perlin_sampler.corn3.unwrap_or(default)
                                } else {
                                    perlin_sampler.edge1
                                        .as_ref()
                                        .map(|edge| edge(yi))
                                        .unwrap_or(default)
                                }
                            } else if yi == 0 {
                                perlin_sampler.edge2
                                    .as_ref()
                                    .map(|edge| edge(xi))
                                    .unwrap_or(default)
                            } else if yi == grid_height {
                                perlin_sampler.edge3
                                    .as_ref()
                                    .map(|edge| edge(xi))
                                    .unwrap_or(default)
                            } else {
                                default
                            }
                        };

                        let Vec2(x, y) = grid_pos;
                        let x0 = x.floor() as i32;
                        let x1 = x0 + 1;
                        let y0 = y.floor() as i32;
                        let y1 = y0 + 1;

                        let v0 = apply_net(x0, y0);
                        let v1 = apply_net(x1, y0);
                        let v2 = apply_net(x0, y1);
                        let v3 = apply_net(x1, y1);
                        let r0 = random_gradient(v0.0, v0.1);
                        let r1 = random_gradient(v1.0, v1.1);
                        let r2 = random_gradient(v2.0, v2.1);
                        let r3 = random_gradient(v3.0, v3.1);

                        // perlin noise
                        //let noise_value = perlin_noise(x, y);
                        //let x0 = x.floor() as i32;
                        //let x1 = x0 + 1;
                        //let y0 = y.floor() as i32;
                        //let y1 = y0 + 1;

                        let sx = x - x0 as f32;
                        let sy = y - y0 as f32;

                        //let n0 = dot_grid_gradient(x0, y0, x, y);
                        //let n1 = dot_grid_gradient(x1, y0, x, y);
                        let n0 = dot_grid_gradient(x0, y0, x, y, r0.0, r0.1);
                        let n1 = dot_grid_gradient(x1, y0, x, y, r1.0, r1.1);
                        let ix0 = interpolate(n0, n1, sx);
                        //let n0 = dot_grid_gradient(x0, y1, x, y);
                        //let n1 = dot_grid_gradient(x1, y1, x, y);
                        let n0 = dot_grid_gradient(x0, y1, x, y, r2.0, r2.1);
                        let n1 = dot_grid_gradient(x1, y1, x, y, r3.0, r3.1);
                        let ix1 = interpolate(n0, n1, sx);

                        let noise_value = interpolate(ix0, ix1, sy);
                        // perlin noise end

                        let scaled = noise_value * f32::sqrt(2.0) * 0.5 + 0.5;
                        let height_value = (scaled * max_height as f32) as u32;

                        min = u32::min(min, height_value);
                        max = u32::max(max, height_value);

                        let height_bytes = height_value.to_le_bytes();
                        let lsb = height_bytes[0];
                        let msb = height_bytes[1];
                        let material = 0u8;

                        //let g = lsb;
                        //let r = msb;
                        //let b = material;

                        //bytes.push(r);
                        //bytes.push(g);
                        //bytes.push(b);
                        bytes.push(lsb);
                        bytes.push(lsb);
                        bytes.push(lsb);
                    }
                }

                //panic!("{} {}", min, max);

                ris_log::trace!("encoding to qoi...");
                let desc = QoiDesc {
                    width: width as u32,
                    height: height as u32,
                    channels: Channels::RGB,
                    color_space: ColorSpace::Linear,
                };
                let qoi_bytes = qoi::encode(&bytes, desc)?;

                ris_log::trace!(
                    "bytes len: {} qoi len: {}",
                    bytes.len(),
                    qoi_bytes.len(),
                );

                ris_log::trace!("serializing...");

                let path_string = format!("assets/in_use/terrain/height_map_{}.qoi", side);
                let filepath = PathBuf::from(path_string);

                if filepath.exists() {
                    std::fs::remove_file(&filepath)?;
                }

                let mut file = std::fs::File::create_new(filepath)?;
                let f = &mut file;
                ris_io::write(f, &qoi_bytes)?;

                ris_log::trace!("done!");
            };

        } // make height maps

        let p = state.camera.borrow().position;
        let abs = p.abs();

        if abs.0 > abs.1 && abs.0 > abs.2 {
            let Vec3(x, y, z) = p;
            let sign = x.signum();

            let mz = z / x;
            let z_ = mz * sign;
            let my = z / y;
            let y_ = z_ / my;
            let p_ = Vec3(sign, y_, z_);

            ris_debug::gizmo::point(p_, Some(Rgb::red()))?;
            ui.label_text("point", format!("{:?}", p_));
            if p.0.is_sign_positive() {
                ui.label_text("face", "right");
            } else {
                ui.label_text("face", "left");
            }
        } else if abs.1 > abs.0 && abs.1 > abs.2 {
            let Vec3(x, y, z) = p;
            let sign = y.signum();

            let mz = z / y;
            let z_ = mz * sign;
            let mx = z / x;
            let x_ = z_ / mx;
            let p_ = Vec3(x_, sign, z_);

            ris_debug::gizmo::point(p_, Some(Rgb::red()))?;
            ui.label_text("point", format!("{:?}", p_));
            if p.1.is_sign_positive() {
                ui.label_text("face", "forward");
            } else {
                ui.label_text("face", "back");
            }
        } else {
            let Vec3(x, y, z) = p;
            let sign = z.signum();

            let my = y / z;
            let y_ = my * sign;
            let mx = y / x;
            let x_ = y_ / mx;
            let p_ = Vec3(x_, y_, sign);

            ris_debug::gizmo::point(p_, Some(Rgb::red()))?;
            ui.label_text("point", format!("{:?}", p_));
            if p.2.is_sign_positive() {
                ui.label_text("face", "up");
            } else {
                ui.label_text("face", "down");
            }
        }

        Ok(())
    }
}

fn serialize_mesh(prototype: MeshPrototype, path: impl AsRef<str>) -> RisResult<()> {
    let cpu_mesh = CpuMesh::try_from(prototype)?;
    ris_log::trace!("serialize...");
    let bytes = ris_mesh::serialize(&cpu_mesh)?;

    ris_log::trace!("write file...");
    let filepath = PathBuf::from(path.as_ref());

    if filepath.exists() {
        std::fs::remove_file(&filepath)?;
    }

    let mut file = std::fs::File::create_new(filepath)?;
    let f = &mut file;
    ris_io::write(f, &bytes)?;

    Ok(())
}

// inspired by xxhash. i took XXH3_64bits() and inlined the specific branches. i also did some
// tweaks to avoid floating-point imprecision and to prevent naive collisions
fn xxhash_vec3(value: Vec3, seed: u64) -> u64 {
    let value = value * 2_000_000f32;
    let input_0 = u64::from_le_bytes((value.0 as i64).to_le_bytes());
    let input_1 = u64::from_le_bytes((value.1 as i64).to_le_bytes());
    let input_2 = u64::from_le_bytes((value.2 as i64).to_le_bytes());

    // XXH_PUBLIC_API XXH64_hash_t XXH3_64bits(XXH_NOESCAPE const void* input, size_t length)
    // XXH3_64bits_internal(input, length, 0, XXH3_kSecret, sizeof(XXH3_kSecret), XXH3_hashLong_64b_default)
    // XXH3_len_0to16_64b((const xxh_u8*)input, len, (const xxh_u8*)secret, seed64)
    // XXH3_len_9to16_64b(input, len, secret, seed)
    //let secret: [u8; 192] = [
    //    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c,
    //    0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    //    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21,
    //    0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    //    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3,
    //    0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    //    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d,
    //    0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    //    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
    //    0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    //    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce,
    //    0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
    //];

    //let bitflip1 = (u64::from_le_bytes(secret[24..32].try_into().unwrap()) ^ u64::from_le_bytes(secret[32..40].try_into().unwrap())).wrapping_add(seed);
    //let bitflip2 = (u64::from_le_bytes(secret[40..48].try_into().unwrap()) ^ u64::from_le_bytes(secret[48..56].try_into().unwrap())).wrapping_sub(seed);

    let bitflip1 = 7458650908927343033u64.wrapping_add(seed);
    let bitflip2 = 12634492766384443962u64.wrapping_sub(seed);

    let input_lo = (input_0 | (input_1 << 32)) ^ bitflip1;
    //let input_hi = (input1 | (input2 << 32)) ^ bitflip2;
    let input_hi = input_2 ^ bitflip2;

    let mul128 = (input_lo as u128).wrapping_mul(input_hi as u128);
    let fold64 = (mul128 as u64) ^ ((mul128 << 64) as u64);
    let acc = 12u64
        .wrapping_add(input_lo.swap_bytes())
        .wrapping_add(input_hi)
        .wrapping_add(fold64);

    // XXH3_avalanche(acc)
    let mut h64 = acc;
    let prime_mx1 = 0x165667919E3779F9u64;
    h64 = h64 ^ (h64 >> 37);
    h64 = h64.wrapping_mul(prime_mx1);
    h64 = h64 ^ (h64 >> 32);
    h64
}

/*fn perlin_noise(x: f32, y: f32) -> f32 {
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);
    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);

    interpolate(ix0, ix1, sy)
}*/

fn dot_grid_gradient(
    ix: i32,
    iy: i32,
    x: f32,
    y: f32,
    grad_x: f32,
    grad_y: f32,
) -> f32 {
    //let (grad_x, grad_y) = random_gradient(ix, iy);
    let dx = x - ix as f32;
    let dy = y - iy as f32;
    dx * grad_x + dy * grad_y
}

fn random_gradient(ix: i32, iy: i32) -> (f32, f32) {
    let w = (8 * std::mem::size_of::<u32>()) as u32;
    let s = w / 2;
    let a = ix as u32;
    let b = iy as u32;
    let a = a.wrapping_mul(3284157443);
    let b = b ^ ((a << s) | (a >> (w-s)));
    let b = b.wrapping_mul(1911520717);
    let a = a ^ ((b << s) | (b >> (w-s)));
    let a = a.wrapping_mul(2048419325);
    let random = a as f32 * (PI / (!(!0u32 >> 1) as f32));
    let v_x = f32::cos(random);
    let v_y = f32::sin(random);
    (v_x, v_y)
}

fn interpolate(a0: f32, a1: f32, x: f32) -> f32 {
    let g = (3.0 - x * 2.0) * x * x;
    (a1 - a0) * g + a0
}
