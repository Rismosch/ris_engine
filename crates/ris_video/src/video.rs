use std::sync::Arc;

use sdl2::Sdl;
use sdl2_sys::SDL_WindowFlags;
use vulkano::swapchain::AcquireError;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use ris_asset::loader::scenes_loader::Material;
use ris_data::scene::Scene;
use ris_math::matrix4x4::Matrix4x4;
use ris_util::error::RisError;

use crate::gpu_objects::UniformBufferObject;
use crate::renderer::Fence;
use crate::renderer::Renderer;

struct RecreateViewport {
    reload_shaders: bool,
}

pub struct Video {
    renderer: Renderer,
    recreate_swapchain: bool,
    recreate_viewport: Option<RecreateViewport>,
    fences: Vec<Option<Arc<Fence>>>,
    previous_fence_i: u32,
}

impl Video {
    pub fn new(sdl_context: &Sdl, material: Material) -> Result<Video, RisError> {
        let renderer = Renderer::initialize(sdl_context, material)?;
        let frames_in_flight = renderer.get_image_count();
        let fences: Vec<Option<Arc<Fence>>> = vec![None; frames_in_flight];

        Ok(Self {
            renderer,
            recreate_swapchain: false,
            recreate_viewport: None,
            fences,
            previous_fence_i: 0,
        })
    }

    pub fn update(&mut self, scene: &Scene) -> Result<(), RisError> {
        let window_flags = self.renderer.window.window_flags();
        let is_minimized = (window_flags & SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32) != 0;
        if is_minimized {
            return Ok(());
        }

        if let Err(error) = self.recreate_renderer() {
            ris_log::error!("failed to rebuild renderer: {}", error);
        }

        let (image_i, suboptimal, acquire_future) = match self.renderer.acquire_swapchain_image() {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return Ok(());
            }
            Err(e) => return ris_util::result_err!("failed to acquire next image: {}", e),
        };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        if let Some(image_fence) = &self.fences[image_i as usize] {
            ris_util::unroll!(image_fence.wait(None), "failed to wait on fence")?;
        }

        // logic that uses the GPU resources that are currently notused (have been waited upon)
        let view = Matrix4x4::view(scene.camera_position, scene.camera_rotation);

        let fovy = 60. * ris_math::DEG2RAD;
        let (w, h) = self.renderer.window.vulkan_drawable_size();
        let aspect_ratio = w as f32 / h as f32;
        let near = 0.01;
        let far = 0.1;
        let proj = Matrix4x4::perspective_projection(fovy, aspect_ratio, near, far);

        let view_proj = proj * view;

        let ubo = UniformBufferObject {
            view,
            proj,
            view_proj,
        };
        self.renderer.update_uniform(image_i as usize, &ubo)?;

        let use_gpu_resources = false;
        let previous_future = match self.fences[self.previous_fence_i as usize].clone() {
            None => self.renderer.synchronize().boxed(),
            Some(fence) => {
                if use_gpu_resources {
                    ris_util::unroll!(fence.wait(None), "failed to wait on fence")?;
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

    pub fn recreate_viewport(&mut self, reload_shaders: bool) {
        self.recreate_viewport = Some(RecreateViewport { reload_shaders })
    }

    fn recreate_renderer(&mut self) -> Result<(), RisError> {
        if let Some(recreate_viewport) = &self.recreate_viewport {
            let reload_shaders = recreate_viewport.reload_shaders;

            self.recreate_viewport = None;
            self.recreate_swapchain = false;

            if reload_shaders {
                self.renderer.reload_shaders()?;
            }

            self.renderer.recreate_viewport()?;
        }

        if self.recreate_swapchain {
            self.recreate_swapchain = false;
            self.renderer.recreate_swapchain()?;
        }

        Ok(())
    }
}
