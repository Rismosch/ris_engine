use std::path::PathBuf;
use std::f32::consts::PI;
use std::hash::Hash;
use std::usize;

use ris_asset::assets::ris_mesh;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::CpuMesh;
use ris_asset_data::mesh::Indices;
use ris_data::ecs::script_prelude::*;
use ris_data::ecs::components::mesh_component::MeshComponent;
use ris_error::prelude::*;
use ris_math::color::Rgb;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[derive(Debug)]
pub struct PlanetScript {
    rng: Rng,
    longitude_steps: usize,
    latitude_steps: usize,
}

impl Default for PlanetScript {
    fn default() -> Self {
        let seed = ris_error::unwrap!(
           Seed::new(),
           "failed to generate seed",
        );
        let rng = Rng::new(seed);

        Self{
            rng,
            longitude_steps: 10,
            latitude_steps: 10,
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

        if ui.button("hi") {
            let start = std::time::Instant::now();
            ris_log::trace!("generate prototype...");

            // vertices
            let mut vertices = Vec::new();

            let longitude_step = 2.0 * PI / self.longitude_steps as f32;
            let latitude_step = PI / self.latitude_steps as f32;

            let mut previous_latitude = 0.0;
            for i in 1..self.latitude_steps {
                 let current_latitude = i as f32 * longitude_step;

                 let (sin0, cos0) = ris_math::fast::sincos(previous_latitude);
                 let (sin1, cos1) = ris_math::fast::sincos(current_latitude);

                 let rotation = Quat::angle_axis(longitude_step, Vec3::up());

                 let v0 = Vec3(cos0, 0.0, sin0);
                 let v1 = Vec3(cos1, 0.0, sin1);
                 let v2 = rotation.rotate(v0);
                 let v3 = rotation.rotate(v1);

                 vertices.push(v0);
                 vertices.push(v1);
                 vertices.push(v2);
                 vertices.push(v2);
                 vertices.push(v1);
                 vertices.push(v3);

                previous_latitude = current_latitude;
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
            let mut uvs = Vec::with_capacity(vertices.len());
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
                None => game_object.add_component(&state.scene)?
            };

            let mesh_component = state.scene.deref(handle)?;
            let asset_id = ris_asset_data::AssetId::Path("meshes/planet_new.ris_mesh".to_string());
            mesh_component.borrow_mut().set_asset_id(Some(asset_id));

            let total_duration = std::time::Instant::now() - start;
            let milliseconds = total_duration.as_secs_f32() * 1000.0;
            ris_log::trace!("done! duration: {}ms", milliseconds);
        }

        Ok(())
    }
}
