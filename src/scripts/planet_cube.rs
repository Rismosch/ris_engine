use std::cell::RefCell;
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
use ris_math::color::ByteColor;
use ris_math::color::Gradient;
use ris_math::color::OkLab;
use ris_math::color::Rgb;
use ris_math::matrix::Mat2;
use ris_math::quaternion::Quat;
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
        let seed = Seed::new();
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
            frame,
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

            let seed = Seed::default();
            let mut rng = Rng::new(seed);
            let only_generate_first_face = false;
            //let width = (1 << 12) + 1; // 1 meter between vertices
            let width = (1 << 6) + 1;
            let continent_count = 7;
            const KERNEL_SIZE: usize = 5;
            const _: () = {
                assert!(KERNEL_SIZE % 2 == 1);
            };
            let kernel_sigma = 1.0;
            let max_h = 0xFFFF;
            ris_log::trace!("resolution: {}x{}", width, width);

            let side_l = "l"; // -x left
            let side_r = "r"; // +x right
            let side_b = "b"; // -y back
            let side_f = "f"; // +y front
            let side_d = "d"; // -z down
            let side_u = "u"; // +z up

            #[derive(Clone, Copy)]
            struct HeightMapValue {
                height: f32,
                continent_index: usize,
            }

            struct HeightMap<'a> {
                side: &'a str,
                values: Vec<HeightMapValue>,
                width: usize,
            }

            impl<'a> HeightMap<'a> {
                fn new(side: &'a str, width: usize) -> Self {
                    let value = HeightMapValue {
                        height: 0.0,
                        continent_index: usize::MAX,
                    };

                    Self {
                        side,
                        values: vec![value; width * width],
                        width,
                    }
                }

                fn get(&self, x: usize, y: usize) -> HeightMapValue {
                    let i = self.index(x, y);
                    self.values[i]
                }

                fn set(&mut self, x: usize, y: usize, value: HeightMapValue) {
                    let i = self.index(x, y);
                    self.values[i] = value;
                }

                fn index(&self, x: usize, y: usize) -> usize {
                    x + y * self.width
                }
            }

            struct Side<'a> {
                perlin_sampler: PerlinSampler,
                height_map: RefCell<HeightMap<'a>>,
            }

            struct PerlinSampler {
                offset: (i32, i32),
                edge0: Option<Box<dyn Fn(i32, (i32, i32)) -> ((i32, i32), Mat2)>>,
                edge1: Option<Box<dyn Fn(i32, (i32, i32)) -> ((i32, i32), Mat2)>>,
                edge2: Option<Box<dyn Fn(i32, (i32, i32)) -> ((i32, i32), Mat2)>>,
                edge3: Option<Box<dyn Fn(i32, (i32, i32)) -> ((i32, i32), Mat2)>>,
            }

            #[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
            struct ContinentPixel {
                side: usize,
                ix: usize,
                iy: usize,
            }

            #[derive(Default, Debug, Clone)]
            struct Continent {
                origin: ContinentPixel,
                discovered_pixels: Vec<ContinentPixel>,
                rotation_axis: Vec3,
            }

            let mut sides = vec![
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (0, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                    },
                    height_map: RefCell::new(HeightMap::new(side_l, width)),
                },
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (1, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                    },
                    height_map: RefCell::new(HeightMap::new(side_b, width)),
                },
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (2, 0),
                        edge0: None,
                        edge1: None,
                        edge2: None,
                        edge3: None,
                    },
                    height_map: RefCell::new(HeightMap::new(side_r, width)),
                },
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (3, 0),
                        edge0: None,
                        edge1: Some(Box::new(|iy, (gw, gh)| ((0, iy), Mat2::identity()))),
                        edge2: None,
                        edge3: None,
                    },
                    height_map: RefCell::new(HeightMap::new(side_f, width)),
                },
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (1, -1),
                        edge0: Some(Box::new(move |iy, (gw, gh)| ((iy, 0), Mat2(Vec2(0.0, 1.0), Vec2(-1.0, 0.0))))),
                        edge1: Some(Box::new(move |iy, (gw, gh)| ((gw - iy + gw * 2, 0), Mat2(Vec2(0.0, -1.0), Vec2(1.0, 0.0))))),
                        edge2: Some(Box::new(move |ix, (gw, gh)| ((gw - ix + gw * 3, 0), Mat2(Vec2(-1.0, 0.0), Vec2(0.0, -1.0))))),
                        edge3: None,
                    },
                    height_map: RefCell::new(HeightMap::new(side_u, width)),
                },
                Side {
                    perlin_sampler: PerlinSampler {
                        offset: (1, 1),
                        edge0: Some(Box::new(move |iy, (gw, gh)| ((gw - iy, gh), Mat2(Vec2(0.0, -1.0), Vec2(1.0, 0.0))))),
                        edge1: Some(Box::new(move |iy, (gw, gh)| ((iy + 2 * gw, gh), Mat2(Vec2(0.0, 1.0), Vec2(-1.0, 0.0))))),
                        edge2: None,
                        edge3: Some(Box::new(move |ix, (gw, gh)| ((gw - ix + gw * 3, gh), Mat2(Vec2(-1.0, 0.0), Vec2(0.0, -1.0))))),
                    },
                    height_map: RefCell::new(HeightMap::new(side_d, width)),
                },
            ];

            let mut continents = vec![Continent::default(); continent_count];

            let mut height_maps = Vec::new();

            // continents
            ris_log::trace!("determine continent starting positions...");
            let mut starting_positions = Vec::<ContinentPixel>::with_capacity(continent_count);

            for _ in 0..starting_positions.capacity() {

                loop {
                    let side = rng.next_i32_between(0, 5) as usize;
                    let ix = rng.next_i32_between(0, width as i32 - 1) as usize;
                    let iy = rng.next_i32_between(0, width as i32) as usize;

                    let candidate = ContinentPixel {
                        side,
                        ix,
                        iy,
                    };

                    let candidate_exists = starting_positions.iter().any(|x| *x == candidate);
                    if candidate_exists {
                        continue;
                    }

                    starting_positions.push(candidate);
                    break;
                }
            }

            for (i, starting_position) in starting_positions.into_iter().enumerate() {
                let side = &mut sides[starting_position.side];
                side.height_map.borrow_mut().set(
                    starting_position.ix,
                    starting_position.iy,
                    HeightMapValue{
                        height: -1.0,
                        continent_index: usize::MAX,
                    },
                );

                let continent = &mut continents[i];
                continent.origin = starting_position.clone();
                continent.discovered_pixels.push(starting_position);
                continent.rotation_axis = rng.next_dir_3();
            }

            ris_log::trace!("generate continents...");
            let mut discovered_pixel_count = 0;
            loop {
                // discover new pixels
                let mut new_pixel_was_discovered = false;

                for (continent_index, continent) in continents.iter_mut().enumerate() {
                    let mut pixel = None;
                    loop {
                        if continent.discovered_pixels.is_empty() {
                            break;
                        }

                        let min = 0i32;
                        let max = continent.discovered_pixels.len() as i32 - 1;
                        let index = rng.next_i32_between(min, max) as usize;
                        let candidate = continent.discovered_pixels.swap_remove(index);

                        let side = &mut sides[candidate.side];
                        let mut h = side.height_map.borrow().get(candidate.ix, candidate.iy);

                        if h.continent_index != usize::MAX {
                            continue;
                        }

                        new_pixel_was_discovered = true;

                        h.continent_index = continent_index;
                        discovered_pixel_count += 1;

                        side.height_map.borrow_mut().set(
                            candidate.ix,
                            candidate.iy,
                            h,
                        );

                        pixel = Some(candidate);
                        break;
                    }

                    let Some(pixel) = pixel else {
                        continue;
                    };
                    
                    // walk left
                    let new_pixel = if pixel.ix == 0 {
                        match pixel.side {
                            // left -> front
                            0 => ContinentPixel {
                                side: 3,
                                ix: width - 1,
                                iy: pixel.iy,
                            },
                            // back -> left
                            1 => ContinentPixel {
                                side: 0,
                                ix: width - 1,
                                iy: pixel.iy,
                            },
                            // right -> back
                            2 => ContinentPixel {
                                side: 1,
                                ix: width - 1,
                                iy: pixel.iy,
                            },
                            // front -> right
                            3 => ContinentPixel {
                                side: 2,
                                ix: width - 1,
                                iy: pixel.iy,
                            },
                            // up -> left
                            4 => ContinentPixel {
                                side: 0,
                                ix: pixel.iy,
                                iy: 0,
                            },
                            // down -> left
                            5 => ContinentPixel {
                                side: 0,
                                ix: width - 1 - pixel.iy,
                                iy: width - 1,
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        ContinentPixel {
                            side: pixel.side,
                            ix: pixel.ix - 1,
                            iy: pixel.iy,
                        }
                    };
                    continent.discovered_pixels.push(new_pixel);

                    // walk right
                    let new_pixel = if pixel.ix == width - 1 {
                        match pixel.side {
                            // left -> back
                            0 => ContinentPixel {
                                side: 1,
                                ix: 0,
                                iy: pixel.iy,
                            },
                            // back -> right
                            1 => ContinentPixel {
                                side: 2,
                                ix: 0,
                                iy: pixel.iy,
                            },
                            // right -> front
                            2 => ContinentPixel {
                                side: 3,
                                ix: 0,
                                iy: pixel.iy,
                            },
                            // front -> left
                            3 => ContinentPixel {
                                side: 0,
                                ix: 0,
                                iy: pixel.iy,
                            },
                            // up -> right
                            4 => ContinentPixel {
                                side: 2,
                                ix: width - 1 - pixel.iy,
                                iy: 0,
                            },
                            // down -> right
                            5 => ContinentPixel {
                                side: 2,
                                ix: pixel.iy,
                                iy: width - 1,
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        ContinentPixel {
                            side: pixel.side,
                            ix: pixel.ix + 1,
                            iy: pixel.iy,
                        }
                    };
                    continent.discovered_pixels.push(new_pixel);

                    // walk up
                    let new_pixel = if pixel.iy == 0 {
                        match pixel.side {
                            // left -> up
                            0 => ContinentPixel {
                                side: 4,
                                ix: 0,
                                iy: pixel.ix,
                            },
                            // back -> up
                            1 => ContinentPixel {
                                side: 4,
                                ix: pixel.ix,
                                iy: width - 1,
                            },
                            // right -> up
                            2 => ContinentPixel {
                                side: 4,
                                ix: width - 1,
                                iy: width - 1 - pixel.ix,
                            },
                            // front -> up
                            3 => ContinentPixel {
                                side: 4,
                                ix: width - 1 - pixel.ix,
                                iy: 0,
                            },
                            // up -> front
                            4 => ContinentPixel {
                                side: 3,
                                ix: width - 1 - pixel.ix,
                                iy: 0,
                            },
                            // down -> back
                            5 => ContinentPixel {
                                side: 1,
                                ix: pixel.ix,
                                iy: width - 1,
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        ContinentPixel {
                            side: pixel.side,
                            ix: pixel.ix,
                            iy: pixel.iy - 1,
                        }
                    };
                    continent.discovered_pixels.push(new_pixel);

                    // walk down
                    let new_pixel = if pixel.iy == width - 1 {
                        match pixel.side {
                            // left -> down
                            0 => ContinentPixel {
                                side: 5,
                                ix: 0,
                                iy: width - 1 - pixel.ix,
                            },
                            // back -> down
                            1 => ContinentPixel {
                                side: 5,
                                ix: pixel.ix,
                                iy: 0,
                            },
                            // right -> down
                            2 => ContinentPixel {
                                side: 5,
                                ix: width - 1,
                                iy: pixel.ix,
                            },
                            // front -> down
                            3 => ContinentPixel {
                                side: 5,
                                ix: width - 1 - pixel.ix,
                                iy: width - 1,
                            },
                            // up -> back
                            4 => ContinentPixel {
                                side: 1,
                                ix: pixel.ix,
                                iy: 0,
                            },
                            // down -> front
                            5 => ContinentPixel {
                                side: 3,
                                ix: width -1 - pixel.ix,
                                iy: 0,
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        ContinentPixel {
                            side: pixel.side,
                            ix: pixel.ix,
                            iy: pixel.iy + 1,
                        }
                    };
                    continent.discovered_pixels.push(new_pixel);
                }
                
                if !new_pixel_was_discovered {
                    break
                }
            }

            // generate kernel
            ris_log::trace!("generate kernel");
            let mut kernel = [[0.0; KERNEL_SIZE]; KERNEL_SIZE];

            let kernel_half = (KERNEL_SIZE / 2) as isize;
            let mut sum = 0.0;
            let s = 2.0 * kernel_sigma * kernel_sigma;

            // fill the kernel with values
            for (ix, column) in kernel.iter_mut().enumerate() {
                for (iy, v) in column.iter_mut().enumerate() {
                    let x = (ix as isize - kernel_half) as f32;
                    let y = (iy as isize - kernel_half) as f32;

                    *v = (f32::exp(-(x * x + y * y) / (2.0 * s))) / (s * PI);
                    sum += *v;
                }
            }

            // normalize kernel
            for column in kernel.iter_mut() {
                for v in column.iter_mut() {
                    *v /= sum;
                }
            }

            // calculate heights on continent boundaries
            ris_log::trace!("calculate height based on plate boundaries... {}", discovered_pixel_count);

            for side in sides.iter() {
                let Side {
                    perlin_sampler,
                    height_map,
                } = side;

                for iy in 0..width {
                    if iy % 1000 == 0 {
                        ris_log::trace!(
                            "finding plate boundaries {}... progress: {}/{}", 
                            height_map.borrow().side,
                            iy, 
                            width,
                        );
                    }

                    for ix in 0..width {
                        let mut h = height_map.borrow().get(ix, iy);
                        let continent_index_lhs = h.continent_index;
                        let continent = &continents[continent_index_lhs];

                        for (i, dx) in (-kernel_half..=kernel_half).enumerate() {
                            for (j, dy) in (-kernel_half..=kernel_half).enumerate() {
                                let kernel_weight = kernel[i][j];

                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                
                                let mut ix_ = ix as isize + dx;
                                let mut iy_ = iy as isize + dy;
                                let mut side_ = height_map.borrow().side;

                                let w = width as isize;

                                let falls_on_upper_left = ix_ < 0 && iy_ < 0;
                                let falls_on_upper_right = ix_ >= w && iy_ < 0;
                                let falls_on_lower_left = ix_ < 0 && iy_ < w;
                                let falls_on_lower_right = ix_ < w && iy_ < w;

                                let falls_on_corner = 
                                    falls_on_upper_left ||
                                    falls_on_upper_right ||
                                    falls_on_lower_left ||
                                    falls_on_lower_right;

                                if falls_on_corner {
                                    continue;
                                }

                                // map ix_ and iy_ onto correct side
                            }
                        }

                        //let angle = 2.0 * PI / (4 * width) as f32;
                        //let rotation = Quat::angle_axis(angle, continent.rotation_axis);

                        //let p = position_on_sphere(
                        //    (ix, iy),
                        //    width,
                        //    height_map.borrow().side,
                        //);
                        //let p_ = rotation.rotate(p);
                        //let dir = (p_ - p).normalize();
                        //ris_log::debug!("{:?}", dir);


                        break;
                    }

                    break;
                }

                break;
            }

            //let angle = 2.0 * PI / (4 * width) as f32;
            //ris_log::trace!("calculate height based on plate boundaries... {}", discovered_pixel_count);
            //for side in sides.iter() {
            //    let Side {
            //        perlin_sampler,
            //        height_map,
            //    } = side;

            //    for iy in 0..width {
            //        if iy % 10 == 0 {
            //            ris_log::trace!(
            //                "finding plate boundaries {}... progress: {}/{}", 
            //                height_map.borrow().side,
            //                iy, 
            //                width,
            //            );
            //        }

            //        for ix in 0..width {
            //            let mut h = height_map.borrow().get(ix, iy);
            //            let continent_index_lhs = h.continent_index;
            //            let continent = &continents[continent_index_lhs];

            //            // find neirest neighbor and distance to it
            //            let neighbor_position = (ix, iy);
            //            let neighbor_side = height_map.borrow().side;
            //            let neighbor_distance = f32::MAX;

            //            #[derive(Debug, Clone, Copy)]
            //            struct Neighbor<'a>{
            //                side: &'a str,
            //                position: (usize, usize),
            //                distance: usize,
            //            }

            //            impl PartialEq for Neighbor<'_> {
            //                fn eq(&self, other: &Self) -> bool {
            //                    self.side == other.side &&
            //                        self.position == other.position
            //                }
            //            }

            //            impl Eq for Neighbor<'_> {}

            //            impl std::hash::Hash for Neighbor<'_> {
            //                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            //                    state.write(self.side.as_bytes());
            //                    state.write(&self.position.0.to_ne_bytes());
            //                    state.write(&self.position.1.to_ne_bytes());
            //                }
            //            }

            //            let mut best_candidate = Neighbor{
            //                side: height_map.borrow().side,
            //                position: (ix, iy),
            //                distance: usize::MAX,
            //            };

            //            let mut queue = std::collections::VecDeque::new();
            //            queue.push_back(best_candidate.clone());
            //            
            //            // breadth first sears bfs
            //            let mut discovered = std::collections::HashSet::<Neighbor>::new();

            //            while let Some(candidate) = queue.pop_front() {
            //                let is_new = discovered.insert(candidate);
            //                if !is_new {
            //                    continue;
            //                }

            //                let side_index = match candidate.side {
            //                    "l" => 0,
            //                    "b" => 1,
            //                    "r" => 2,
            //                    "f" => 3,
            //                    "u" => 4,
            //                    "d" => 5,
            //                    _ => unreachable!(),
            //                };
            //                if sides[side_index].height_map.borrow().get(ix, iy).continent_index != h.continent_index {
            //                    // found new candidate!
            //                    if candidate.distance < best_candidate.distance {
            //                        best_candidate = candidate;
            //                    }
            //                    continue;
            //                }

            //                let distance = if candidate.distance == usize::MAX {
            //                    1
            //                } else {
            //                    candidate.distance + 1
            //                };

            //                // move left
            //                let new_candidate = if candidate.position.0 == 0 {
            //                    match candidate.side {
            //                        "l" => Neighbor{
            //                            side: "f",
            //                            position: (
            //                                width - 1,
            //                                candidate.position.1,
            //                            ),
            //                            distance,
            //                        },
            //                        "b" => Neighbor{
            //                            side: "l",
            //                            position: (
            //                                width - 1,
            //                                candidate.position.1,
            //                            ),
            //                            distance,
            //                        },
            //                        "r" => Neighbor{
            //                            side: "b",
            //                            position: (
            //                                width - 1,
            //                                candidate.position.1,
            //                            ),
            //                            distance,
            //                        },
            //                        "f" => Neighbor{
            //                            side: "r",
            //                            position: (
            //                                width - 1,
            //                                candidate.position.1,
            //                            ),
            //                            distance,
            //                        },
            //                        "u" => Neighbor{
            //                            side: "l",
            //                            position: (
            //                                candidate.position.1,
            //                                0,
            //                            ),
            //                            distance,
            //                        },
            //                        "d" => Neighbor{
            //                            side: "l",
            //                            position: (
            //                                width - 1 - candidate.position.1,
            //                                width - 1,
            //                            ),
            //                            distance,
            //                        },
            //                        _ => unreachable!(),
            //                    }
            //                } else {
            //                    Neighbor {
            //                        side: candidate.side,
            //                        position: (candidate.position.0 - 1, candidate.position.1),
            //                        distance,
            //                    }
            //                };
            //                queue.push_back(new_candidate);

            //                // move right
            //                let new_candidate = if candidate.position.0 == width - 1 {
            //                    match candidate.side {
            //                        "l" => Neighbor {
            //                            side: "b",
            //                            position: (0, candidate.position.1),
            //                            distance,
            //                        },
            //                        "b" => Neighbor {
            //                            side: "r",
            //                            position: (0, candidate.position.1),
            //                            distance,
            //                        },
            //                        "r" => Neighbor {
            //                            side: "f",
            //                            position: (0, candidate.position.1),
            //                            distance,
            //                        },
            //                        "f" => Neighbor {
            //                            side: "l",
            //                            position: (0, candidate.position.1),
            //                            distance,
            //                        },
            //                        "u" => Neighbor {
            //                            side: "r",
            //                            position: (width - 1 - candidate.position.1,0),
            //                            distance,
            //                        },
            //                        "d" => Neighbor {
            //                            side: "r",
            //                            position: (candidate.position.1,width - 1),
            //                            distance,
            //                        },
            //                        _ => unreachable!(),
            //                    }
            //                } else {
            //                    Neighbor {
            //                        side: candidate.side,
            //                        position: (candidate.position.0 + 1, candidate.position.1),
            //                        distance,
            //                    }
            //                };
            //                queue.push_back(new_candidate);

            //                // move up
            //                let new_candidate = if candidate.position.1 == 0 {
            //                    match candidate.side {
            //                        "l" => Neighbor {
            //                            side: "u",
            //                            position: (0, candidate.position.0),
            //                            distance,
            //                        },
            //                        "b" => Neighbor {
            //                            side: "u",
            //                            position: (candidate.position.0, width - 1),
            //                            distance,
            //                        },
            //                        "r" => Neighbor {
            //                            side: "u",
            //                            position: (width - 1, width - 1 - candidate.position.0),
            //                            distance,
            //                        },
            //                        "f" => Neighbor {
            //                            side: "u",
            //                            position: (width - 1 - candidate.position.0, 0),
            //                            distance,
            //                        },
            //                        "u" => Neighbor {
            //                            side: "f",
            //                            position: (width - 1 - candidate.position.0, 0),
            //                            distance,
            //                        },
            //                        "d" => Neighbor {
            //                            side: "b",
            //                            position: (candidate.position.0, width - 1),
            //                            distance,
            //                        },
            //                        _ => unreachable!(),
            //                    }
            //                } else {
            //                    Neighbor {
            //                        side: candidate.side,
            //                        position: (candidate.position.0, candidate.position.1 - 1),
            //                        distance,
            //                    }
            //                };
            //                queue.push_back(new_candidate);

            //                // move down
            //                let new_candidate = if candidate.position.1 == width - 1 {
            //                    match candidate.side {
            //                        "l" => Neighbor {
            //                            side: "d",
            //                            position: (0, width - 1 - candidate.position.0),
            //                            distance,
            //                        },
            //                        "b" => Neighbor {
            //                            side: "d",
            //                            position: (candidate.position.0, 0),
            //                            distance,
            //                        },
            //                        "r" => Neighbor {
            //                            side: "d",
            //                            position: (width - 1, candidate.position.0),
            //                            distance,
            //                        },
            //                        "f" => Neighbor {
            //                            side: "d",
            //                            position: (width - 1 - candidate.position.0, width - 1),
            //                            distance,
            //                        },
            //                        "u" => Neighbor {
            //                            side: "b",
            //                            position: (candidate.position.0, 0),
            //                            distance,
            //                        },
            //                        "d" => Neighbor {
            //                            side: "f",
            //                            position: (width - 1 - candidate.position.0, 0),
            //                            distance,
            //                        },
            //                        _ => unreachable!(),
            //                    }
            //                } else {
            //                    Neighbor {
            //                        side: candidate.side,
            //                        position: (candidate.position.0, candidate.position.1 + 1),
            //                        distance,
            //                    }
            //                };
            //                queue.push_back(new_candidate);
            //            }

            //            // best candidate found:
            //            let mut h = height_map.borrow().get(ix, iy);

            //            let side_index = match best_candidate.side {
            //                "l" => 0,
            //                "b" => 1,
            //                "r" => 2,
            //                "f" => 3,
            //                "u" => 4,
            //                "d" => 5,
            //                _ => unreachable!(),
            //            };
            //            let continent_index = sides[side_index].height_map.borrow().get(ix, iy).continent_index;
            //            h.height = continent_index as f32;
            //            height_map.borrow_mut().set(ix, iy, h);
            //        }
            //    }
            //}

            // continents end

            while let Some(side) = sides.pop() {
                let Side {
                    perlin_sampler,
                    mut height_map,
                } = side;

                for h in height_map.borrow_mut().values.iter_mut() {
                    if h.height >= 0.0 {
                        h.height = h.continent_index as f32;
                    }
                }

                height_maps.push(height_map);
            }

            // sides
            for (i, side) in sides.into_iter().enumerate() {
                let Side {
                    perlin_sampler,
                    height_map,
                } = side;

                ris_log::trace!("generating side... {} ({})", height_map.borrow().side, i);

                let mut layer = 0;
                loop {
                    layer += 1;
                    let grid_width: i32 = 1 << layer;
                    let grid_height: i32 = grid_width;
                    let grid_weight = 1.0 / (layer as f32 * layer as f32);

                    if grid_width >= width as i32 {
                        break;
                    }

                    for iy in 0..width {
                        if iy % 100 == 0 {
                            ris_log::trace!(
                                "generating side {} ({})... progress: {}/{} layer: {}", 
                                height_map.borrow().side,
                                i,
                                iy, 
                                width,
                                layer,
                            );
                        }

                        for ix in 0..width {
                            let coord = Vec2(ix as f32 + 0.5, iy as f32 + 0.5);
                            let heigh_map_width = height_map.borrow().width as f32;
                            let size = Vec2(heigh_map_width, heigh_map_width);
                            let normalized = coord / size;
                            let grid = Vec2(grid_width as f32, grid_height as f32);
                            let p = normalized * grid;

                            // this closure connects the edges and corners of different sizes, to
                            // ensure that the perlin noise ist continuous over the whole cube
                            let apply_net = |ix: i32, iy: i32| {
                                let offset_x = perlin_sampler.offset.0 * grid_width;
                                let offset_y = perlin_sampler.offset.1 * grid_height;
                                let default_x = ix + offset_x;
                                let default_y = iy + offset_y;
                                let default = ((default_x, default_y), Mat2::identity());

                                if ix == 0 {
                                    if iy == 0 {
                                        ((default_x, default_y), Mat2::init(0.0))
                                    } else if iy == grid_height {
                                        ((default_x, default_y), Mat2::init(0.0))
                                    } else {
                                        perlin_sampler.edge0
                                            .as_ref()
                                            .map(|edge| edge(iy, (grid_width, grid_height)))
                                            .unwrap_or(default)
                                    }
                                } else if ix == grid_width {
                                    if iy == 0 {
                                        ((default_x, default_y), Mat2::init(0.0))
                                    } else if iy == grid_height {
                                        ((default_x, default_y), Mat2::init(0.0))
                                    } else {
                                        perlin_sampler.edge1
                                            .as_ref()
                                            .map(|edge| edge(iy, (grid_width, grid_height)))
                                            .unwrap_or(default)
                                    }
                                } else if iy == 0 {
                                    perlin_sampler.edge2
                                        .as_ref()
                                        .map(|edge| edge(ix, (grid_width, grid_height)))
                                        .unwrap_or(default)
                                } else if iy == grid_height {
                                    perlin_sampler.edge3
                                        .as_ref()
                                        .map(|edge| edge(ix, (grid_width, grid_height)))
                                        .unwrap_or(default)
                                } else {
                                    default
                                }
                            };

                            // perlin noise
                            let m0 = p.x().floor() as i32;
                            let m1 = m0 + 1;
                            let n0 = p.y().floor() as i32;
                            let n1 = n0 + 1;

                            let (iq0, mat0) = apply_net(m0, n0);
                            let (iq1, mat1) = apply_net(m1, n0);
                            let (iq2, mat2) = apply_net(m0, n1);
                            let (iq3, mat3) = apply_net(m1, n1);
                            let g0 = mat0 * random_gradient(iq0.0, iq0.1, seed);
                            let g1 = mat1 * random_gradient(iq1.0, iq1.1, seed);
                            let g2 = mat2 * random_gradient(iq2.0, iq2.1, seed);
                            let g3 = mat3 * random_gradient(iq3.0, iq3.1, seed);

                            let q0 = Vec2(m0 as f32, n0 as f32);
                            let q1 = Vec2(m1 as f32, n0 as f32);
                            let q2 = Vec2(m0 as f32, n1 as f32);
                            let q3 = Vec2(m1 as f32, n1 as f32);

                            let s0 = g0.dot(p - q0);
                            let s1 = g1.dot(p - q1);
                            let s2 = g2.dot(p - q2);
                            let s3 = g3.dot(p - q3);

                            let h = |x: f32| (3.0 - x * 2.0) * x * x;
                            let Vec2(x, y) = p - q0;
                            let f0 = s0 * h(1.0 - x) + s1 * h(x);
                            let f1 = s2 * h(1.0 - x) + s3 * h(x);
                            let f = f0 * h(1.0 - y) + f1 * h(y);
                            // perlin noise end

                            let mut h = height_map.borrow().get(ix, iy);
                            h.height += f * grid_weight;
                            height_map.borrow_mut().set(ix, iy, h);
                        }
                    }
                }

                height_maps.push(height_map);

                if only_generate_first_face {
                    break;
                }
            } // end sides

            // normalize and apply weight to heightmap
            ris_log::trace!("normalize and apply weight...");
            let normalize = |height_maps: &mut [RefCell<HeightMap<'_>>]| {
                let mut min = f32::MAX;
                let mut max = f32::MIN;
                for height_map in height_maps.iter() {
                    for h in height_map.borrow().values.iter() {
                        min = f32::min(min, h.height);
                        max = f32::max(max, h.height);
                    }
                }

                for height_map in height_maps.iter_mut() {
                    for h in height_map.borrow_mut().values.iter_mut() {
                        h.height = (h.height - min) / (max - min);
                    }
                }
            };

            normalize(&mut height_maps);

            for height_map in height_maps.iter_mut() {
                for h in height_map.borrow_mut().values.iter_mut() {
                    //// sigmoid
                    //let steepness = 10.0;
                    //let center = 0.5;
                    //*h = 1.0 / (1.0 + f32::exp(-steepness * (*h - center)));

                    // https://www.desmos.com/calculator/9qm31r4kfd
                    let inverse_smoothstep = 0.5 - f32::sin(f32::asin(1.0 - 2.0 * h.height) / 3.0);
                    let power = h.height * h.height;
                    let weight = 1.0 - h.height;
                    h.height = ris_math::common::mix(inverse_smoothstep, power, weight);
                }
            }

            normalize(&mut height_maps);

            // qoi
            let gradient = Gradient::try_from([
                OkLab::from(Rgb::from_hex("#00008a")?),
                OkLab::from(Rgb::from_hex("#1d90ff")?),
                OkLab::from(Rgb::from_hex("#04e100")?),
                OkLab::from(Rgb::from_hex("#ffff00")?),
                OkLab::from(Rgb::from_hex("#ff8b00")?),
                OkLab::from(Rgb::from_hex("#ff0300")?),
                OkLab::from(Rgb::from_hex("#a64020")?),
            ])?;

            for height_map in height_maps.into_iter() {
                ris_log::trace!("convert height map to bytes...");
                let mut bytes = Vec::with_capacity(height_map.borrow().values.len() * 3);

                for h in height_map.borrow().values.iter() {
                    let lab = gradient.sample(h.height);
                    let rgb = Rgb::from(lab);
                    let [r, g, b] = rgb.to_u8();
                    bytes.push(r);
                    bytes.push(g);
                    bytes.push(b);
                }

                ris_log::trace!("encoding to qoi...");
                let qoi_width = height_map.borrow().width as u32;
                let desc = QoiDesc {
                    width: qoi_width,
                    height: qoi_width,
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

                let path_string = format!("assets/in_use/terrain/height_map_{}.qoi", height_map.borrow().side);
                let filepath = PathBuf::from(path_string);

                if filepath.exists() {
                    std::fs::remove_file(&filepath)?;
                }

                let mut file = std::fs::File::create_new(filepath)?;
                let f = &mut file;
                ris_io::write(f, &qoi_bytes)?;
            } // end qoi

            ris_log::trace!("done!");

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

fn random_gradient(ix: i32, iy: i32, seed: Seed) -> Vec2 {
    let Seed(seed_value) = seed;
    let seed_a = seed_value & 0xFFFFFFFF;
    let seed_b = (seed_value >> 32) & 0xFFFFFFFF;

    let w = (8 * std::mem::size_of::<u32>()) as u32;
    let s = w / 2;
    let a = (ix as u32) ^ (seed_a as u32);
    let b = (iy as u32) ^ (seed_b as u32);
    let a = a.wrapping_mul(3284157443);
    let b = b ^ ((a << s) | (a >> (w-s)));
    let b = b.wrapping_mul(1911520717);
    let a = a ^ ((b << s) | (b >> (w-s)));
    let a = a.wrapping_mul(2048419325);
    let random = a as f32 * (PI / (!(!0u32 >> 1) as f32));
    let v_x = f32::cos(random);
    let v_y = f32::sin(random);
    Vec2(v_x, v_y)
}

fn position_on_sphere(
    texture_coordinate: (usize, usize),
    width: usize,
    side: &str,
) -> Vec3 {
    let (ix, iy) = texture_coordinate;

    // normalize texture coordinates
    let x = 2.0 * (ix as f32 / width as f32) - 1.0;
    let y = 2.0 * (iy as f32 / width as f32) - 1.0;

    // get position on cube
    let v = match side {
        "l" => Vec3(
            -1.0,
            -x,
            -y,
        ),
        "b" => Vec3(
            x,
            -1.0,
            -y,
        ),
        "r" => Vec3(
            1.0,
            x,
            -y,
        ),
        "f" => Vec3(
            -x,
            1.0,
            -y,
        ),
        "u" => Vec3(
            x,
            -y,
            1.0,
        ),
        "d" => Vec3(
            x,
            y,
            -1.0,
        ),
        _ => unreachable!(),
    };

    // normalize to get position on sphere
    let Vec3(x, y, z) = v;
    let x2 = x * x;
    let y2 = y * y;
    let z2 = z * z;
    let sx = x * f32::sqrt(1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0);
    let sy = y * f32::sqrt(1.0 - x2 / 2.0 - z2 / 2.0 + x2 * z2 / 3.0);
    let sz = z * f32::sqrt(1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0);

    Vec3(sx, sy, sz)
}
