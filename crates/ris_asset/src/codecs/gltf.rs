// glTF implemented in Rust
// original spec: https://www.rfc-editor.org/rfc/rfc8259

use ris_error::prelude::*;

use super::json::JsonObject;
use super::json::JsonValue;

// notes
//
// spec claims all buffers must use little endian
//
// all bufferviews are used for a single purpose, which allows parallelization of reading them
//
// only vertex buffers may use stride. if not a vertex buffer, stride must not be used
//
// bin chunk is the first element of `buffers` and it must have its `buffer.uri` to be undefined.
// undefined buffers are allowed, but only the first one refers to the bin chunk

#[derive(Debug, Clone)]
pub struct Gltf {
    pub asset: Asset,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub version: String,
    pub min_version: Option<String>,
    pub generator: Option<String>,
    pub copyright: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<Node>,
}

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
pub struct Node {
    pub name: Option<String>,
    pub transform: NodeTransform,
    pub children: Vec<Node>,
    pub camera: Option<()>,
    pub skin: Option<()>,
    pub mesh: Option<()>,
    pub weights: Option<()>,
}

impl Gltf {
    pub fn deserialize(json: impl AsRef<str>, bin: &[u8]) -> RisResult<Self> {
        let json = json.as_ref();

        let json_value = JsonValue::deserialize(json)?;
        let JsonValue::Object(json) = json_value else {
            return ris_error::new_result!("gltf json is not an object");
        };

        // asset
        let asset = json.get::<&JsonObject>("asset").into_ris_error()?;

        let version = asset.get::<String>("version").into_ris_error()?;
        let min_version = asset.get::<String>("minVersion");
        let generator = asset.get::<String>("generator");
        let copyright = asset.get::<String>("copyright");

        ris_error::assert!(version == "2.0")?;
        if let Some(min_version) = min_version.as_ref() {
            ris_error::assert!(version == *min_version)?;
        }

        let asset = Asset {
            version,
            min_version,
            generator,
            copyright,
        };

        // scenes
        let json_nodes = json.get::<Vec<&JsonObject>>("nodes")
            .unwrap_or(Vec::with_capacity(0));
        let json_scenes = json.get::<Vec<&JsonObject>>("scenes")
            .unwrap_or(Vec::with_capacity(0));

        let mut scenes = Vec::with_capacity(json_scenes.len());
        for json_scene in json_scenes {
            let name = json_scene.get::<String>("name");
            let node_indices = json_scene.get::<Vec<usize>>("nodes");

            let nodes = if let Some(node_indices) = node_indices {
                let mut scene_nodes = Vec::with_capacity(node_indices.len());

                for node_index in node_indices {
                    let node = deserialize_node(
                        &json_nodes,
                        node_index,
                        node_index,
                    )?;
                    scene_nodes.push(node);
                }

                scene_nodes
            } else {
                Vec::with_capacity(0)
            };

            let scene = Scene {
                name,
                nodes,
            };
            scenes.push(scene);
        }

        // construct gltf
        let gltf = Self {
            asset,
            scenes,
        };

        ris_log::debug!("hoi: {:#?}", gltf);
        ris_error::new_result!("reached end")
    }
}

fn deserialize_node(json_nodes: &[&JsonObject], index: usize, root_index: usize) -> RisResult<Node> {
    let json_node = json_nodes.get(index).into_ris_error()?;

    let name = json_node.get::<String>("name");
    let child_indices = json_node.get::<Vec<usize>>("children")
        .unwrap_or(Vec::with_capacity(0));

    let mut children = Vec::with_capacity(child_indices.capacity());
    for child_index in child_indices {
        if child_index == root_index {
            return ris_error::new_result!("circular node hiarchy detected. failed to deserialize node");
        }

        let child = deserialize_node(json_nodes, child_index, root_index)?;
        children.push(child);
    }

    let matrix = json_node.get::<Vec<f32>>("matrix");
    let translation = json_node.get::<Vec<f32>>("translation");
    let rotation = json_node.get::<Vec<f32>>("rotation");
    let scale = json_node.get::<Vec<f32>>("scale");

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
        name,
        transform,
        children,
        camera: None,
        skin: None,
        mesh: None,
        weights: None,
    };

    Ok(node)
}
