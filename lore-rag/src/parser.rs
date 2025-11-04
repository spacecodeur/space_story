use crate::embeddings::EmbeddingModel;
use crate::types::{Item, ItemType};
use serde_json::Value;

/// Recursively traverse JSON to extract all lore elements with hierarchy tracking
///
/// This function recursively traverses the JSON structure, identifies all objects
/// with a "name" field, generates their embeddings, and constructs their
/// hierarchical context.
///
/// # Arguments
/// * `value` - JSON node to explore
/// * `out` - Vector accumulating found items
/// * `path` - Stack of parent names (to construct hierarchical path)
/// * `current_type` - Type of the element being explored
/// * `embedding_model` - Embedding model to vectorize text
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` on parsing error
pub fn collect_items(
    value: &Value,
    out: &mut Vec<Item>,
    path: &mut Vec<String>,
    current_type: ItemType,
    embedding_model: &EmbeddingModel,
) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            // If the object has a "name" field, it's a lore element
            if let Some(name_value) = map.get("name") {
                let name = name_value
                    .as_str()
                    .ok_or_else(|| format!("The 'name' field must be a string"))?;

                let desc = map
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                // Construct full text for embedding
                let text = if desc.is_empty() {
                    name.to_string()
                } else {
                    format!("{}: {}", name, desc)
                };

                // Generate embedding vector
                let vec = embedding_model
                    .embed(&text)
                    .map_err(|e| format!("Error embedding '{}': {}", name, e))?;
                let id = out.len();

                // Construct hierarchical path
                let parent_path = path.join(" > ");
                let hierarchy_level = path.len();

                out.push(Item::new(
                    id,
                    name.to_string(),
                    text,
                    vec,
                    current_type.clone(),
                    parent_path,
                    hierarchy_level,
                ));

                // Add this name to the path for children
                path.push(name.to_string());
            }

            // Explore known hierarchical keys
            let hierarchy_keys = [
                ("worlds", ItemType::World),
                ("regions", ItemType::Region),
                ("locations", ItemType::Location),
                ("characters", ItemType::Character),
                ("events", ItemType::Event),
                ("factions", ItemType::Faction),
            ];

            for (key, item_type) in hierarchy_keys {
                if let Some(child_value) = map.get(key) {
                    collect_items(child_value, out, path, item_type, embedding_model)?;
                }
            }

            // Remove name from path after exploring children
            if map.contains_key("name") {
                path.pop();
            }

            Ok(())
        }
        Value::Array(arr) => {
            for v in arr {
                collect_items(v, out, path, current_type.clone(), embedding_model)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
