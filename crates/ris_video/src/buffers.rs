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

use ris_util::error::RisError;

use crate::gpu_objects::UniformBufferObject;
use crate::gpu_objects::Vertex3d;

pub type Uniform<U> = (Subbuffer<U>, Arc<PersistentDescriptorSet>);

pub struct Buffers {
    pub vertex: Subbuffer<[Vertex3d]>,
    pub index: Subbuffer<[u16]>,
    pub uniforms: Vec<Uniform<UniformBufferObject>>,
}

impl Buffers {
    pub fn new(
        allocators: &crate::allocators::Allocators,
        uniform_buffer_count: usize,
        pipeline: &Arc<GraphicsPipeline>,
    ) -> Result<Self, RisError> {
        // vertex
        let red = [1.0, 0.0, 0.0];
        let green = [0.0, 1.0, 0.0];
        let blue = [0.0, 0.0, 1.0];
        let cyan = [0.0, 1.0, 1.0];
        let magenta = [1.0, 0.0, 1.0];
        let yellow = [1.0, 1.0, 0.0];

        let pos = 0.25;
        let offset = 1.;

        // cube 1
        let v1 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: magenta,
        };
        let v2 = Vertex3d {
            position: [pos, -pos, -pos],
            color: magenta,
        };
        let v3 = Vertex3d {
            position: [-pos, -pos, pos],
            color: magenta,
        };
        let v4 = Vertex3d {
            position: [pos, -pos, pos],
            color: magenta,
        };

        let v5 = Vertex3d {
            position: [-pos, pos, -pos],
            color: green,
        };
        let v6 = Vertex3d {
            position: [-pos, pos, pos],
            color: green,
        };
        let v7 = Vertex3d {
            position: [pos, pos, -pos],
            color: green,
        };
        let v8 = Vertex3d {
            position: [pos, pos, pos],
            color: green,
        };

        let v9 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: yellow,
        };
        let v10 = Vertex3d {
            position: [-pos, pos, -pos],
            color: yellow,
        };
        let v11 = Vertex3d {
            position: [pos, -pos, -pos],
            color: yellow,
        };
        let v12 = Vertex3d {
            position: [pos, pos, -pos],
            color: yellow,
        };

        let v13 = Vertex3d {
            position: [-pos, -pos, pos],
            color: blue,
        };
        let v14 = Vertex3d {
            position: [pos, -pos, pos],
            color: blue,
        };
        let v15 = Vertex3d {
            position: [-pos, pos, pos],
            color: blue,
        };
        let v16 = Vertex3d {
            position: [pos, pos, pos],
            color: blue,
        };

        let v17 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: cyan,
        };
        let v18 = Vertex3d {
            position: [-pos, -pos, pos],
            color: cyan,
        };
        let v19 = Vertex3d {
            position: [-pos, pos, -pos],
            color: cyan,
        };
        let v20 = Vertex3d {
            position: [-pos, pos, pos],
            color: cyan,
        };

        let v21 = Vertex3d {
            position: [pos, -pos, -pos],
            color: red,
        };
        let v22 = Vertex3d {
            position: [pos, pos, -pos],
            color: red,
        };
        let v23 = Vertex3d {
            position: [pos, -pos, pos],
            color: red,
        };
        let v24 = Vertex3d {
            position: [pos, pos, pos],
            color: red,
        };

        // cube 2
        let v1_2 = Vertex3d {
            position: [-pos + offset, -pos, -pos],
            color: magenta,
        };
        let v2_2 = Vertex3d {
            position: [pos + offset, -pos, -pos],
            color: magenta,
        };
        let v3_2 = Vertex3d {
            position: [-pos + offset, -pos, pos],
            color: magenta,
        };
        let v4_2 = Vertex3d {
            position: [pos + offset, -pos, pos],
            color: magenta,
        };

        let v5_2 = Vertex3d {
            position: [-pos + offset, pos, -pos],
            color: green,
        };
        let v6_2 = Vertex3d {
            position: [-pos + offset, pos, pos],
            color: green,
        };
        let v7_2 = Vertex3d {
            position: [pos + offset, pos, -pos],
            color: green,
        };
        let v8_2 = Vertex3d {
            position: [pos + offset, pos, pos],
            color: green,
        };

        let v9_2 = Vertex3d {
            position: [-pos + offset, -pos, -pos],
            color: yellow,
        };
        let v10_2 = Vertex3d {
            position: [-pos + offset, pos, -pos],
            color: yellow,
        };
        let v11_2 = Vertex3d {
            position: [pos + offset, -pos, -pos],
            color: yellow,
        };
        let v12_2 = Vertex3d {
            position: [pos + offset, pos, -pos],
            color: yellow,
        };

        let v13_2 = Vertex3d {
            position: [-pos + offset, -pos, pos],
            color: blue,
        };
        let v14_2 = Vertex3d {
            position: [pos + offset, -pos, pos],
            color: blue,
        };
        let v15_2 = Vertex3d {
            position: [-pos + offset, pos, pos],
            color: blue,
        };
        let v16_2 = Vertex3d {
            position: [pos + offset, pos, pos],
            color: blue,
        };

        let v17_2 = Vertex3d {
            position: [-pos + offset, -pos, -pos],
            color: cyan,
        };
        let v18_2 = Vertex3d {
            position: [-pos + offset, -pos, pos],
            color: cyan,
        };
        let v19_2 = Vertex3d {
            position: [-pos + offset, pos, -pos],
            color: cyan,
        };
        let v20_2 = Vertex3d {
            position: [-pos + offset, pos, pos],
            color: cyan,
        };

        let v21_2 = Vertex3d {
            position: [pos + offset, -pos, -pos],
            color: red,
        };
        let v22_2 = Vertex3d {
            position: [pos + offset, pos, -pos],
            color: red,
        };
        let v23_2 = Vertex3d {
            position: [pos + offset, -pos, pos],
            color: red,
        };
        let v24_2 = Vertex3d {
            position: [pos + offset, pos, pos],
            color: red,
        };

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
                vec![
                    // cube 1
                    v1, v2, v3, v4, // magenta
                    v5, v6, v7, v8, // green
                    v9, v10, v11, v12, // yellow
                    v13, v14, v15, v16, // blue
                    v17, v18, v19, v20, // cyan
                    v21, v22, v23, v24, // red
                    // cube 2
                    v1_2, v2_2, v3_2, v4_2, // magenta
                    v5_2, v6_2, v7_2, v8_2, // green
                    v9_2, v10_2, v11_2, v12_2, // yellow
                    v13_2, v14_2, v15_2, v16_2, // blue
                    v17_2, v18_2, v19_2, v20_2, // cyan
                    v21_2, v22_2, v23_2, v24_2, // red
                ],
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
                vec![
                    // cube 1
                    0, 1, 2, 3, 2, 1, // magenta
                    4, 5, 6, 7, 6, 5, // green
                    8, 9, 10, 11, 10, 9, // yellow
                    12, 13, 14, 15, 14, 13, // blue
                    16, 17, 18, 19, 18, 17, // cyan
                    20, 21, 22, 23, 22, 21, // red
                    // cube 2
                    24, 25, 26, 27, 26, 25, // magenta
                    28, 29, 30, 31, 30, 29, // green
                    32, 33, 34, 35, 34, 33, // yellow
                    36, 37, 38, 39, 38, 37, // blue
                    40, 41, 42, 43, 42, 41, // cyan
                    44, 45, 46, 47, 46, 45, // red
                ],
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
