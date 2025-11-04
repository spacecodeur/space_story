use crate::embeddings::EmbeddingModel;
use crate::types::{detect_item_type_from_query, Item};
use hnsw_rs::prelude::*;

/// Retrieves relevant context from a text query
///
/// This function implements the "Retrieval" part of the RAG system:
/// 1. Generates the query embedding
/// 2. Automatically detects the type of element being searched (characters, locations, etc.)
/// 3. Performs vector search in the HNSW index
/// 4. Filters results by type if a filter was detected
/// 5. Returns the top_k most relevant results
///
/// # Arguments
/// * `query` - Query text
/// * `hnsw` - HNSW vector index
/// * `items` - List of indexed items
/// * `top_k` - Number of results to return
/// * `embedding_model` - Embedding model to vectorize the query
///
/// # Returns
/// Formatted string containing the narrative context
pub fn retrieve_context(
    query: &str,
    hnsw: &Hnsw<f32, DistCosine>,
    items: &[Item],
    top_k: usize,
    embedding_model: &EmbeddingModel,
) -> Result<String, String> {
    let qv = embedding_model
        .embed(query)
        .map_err(|e| format!("Error embedding query: {}", e))?;

    // Automatically detect the type being searched for
    let filter_type = detect_item_type_from_query(query);

    // Search with a larger buffer to allow for filtering
    let search_k = if filter_type.is_some() { top_k * 3 } else { top_k };
    let res = hnsw.search(&qv[..], search_k, 64);

    let mut context = String::new();

    // Display detected filter if applicable
    if let Some(ref item_type) = filter_type {
        context.push_str(&format!("Filter: {} only\n\n", item_type.as_str()));
    }

    if res.is_empty() {
        context.push_str("No relevant items found.\n");
    } else {
        // Filter results by type if detected
        let filtered_results: Vec<_> = res
            .iter()
            .filter_map(|neighbor| {
                items.get(neighbor.d_id).map(|item| (item, neighbor))
            })
            .filter(|(item, _)| {
                // If a filter is active, keep only items of the correct type
                if let Some(ref filter) = filter_type {
                    &item.item_type == filter
                } else {
                    true
                }
            })
            .take(top_k)
            .collect();

        if filtered_results.is_empty() {
            context.push_str("No items of the requested type found.\n");
        } else {
            for (rank, (item, neighbor)) in filtered_results.iter().enumerate() {
                let similarity = 1.0 - neighbor.distance;
                context.push_str(&format!("{}. {} (similarity: {:.3})\n", rank + 1, item.display(), similarity));
            }
        }
    }

    Ok(context)
}
