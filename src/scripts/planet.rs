use ris_asset::assets::ris_mesh;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::CpuMesh;
use ris_data::ecs::script_prelude::*;
use ris_math::vector::Vec3;

#[derive(Debug)]
pub struct PlanetScript {

}

impl Default for PlanetScript {
    fn default() -> Self {
        Self{}
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
            let a = 1.0;
            let b = (1.0 + f32::sqrt(5.0)) / 2.0;
            let a = a / 2.0;
            let b = b / 2.0;

            // rectangle 1
            let v1 = Vec3(a, b, 0.0);
            let v2 = Vec3(-a, b, 0.0);
            let v3 = Vec3(-a, -b, 0.0);
            let v4 = Vec3(a, -b, 0.0);

            // rectangle 2
            let v5 = Vec3(b, 0.0, a);
            let v6 = Vec3(-b, 0.0, a);
            let v7 = Vec3(-b, 0.0, -a);
            let v8 = Vec3(b, 0.0, -a);

            // rectangle 3
            let v9 = Vec3(0.0, a, b);
            let v10 = Vec3(0.0, -a, b);
            let v11 = Vec3(0.0, -a, -b);
            let v12 = Vec3(0.0, a, -b);

            // vertices
            let sides = 20;
            let mut vertices = Vec::with_capacity(count);

            // normals
            let mut normals = Vec::with_capacity(count);

            // uvs
            let mut uvs = Vec::with_capacity(count);

            // indices
            let mut indices = Vec::with_capacity(count);
            for i in 0..count as u16 {
                indices.push(i * 3 + 1);
                indices.push(i * 3 + 1);
                indices.push(i * 3 + 1);
            }

            let prototype = MeshPrototype{
                vertices,
                normals,
                uvs,
                indices,
            };
            let cpu_mesh = CpuMesh::try_from(prototype)?;
            let bytes = ris_mesh::serialize(&cpu_mesh)?;
        }

        Ok(())
    }
}

