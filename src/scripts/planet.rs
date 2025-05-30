use ris_asset::assets::ris_mesh;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::CpuMesh;
use ris_data::ecs::script_prelude::*;
use ris_error::prelude::*;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;

#[derive(Debug)]
pub struct PlanetScript {
    subdivisions: usize,
}

impl Default for PlanetScript {
    fn default() -> Self {
        Self{
            subdivisions: 6,
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

            // normals
            let mut normals = Vec::with_capacity(vertices.len());
            for vertices in vertices.chunks(3) {
                let v0 = *vertices.get(0).into_ris_error()?;
                let v1 = *vertices.get(1).into_ris_error()?;
                let v2 = *vertices.get(2).into_ris_error()?;

                let t1 = v1 - v0;
                let t2 = v2 - v0;
                let normal = Vec3::cross(t2, t1);

                normals.push(normal);
                normals.push(normal);
                normals.push(normal);
            }

            // uvs
            let mut uvs = Vec::with_capacity(vertices.len());
            for _ in 0..20 {
                uvs.push(Vec2(0.0, 0.0));
                uvs.push(Vec2(1.0, 0.0));
                uvs.push(Vec2(0.0, 1.0));
            }

            // indices
            let mut indices = Vec::with_capacity(vertices.len());
            for i in 0..20 {
                indices.push(i * 3 + 0);
                indices.push(i * 3 + 1);
                indices.push(i * 3 + 2);
            }

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
                    let v3 = (v0 + v1) / 2.0;
                    let v4 = (v1 + v2) / 2.0;
                    let v5 = (v0 + v2) / 2.0;

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

                // normals
                let mut new_normals = Vec::with_capacity(vertices.len());
                for vertices in vertices.chunks(3) {
                    let v0 = *vertices.get(0).into_ris_error()?;
                    let v1 = *vertices.get(1).into_ris_error()?;
                    let v2 = *vertices.get(2).into_ris_error()?;

                    let t1 = v1 - v0;
                    let t2 = v2 - v0;
                    let normal = Vec3::cross(t2, t1);

                    new_normals.push(normal);
                    new_normals.push(normal);
                    new_normals.push(normal);
                }
                normals = new_normals;

                // uvs
                let mut new_uvs = Vec::with_capacity(vertices.len());
                for _ in vertices.chunks(3) {
                    new_uvs.push(Vec2(0.0, 0.0));
                    new_uvs.push(Vec2(1.0, 0.0));
                    new_uvs.push(Vec2(0.0, 1.0));
                }
                uvs = new_uvs;
                
                // indices
                let mut new_indices = Vec::with_capacity(vertices.len());
                for (i, _) in vertices.iter().enumerate() {
                    new_indices.push(i as u16);
                }
                indices = new_indices;

                subdivision -= 1;
            }

            ris_log::trace!("generate mesh... vertices: {}", vertices.len());
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
            let filename = format!("planet_{}.ris_mesh", self.subdivisions);
            let mut file = std::fs::File::create_new(filename)?;
            let f = &mut file;
            ris_io::write(f, &bytes)?;
            ris_log::trace!("done!");

        }

        Ok(())
    }
}

