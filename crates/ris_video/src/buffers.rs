use std::sync::Arc;

use vulkano::buffer::Buffer;
use vulkano::buffer::BufferCreateInfo;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::memory::allocator::MemoryUsage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::Pipeline;

use ris_math::color;
use ris_util::error::RisError;

use crate::gpu_objects::UniformBufferObject;
use crate::gpu_objects::Vertex3d;

pub type Uniform<U> = (Subbuffer<U>, Arc<PersistentDescriptorSet>);

pub struct Buffers {
    pub vertex: Subbuffer<[Vertex3d]>,
    pub index: Subbuffer<[u32]>,
    pub uniforms: Vec<Uniform<UniformBufferObject>>,
}

impl Buffers {
    pub fn new(
        allocators: &crate::allocators::Allocators,
        uniform_buffer_count: usize,
        pipeline: &Arc<GraphicsPipeline>,
    ) -> Result<Self, RisError> {
        let size = 0.01;
        let offset = 0.02;
        let side = 128;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..side {
            for j in 0..side {
                for k in 0..side {
                    let x = i as f32 * offset;
                    let y = j as f32 * offset;
                    let z = k as f32 * offset;

                    let scale_a = 1.;
                    let scale_b = 1.;

                    let l = i as f32 / side as f32;
                    let a = scale_a * j as f32 / side as f32 - scale_a / 2.;
                    let b = scale_b * k as f32 / side as f32 - scale_b / 2.;

                    let lab = color::Lab {l, a, b};
                    let rgb = color::Rgb::from(lab);

                    let color = [
                        rgb.r,
                        rgb.g,
                        rgb.b,
                    ];

                    let v0 = Vertex3d {
                        position: [x, y, z],
                        color,
                    };
                    let v1 = Vertex3d {
                        position: [x + size, y, z],
                        color,
                    };
                    let v2 = Vertex3d {
                        position: [x, y + size, z],
                        color,
                    };
                    let v3 = Vertex3d {
                        position: [x + size, y + size, z],
                        color,
                    };
                    let v4 = Vertex3d {
                        position: [x, y, z + size],
                        color,
                    };
                    let v5 = Vertex3d {
                        position: [x + size, y, z + size],
                        color,
                    };
                    let v6 = Vertex3d {
                        position: [x, y + size, z + size],
                        color,
                    };
                    let v7 = Vertex3d {
                        position: [x + size, y + size, z + size],
                        color,
                    };

                    vertices.push(v0);
                    vertices.push(v1);
                    vertices.push(v2);
                    vertices.push(v3);
                    vertices.push(v4);
                    vertices.push(v5);
                    vertices.push(v6);
                    vertices.push(v7);

                    let max = side - 1;
                    if !rgb.is_valid()
                        //&& (i != 0 && i != max && j != 0 && j != max && k != 0 && k != max)
                    {
                        continue;
                    }

                    let index = i * 8 * side * side + j * 8 * side + k * 8;
                    // right
                    indices.push(index + 1);
                    indices.push(index + 5);
                    indices.push(index + 3);
                    indices.push(index + 7);
                    indices.push(index + 3);
                    indices.push(index + 5);

                    // left
                    indices.push(index);
                    indices.push(index + 2);
                    indices.push(index + 4);
                    indices.push(index + 6);
                    indices.push(index + 4);
                    indices.push(index + 2);

                    // front
                    indices.push(index);
                    indices.push(index + 4);
                    indices.push(index + 1);
                    indices.push(index + 5);
                    indices.push(index + 1);
                    indices.push(index + 4);
                    
                    // back
                    indices.push(index + 2);
                    indices.push(index + 3);
                    indices.push(index + 6);
                    indices.push(index + 7);
                    indices.push(index + 6);
                    indices.push(index + 3);

                    // top
                    indices.push(index + 4);
                    indices.push(index + 6);
                    indices.push(index + 5);
                    indices.push(index + 7);
                    indices.push(index + 5);
                    indices.push(index + 6);

                    // bottom
                    indices.push(index);
                    indices.push(index + 1);
                    indices.push(index + 2);
                    indices.push(index + 3);
                    indices.push(index + 2);
                    indices.push(index + 1);
                }
            }
        }

        //panic!("{:?}", indices);

        let vertex = ris_util::unroll!(
            Buffer::from_iter(
                &allocators.memory,
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                vertices,
            ),
            "failed to create vertex buffer"
        )?;

        // index
        let index = ris_util::unroll!(
            Buffer::from_iter(
                &allocators.memory,
                BufferCreateInfo {
                    usage: BufferUsage::INDEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                indices,
            ),
            "failed to create index buffer"
        )?;

        // uniform
        let mut uniforms = Vec::new();
        for _ in 0..uniform_buffer_count {
            let ubo = UniformBufferObject::default();

            let uniform_buffer = ris_util::unroll!(
                Buffer::from_data(
                    &allocators.memory,
                    BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        usage: MemoryUsage::Upload,
                        ..Default::default()
                    },
                    ubo,
                ),
                "failed to create uniform buffer"
            )?;

            let descriptor_set = ris_util::unroll!(
                PersistentDescriptorSet::new(
                    &allocators.descriptor_set,
                    ris_util::unroll_option!(
                        pipeline.layout().set_layouts().get(0),
                        "failed to get descriptor set layout"
                    )?
                    .clone(),
                    [WriteDescriptorSet::buffer(0, uniform_buffer.clone())],
                ),
                "failed to create persistent descriptor set"
            )?;

            uniforms.push((uniform_buffer, descriptor_set));
        }

        Ok(Self {
            vertex,
            index,
            uniforms,
        })
    }
}
