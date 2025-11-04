//! # Lore RAG
//!
//! A Retrieval-Augmented Generation (RAG) engine for narrative lore.
//!
//! This crate handles the complete pipeline from JSON lore ingestion to semantic search:
//! - Parse flexible JSON structures with hierarchical relationships
//! - Generate semantic embeddings using local BERT models (via Candle)
//! - Build HNSW vector indices for fast similarity search
//! - Retrieve contextually relevant lore with intelligent filtering
//!
//! ## Architecture
//!
//! The crate is organized into four main modules:
//! - `embeddings`: BERT-based semantic embedding generation
//! - `types`: Core data types (Item, ItemType) and query detection
//! - `parser`: Recursive JSON traversal with hierarchy tracking
//! - `retrieval`: Vector search with automatic type filtering
//!
//! ## Example Usage
//!
//! ```no_run
//! use lore_rag::{LoreEngine, LoreEngineConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the RAG engine
//! let config = LoreEngineConfig::default();
//! let mut engine = LoreEngine::new(config)?;
//!
//! // Load and index lore from JSON
//! engine.load_from_file("lore.json")?;
//!
//! // Query for relevant context
//! let context = engine.query("Who are the important characters?", 3)?;
//! println!("{}", context);
//! # Ok(())
//! # }
//! ```

mod embeddings;
mod parser;
mod retrieval;
mod types;

// Re-export public API
pub use embeddings::EmbeddingModel;
pub use types::{detect_item_type_from_query, Item, ItemType};

use hnsw_rs::prelude::*;
use serde_json::Value;
use std::fs;

/// Configuration for the Lore RAG engine
#[derive(Clone, Debug)]
pub struct LoreEngineConfig {
    /// Maximum number of connections per node in HNSW graph
    pub hnsw_max_nb_conn: usize,
    /// Maximum number of layers in HNSW graph
    pub hnsw_max_layer: usize,
    /// Construction quality parameter for HNSW
    pub hnsw_ef_construction: usize,
}

impl Default for LoreEngineConfig {
    fn default() -> Self {
        Self {
            hnsw_max_nb_conn: 16,
            hnsw_max_layer: 16,
            hnsw_ef_construction: 200,
        }
    }
}

/// Main RAG engine for lore management
///
/// This struct orchestrates the complete RAG pipeline:
/// 1. JSON parsing and item extraction
/// 2. Embedding generation
/// 3. Vector index construction
/// 4. Semantic search with filtering
pub struct LoreEngine {
    config: LoreEngineConfig,
    embedding_model: EmbeddingModel,
    items: Vec<Item>,
    index: Option<Hnsw<'static, f32, DistCosine>>,
}

impl LoreEngine {
    /// Create a new LoreEngine with the given configuration
    ///
    /// This initializes the embedding model (downloads from HuggingFace on first run)
    pub fn new(config: LoreEngineConfig) -> Result<Self, String> {
        let embedding_model = EmbeddingModel::new()
            .map_err(|e| format!("Error initializing embedding model: {}", e))?;

        Ok(Self {
            config,
            embedding_model,
            items: Vec::new(),
            index: None,
        })
    }

    /// Load lore from a JSON file and build the search index
    ///
    /// This parses the JSON, generates embeddings for all items,
    /// and constructs the HNSW vector index.
    pub fn load_from_file(&mut self, filename: &str) -> Result<(), String> {
        let data = fs::read_to_string(filename)
            .map_err(|e| format!("Error reading file '{}': {}", filename, e))?;

        self.load_from_json_str(&data)
    }

    /// Load lore from a JSON string and build the search index
    pub fn load_from_json_str(&mut self, json_str: &str) -> Result<(), String> {
        let json: Value = serde_json::from_str(json_str)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        self.load_from_json_value(&json)
    }

    /// Load lore from a serde_json::Value and build the search index
    pub fn load_from_json_value(&mut self, json: &Value) -> Result<(), String> {
        // Extract items with hierarchy
        let mut items: Vec<Item> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        parser::collect_items(&json, &mut items, &mut path, ItemType::Unknown, &self.embedding_model)?;

        if items.is_empty() {
            return Err("No items found in JSON. Ensure objects have a 'name' field.".to_string());
        }

        // Build HNSW index
        let max_elements = items.len().max(1);
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(
            self.config.hnsw_max_nb_conn,
            max_elements,
            self.config.hnsw_max_layer,
            self.config.hnsw_ef_construction,
            DistCosine {},
        );

        // Insert all embeddings
        for item in &items {
            hnsw.insert((&item.vec[..], item.id));
        }

        self.items = items;
        self.index = Some(hnsw);

        Ok(())
    }

    /// Query the lore database for relevant context
    ///
    /// Returns a formatted string with the top-k most relevant items,
    /// automatically filtered by detected query type (characters, locations, etc.)
    ///
    /// # Arguments
    /// * `query` - Natural language query
    /// * `top_k` - Number of results to return
    pub fn query(&self, query: &str, top_k: usize) -> Result<String, String> {
        let index = self.index.as_ref()
            .ok_or_else(|| "No index loaded. Call load_from_file() first.".to_string())?;

        retrieval::retrieve_context(query, index, &self.items, top_k, &self.embedding_model)
    }

    /// Get statistics about the loaded lore
    pub fn stats(&self) -> LoreStats {
        let mut type_counts = std::collections::HashMap::new();
        for item in &self.items {
            let type_str = item.item_type.as_str().to_string();
            *type_counts.entry(type_str).or_insert(0) += 1;
        }

        LoreStats {
            total_items: self.items.len(),
            type_counts,
        }
    }

    /// Get the embedding model dimension
    pub fn embedding_dimension(&self) -> usize {
        self.embedding_model.dimension()
    }

    /// Get reference to all loaded items
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

/// Statistics about loaded lore
#[derive(Debug)]
pub struct LoreStats {
    pub total_items: usize,
    pub type_counts: std::collections::HashMap<String, usize>,
}
