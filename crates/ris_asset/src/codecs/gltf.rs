// glTF implemented in Rust
// original spec: https://www.rfc-editor.org/rfc/rfc8259

use ris_error::prelude::*;

use super::json::JsonNumber;
use super::json::JsonObject;
use super::json::JsonValue;

#[derive(Debug, Clone)]
pub struct Accessor {
    pub buffer_view: Option<usize>,
    pub byte_offset: usize,
    pub component_type: AccessorComponentType,
    pub normalized: bool,
    pub count: usize,
    pub accessor_type: AccessorType,
    pub max: Option<JsonNumber>,
    pub min: Option<JsonNumber>,
    pub sparse: Option<AccessorSparse>,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub enum AccessorComponentType {
    I8,
    U8,
    I16,
    U16,
    U32,
    F32,
}

#[derive(Debug, Clone)]
pub struct AccessorSparse {
    pub count: usize,
    pub indices: AccessorSparseIndices,
    pub values: AccessorSparseValues,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct AccessorSparseIndices {
    pub buffer_view: usize,
    pub byte_offset: usize,
    pub component_type: AccessorSparseIndicesComponentType,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub enum AccessorSparseIndicesComponentType {
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone)]
pub struct AccessorSparseValues {
    pub buffer_view: usize,
    pub byte_offset: usize,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[derive(Debug, Clone)]
pub struct Animation {
    // todo
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub copyright: Option<String>,
    pub generator: Option<String>,
    pub version: String,
    pub min_version: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub uri: Option<String>,
    pub byte_length: usize,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct BufferView {
    pub buffer: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub byte_stride: Option<usize>,
    pub target: Option<BufferViewTarget>,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub enum BufferViewTarget {
    ArrayBuffer,
    ElementArrayBuffer,
}

#[derive(Debug, Clone)]
pub struct Camera {
    // todo
}

#[derive(Debug, Clone)]
pub struct Gltf {
    pub extensions_used: Vec<String>,
    pub extensions_required: Vec<String>,
    pub accessors: Vec<Accessor>,
    pub animations: Vec<Animation>,
    pub asset: Asset,
    pub buffers: Vec<Buffer>,
    pub buffer_views: Vec<BufferView>,
    pub cameras: Vec<Camera>,
    pub images: Vec<Image>,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub samplers: Vec<Sampler>,
    pub scene: Option<usize>,
    pub scenes: Vec<Scene>,
    pub skins: Vec<Skin>,
    pub textures: Vec<Texture>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct Image {
    // todo
}

#[derive(Debug, Clone)]
pub struct Material {
    // todo
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub primitives: Vec<MeshPrimitive>,
    pub weights: Vec<JsonNumber>,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct MeshPrimitive {
    //todo
}

#[derive(Debug, Clone)]
pub struct Node {
    pub camera: Option<usize>,
    pub children: Vec<usize>,
    pub skin: Option<usize>,
    pub transform: NodeTransform,
    pub mesh: Option<usize>,
    pub weights: Vec<f32>,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

/// gltf coordinate system is 
///  x => left
///  y => up
///  z => forward
#[derive(Debug, Clone)]
pub enum NodeTransform {
    Matrix([f32; 16]),
    TRS{
        translation: [f32; 3],
        rotation: [f32; 4],
        scale: [f32; 3],
    }
}

#[derive(Debug, Clone)]
pub struct Sampler {
    // todo
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub nodes: Vec<usize>,
    pub name: Option<String>,
    pub extensions: Option<JsonObject>,
    pub extras: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct Skin {
    // todo
}

#[derive(Debug, Clone)]
pub struct Texture {
    // todo
}

impl Gltf {
    pub fn deserialize(json: impl AsRef<str>) -> RisResult<Self> {
        let json = json.as_ref();

        let json_value = JsonValue::deserialize(json)?;
        let JsonValue::Object(json_gltf) = json_value else {
            return ris_error::new_result!("gltf json is not an object");
        };

        // asset
        let json_asset = json_gltf.get::<&JsonObject>("asset").into_ris_error()?;

        let version_string = json_asset.get::<String>("version").into_ris_error()?;
        ris_error::assert!(version_string == "2.0")?;
        let copyright = json_asset.get::<String>("copyright");
        let generator = json_asset.get::<String>("generator");
        let min_version_string = json_asset.get::<String>("minVersion");
        let extensions = json_asset.get::<&JsonObject>("extensions").cloned();
        let extras = json_asset.get::<&JsonValue>("extras").cloned();

        if let Some(min_version_string) = min_version_string.as_ref() {
            let version = version_string.split('.').collect::<Vec<_>>();
            let min_version = min_version_string.split('.').collect::<Vec<_>>();
            ris_error::assert!(version.len() == 2)?;
            ris_error::assert!(min_version.len() == 2)?;
            let major_version = version[0].parse::<usize>()?;
            let minor_version = version[1].parse::<usize>()?;
            let major_min_version = min_version[0].parse::<usize>()?;
            let minor_min_version = min_version[1].parse::<usize>()?;

            let is_greater_case_1 = major_version < major_min_version;
            let is_greater_case_2 = major_version == major_min_version && minor_version < minor_min_version; 
            if is_greater_case_1 || is_greater_case_2 {
                return ris_error::new_result!(
                    "minVersion {:?} may not be greater than version {:?}",
                    min_version_string,
                    version_string,
                );
            }

            ris_error::assert!(version == *min_version)?;
        }

        let asset = Asset {
            copyright,
            generator,
            version: version_string,
            min_version: min_version_string,
            extensions,
            extras,
        };

        // nodes
        let json_nodes = json_gltf.get::<Vec<&JsonObject>>("nodes")
            .unwrap_or(Vec::with_capacity(0));
        let mut nodes = Vec::with_capacity(json_nodes.len());
        for json_node in json_nodes {
            let camera = json_node.get::<usize>("camera");
            let children = json_node.get::<Vec<usize>>("children")
                .unwrap_or(Vec::with_capacity(0));
            let skin = json_node.get::<usize>("skin");
            let matrix = json_node.get::<Vec<f32>>("matrix");
            let mesh = json_node.get::<usize>("mesh");
            let rotation = json_node.get::<Vec<f32>>("rotation");
            let scale = json_node.get::<Vec<f32>>("scale");
            let translation = json_node.get::<Vec<f32>>("translation");
            let weights = json_node.get::<Vec<f32>>("weights")
                .unwrap_or(Vec::with_capacity(0));
            let name = json_node.get::<String>("name");
            let extensions = json_node.get::<&JsonObject>("extensions").cloned();
            let extras = json_node.get::<&JsonValue>("extras").cloned();


            let transform = match (matrix, translation, rotation, scale) {
                (Some(m), None, None, None) => {
                    let mut matrix = [
                        1.0,0.0,0.0,0.0,
                        0.0,1.0,0.0,0.0,
                        0.0,0.0,1.0,0.0,
                        0.0,0.0,0.0,1.0,
                    ];

                    if let Ok(m) = m.try_into() {
                        matrix = m;
                    }

                    NodeTransform::Matrix(matrix)
                },
                (None, t, r, s) => {
                    let mut translation = [0.0, 0.0, 0.0];
                    let mut rotation = [0.0, 0.0, 0.0, 1.0];
                    let mut scale = [1.0, 1.0, 1.0];

                    if let Some(t) = t {
                        if let Ok(t) = t.try_into() {
                            translation = t;
                        }
                    }
                    if let Some(r) = r {
                        if let Ok(r) = r.try_into() {
                            rotation = r;
                        }
                    }
                    if let Some(s) = s {
                        if let Ok(s) = s.try_into() {
                            scale = s;
                        }
                    }

                    NodeTransform::TRS {
                        translation,
                        rotation,
                        scale,
                    }
                },
                _ => return ris_error::new_result!("invalid transform"),
            };


            let node = Node {
                camera,
                children,
                skin,
                transform,
                mesh,
                weights,
                name,
                extensions,
                extras,
            };

            nodes.push(node);
        }

        // todo: check for circular hiearchy

        // scenes
        let json_scenes = json_gltf.get::<Vec<&JsonObject>>("scenes")
            .unwrap_or(Vec::with_capacity(0));

        let mut scenes = Vec::with_capacity(json_scenes.len());
        for json_scene in json_scenes {
            let nodes = json_scene.get::<Vec<usize>>("nodes")
                .unwrap_or(Vec::with_capacity(0));
            let name = json_scene.get::<String>("name");
            let extensions = json_scene.get::<&JsonObject>("extensions").cloned();
            let extras = json_scene.get::<&JsonValue>("extras").cloned();

            // todo: check if nodes are root nodes, i.e. they aren't found in any children of any node

            let scene = Scene {
                nodes,
                name,
                extensions,
                extras,
            };
            scenes.push(scene);
        }

        let scene = json_gltf.get::<usize>("scene");

        // buffers
        let json_buffers = json_gltf.get::<Vec<&JsonObject>>("buffers")
            .unwrap_or(Vec::with_capacity(0));

        let mut buffers = Vec::with_capacity(json_buffers.len());
        for json_buffer in json_buffers {
            let uri = json_buffer.get::<String>("uri");
            let byte_length = json_buffer.get::<usize>("byteLength").into_ris_error()?;
            ris_error::assert!(byte_length >= 1)?;
            let name = json_buffer.get::<String>("name");
            let extensions = json_buffer.get::<&JsonObject>("extensions").cloned();
            let extras = json_buffer.get::<&JsonValue>("extras").cloned();

            let buffer = Buffer {
                uri,
                byte_length,
                name,
                extensions,
                extras,
            };

            buffers.push(buffer);
        }

        // buffer views
        let json_buffer_views = json_gltf.get::<Vec<&JsonObject>>("bufferViews")
            .unwrap_or(Vec::with_capacity(0));

        let mut buffer_views = Vec::with_capacity(json_buffer_views.len());
        for json_buffer_view in json_buffer_views {
            let buffer = json_buffer_view.get::<usize>("buffer").into_ris_error()?;
            let byte_offset = json_buffer_view.get::<usize>("byteOffset").unwrap_or(0);
            let byte_length = json_buffer_view.get::<usize>("byteLength").into_ris_error()?;
            let byte_stride = json_buffer_view.get::<usize>("byteStride");
            let target = match json_buffer_view.get::<usize>("target") {
                None => None,
                Some(34962) => Some(BufferViewTarget::ArrayBuffer),
                Some(34963) => Some(BufferViewTarget::ElementArrayBuffer),
                Some(target) => return ris_error::new_result!("invalid buffer view target: {}", target),
            };
            let name = json_buffer_view.get::<String>("name");
            let extensions = json_buffer_view.get::<&JsonObject>("extensions").cloned();
            let extras = json_buffer_view.get::<&JsonValue>("extras").cloned();

            let buffer_view = BufferView {
                buffer,
                byte_offset,
                byte_length,
                byte_stride,
                target,
                name,
                extensions,
                extras,
            };
            buffer_views.push(buffer_view);
        }

        // accessors
        let json_accessors = json_gltf.get::<Vec<&JsonObject>>("accessors")
            .unwrap_or(Vec::with_capacity(0));

        let mut accessors = Vec::with_capacity(json_accessors.len());
        for json_accessor in json_accessors {
            let buffer_view = json_accessor.get::<usize>("bufferView");
            let byte_offset = json_accessor.get::<usize>("byteOffset")
                .unwrap_or(0);
            let component_type = match json_accessor.get::<usize>("componentType") {
                Some(5120) => AccessorComponentType::I8,
                Some(5121) => AccessorComponentType::U8,
                Some(5122) => AccessorComponentType::I16,
                Some(5123) => AccessorComponentType::U16,
                Some(5125) => AccessorComponentType::U32,
                Some(5126) => AccessorComponentType::F32,
                component_type => return ris_error::new_result!("invalid component type: {:?}", component_type),
            };
            let normalized = json_accessor.get::<bool>("normalized")
                .unwrap_or(false);
            let count = json_accessor.get::<usize>("count").into_ris_error()?;
            let accessor_type = match json_accessor.get::<&str>("type") {
                Some("SCALAR") => AccessorType::Scalar,
                Some("VEC2") => AccessorType::Vec2,
                Some("VEC3") => AccessorType::Vec3,
                Some("VEC4") => AccessorType::Vec4,
                Some("MAT2") => AccessorType::Mat2,
                Some("MAT3") => AccessorType::Mat3,
                Some("MAT4") => AccessorType::Mat4,
                accessor_type => return ris_error::new_result!("invalid accessor type: {:?}", accessor_type),
            };
            let max = json_accessor.get::<JsonNumber>("max");
            let min = json_accessor.get::<JsonNumber>("min");
            let sparse = if let Some(json_sparse) = json_accessor.get::<&JsonObject>("sparse") {
                let count = json_sparse.get::<usize>("count").into_ris_error()?;
                let json_indices = json_sparse.get::<&JsonObject>("indices").into_ris_error()?;
                let buffer_view = json_indices.get::<usize>("bufferView").into_ris_error()?;
                let byte_offset = json_indices.get::<usize>("bufferOffset")
                    .unwrap_or(0);
                let component_type = match json_indices.get::<usize>("componentType") {
                    Some(5121) => AccessorSparseIndicesComponentType::U8,
                    Some(5123) => AccessorSparseIndicesComponentType::U16,
                    Some(5125) => AccessorSparseIndicesComponentType::U32,
                    component_type => return ris_error::new_result!("invalid accessor spare indices component type: {:?}", component_type),
                };
                let extensions = json_indices.get::<&JsonObject>("extensions").cloned();
                let extras = json_indices.get::<&JsonValue>("extras").cloned();
                let indices = AccessorSparseIndices {
                    buffer_view,
                    byte_offset,
                    component_type,
                    extensions,
                    extras,
                };
                let json_values = json_sparse.get::<&JsonObject>("values").into_ris_error()?;
                let buffer_view = json_values.get::<usize>("bufferView").into_ris_error()?;
                let byte_offset = json_values.get::<usize>("byteOffset")
                    .unwrap_or(0);
                let extensions = json_values.get::<&JsonObject>("extensions").cloned();
                let extras = json_values.get::<&JsonValue>("extras").cloned();
                let values = AccessorSparseValues {
                    buffer_view,
                    byte_offset,
                    extensions,
                    extras,
                };
                let extensions = json_sparse.get::<&JsonObject>("extensions").cloned();
                let extras = json_sparse.get::<&JsonValue>("extras").cloned();

                // todo: validate sparse

                Some(AccessorSparse{
                    count,
                    indices,
                    values,
                    extensions,
                    extras,
                })
            } else {
                None
            };
            let name = json_accessor.get::<String>("name");
            let extensions = json_accessor.get::<&JsonObject>("extensions").cloned();
            let extras = json_accessor.get::<&JsonValue>("extras").cloned();

            let accessor = Accessor {
                buffer_view,
                byte_offset,
                component_type,
                normalized,
                count,
                accessor_type,
                max,
                min,
                sparse,
                name,
                extensions,
                extras,
            };
            accessors.push(accessor);
        }

        // todo: validate accessor
        
        // meshes
        let json_meshes = json_gltf.get::<Vec<&JsonObject>>("meshes")
            .unwrap_or(Vec::with_capacity(0));
        let mut meshes = Vec::with_capacity(json_meshes.len());
        for json_mesh in json_meshes {
            let json_primitives = json_mesh.get::<Vec<&JsonObject>>("primitives")
                .unwrap_or(Vec::with_capacity(0));
            ris_error::assert!(!json_primitives.is_empty())?;
            let mut primitives = Vec::with_capacity(json_primitives.len());
            for json_primitive in json_primitives {
                ris_log::error!("todo");
            }
            let weights = json_mesh.get::<Vec<JsonNumber>>("weights")
                .unwrap_or(Vec::with_capacity(0));
            let name = json_mesh.get::<String>("name");
            let extensions = json_mesh.get::<&JsonObject>("extensions").cloned();
            let extras = json_mesh.get::<&JsonValue>("extras").cloned();

            let mesh = Mesh {
                primitives,
                weights,
                name,
                extensions,
                extras,
            };
            ris_log::error!("mesh: {:#?}", mesh);
            meshes.push(mesh);
        }

        // construct gltf
        let extensions_used = json_gltf.get::<Vec<String>>("extensionsUsed")
            .unwrap_or(Vec::with_capacity(0));
        let extensions_required = json_gltf.get::<Vec<String>>("extensionsRequired")
            .unwrap_or(Vec::with_capacity(0));
        let extensions = json_gltf.get::<&JsonObject>("extensions").cloned();
        let extras = json_gltf.get::<&JsonValue>("extras").cloned();

        let gltf = Self {
            extensions_used,
            extensions_required,
            accessors,
            animations: Vec::new(),
            asset,
            buffers,
            buffer_views,
            cameras: Vec::new(),
            images: Vec::new(),
            materials: Vec::new(),
            meshes,
            nodes,
            samplers: Vec::new(),
            scene,
            scenes,
            skins: Vec::new(),
            textures: Vec::new(),
            extensions,
            extras,
        };

        Ok(gltf)
    }
}

