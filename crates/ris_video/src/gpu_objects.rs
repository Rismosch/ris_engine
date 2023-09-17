use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::IncompatibleVertexDefinitionError;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use vulkano::pipeline::graphics::vertex_input::VertexInputAttributeDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputBindingDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::shader::ShaderInterface;
use vulkano::DeviceSize;

use ris_math::matrix4x4::Matrix4x4;

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view: Matrix4x4,
    pub proj: Matrix4x4,
    pub view_proj: Matrix4x4,
    pub debug_x: i32,
    pub debug_y: i32,
}

#[derive(BufferContents, Vertex, Default)]
#[repr(C)]
pub struct Vertex3d {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

pub struct CustomVertexDefinition;

unsafe impl VertexDefinition for CustomVertexDefinition {
    fn definition(
        &self,
        interface: &ShaderInterface,
    ) -> Result<VertexInputState, IncompatibleVertexDefinitionError> {
        let buffer = Vertex3d::per_vertex();

        let bindings = [(
            0u32,
            VertexInputBindingDescription {
                stride: buffer.stride,
                input_rate: buffer.input_rate,
            },
        )];

        let mut attributes: Vec<(u32, VertexInputAttributeDescription)> = Vec::new();

        for (binding, element) in interface.elements().iter().enumerate() {
            let infos = match binding {
                0 => Ok(buffer.members.get("position").unwrap().clone()),
                1 => Ok(buffer.members.get("color").unwrap().clone()),
                _ => Err(IncompatibleVertexDefinitionError::MissingAttribute {
                    attribute: String::from("no name"),
                }),
            }?;

            // TODO: ShaderInterfaceEntryType does not properly support 64bit.
            //       Once it does the below logic around num_elements and num_locations
            //       might have to be updated.
            if infos.num_components() != element.ty.num_components
                || infos.num_elements != element.ty.num_elements
            {
                return Err(IncompatibleVertexDefinitionError::FormatMismatch {
                    attribute: String::from("no name"),
                    shader: element.ty,
                    definition: infos,
                });
            }

            let mut offset = infos.offset as DeviceSize;
            let block_size = infos.format.block_size().unwrap();
            // Double precision formats can exceed a single location.
            // R64B64G64A64_SFLOAT requires two locations, so we need to adapt how we bind
            let location_range = if block_size > 16 {
                (element.location..element.location + 2 * element.ty.num_elements).step_by(2)
            } else {
                (element.location..element.location + element.ty.num_elements).step_by(1)
            };

            for location in location_range {
                attributes.push((
                    location,
                    VertexInputAttributeDescription {
                        binding: 0u32,
                        format: infos.format,
                        offset: offset as u32,
                    },
                ));
                offset += block_size;
            }
        }

        Ok(VertexInputState::new()
            .bindings(bindings)
            .attributes(attributes))
    }
}
