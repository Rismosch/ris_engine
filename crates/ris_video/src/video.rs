use sdl2::Sdl;
use sdl2_sys::SDL_WindowFlags;
use std::sync::Arc;
use vulkano::swapchain::AcquireError;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use ris_data::scene::Scene;
use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;

use crate::gpu_objects::UniformBufferObject;
use crate::renderer::Fence;
use crate::renderer::Renderer;

pub struct Video {
    renderer: Renderer,
    recreate_swapchain: bool,
    window_resized: bool,
    fences: Vec<Option<Arc<Fence>>>,
    previous_fence_i: u32,
}

impl Video {
    pub fn new(sdl_context: &Sdl) -> Result<Video, String> {
        let renderer = Renderer::initialize(sdl_context)?;
        let frames_in_flight = renderer.get_image_count();
        let fences: Vec<Option<Arc<Fence>>> = vec![None; frames_in_flight];

        Ok(Self {
            renderer,
            recreate_swapchain: false,
            window_resized: false,
            fences,
            previous_fence_i: 0,
        })
    }

    pub fn update(&mut self, scene: &Scene) -> Result<(), String> {
        let window_flags = self.renderer.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;

        if is_minimized {
            return Ok(());
        }

        if self.window_resized {
            self.window_resized = false;
            self.recreate_swapchain = false;
            self.renderer.recreate_viewport()?;
        }

        if self.recreate_swapchain {
            self.renderer.recreate_swapchain()?;
            self.recreate_swapchain = false;
        }

        let (image_i, suboptimal, acquire_future) = match self.renderer.acquire_swapchain_image() {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return Ok(());
            }
            Err(e) => return Err(format!("failed to acquire next image: {}", e)),
        };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        if let Some(image_fence) = &self.fences[image_i as usize] {
            image_fence
                .wait(None)
                .map_err(|e| format!("failed to wait on fence: {}", e))?;
        }

        // logic that uses the GPU resources that are currently notused (have been waited upon)
        let rotation = Quaternion{
            w: scene.camera_rotation.w,
            x: scene.camera_rotation.x,
            y: scene.camera_rotation.y,
            z: scene.camera_rotation.z,
        };
        let position = Vector3{
            x: scene.camera_position.x,
            y: scene.camera_position.y,
            z: scene.camera_position.z,
        };
        let mut view_matrix = Matrix4x4::view(rotation, position).transposed();
        //view_matrix.m00 = 0.10;
        //view_matrix.m01 = 0.15;
        //view_matrix.m02 = 0.20;
        //view_matrix.m03 = 0.25;
        //view_matrix.m10 = 0.30;
        //view_matrix.m11 = 0.35;
        //view_matrix.m12 = 0.40;
        //view_matrix.m13 = 0.45;
        //view_matrix.m20 = 0.50;
        //view_matrix.m21 = 0.55;
        //view_matrix.m22 = 0.60;
        //view_matrix.m23 = 0.65;
        //view_matrix.m30 = 0.70;
        //view_matrix.m31 = 0.75;
        //view_matrix.m32 = 0.80;
        //view_matrix.m33 = 0.85;

        //let fovy = 60. * ris_math::DEG2RAD;
        //let (w, h) = self.renderer.window.vulkan_drawable_size();
        //let aspect_ratio = w as f32 / h as f32;
        //let near = 0.01;
        //let far = 1000.;
        //let projection_matrix = Matrix4x4::perspective_projection(fovy, aspect_ratio, near, far);
        let projection_matrix = Matrix4x4::identity();

        let ubo = UniformBufferObject {
            debug_x: scene.debug_x,
            debug_y: scene.debug_y,
            view_matrix,
            projection_matrix,
        };
        self.renderer.update_uniform(image_i as usize, &ubo)?;

        let use_gpu_resources = false;
        let previous_future = match self.fences[self.previous_fence_i as usize].clone() {
            None => self.renderer.synchronize().boxed(),
            Some(fence) => {
                if use_gpu_resources {
                    fence
                        .wait(None)
                        .map_err(|e| format!("failed to wait on fence: {}", e))?;
                }
                fence.boxed()
            }
        };

        if use_gpu_resources {
            // logic that can use every GPU resource (the GPU is sleeping)
        }

        let result = self
            .renderer
            .flush_next_future(previous_future, acquire_future, image_i)?;

        self.fences[image_i as usize] = match result {
            Ok(fence) => Some(Arc::new(fence)),
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                None
            }
            Err(e) => {
                ris_log::warning!("failed to flush future: {}", e);
                None
            }
        };

        self.previous_fence_i = image_i;

        Ok(())
    }

    pub fn on_window_resize(&mut self) {
        self.window_resized = true;
    }
}
