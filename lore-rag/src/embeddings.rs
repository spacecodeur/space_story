use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::{PaddingParams, Tokenizer};

/// Embedding model for generating semantic vectors from text
pub struct EmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl EmbeddingModel {
    /// Loads the all-MiniLM-L6-v2 model from HuggingFace Hub
    /// The model will be downloaded locally and cached
    pub fn new() -> Result<Self> {
        println!("Initializing embedding model...");

        // Use CPU by default (compatible with all systems)
        let device = Device::Cpu;

        // Download the model from HuggingFace Hub
        let model_id = "sentence-transformers/all-MiniLM-L6-v2";
        let revision = "main";

        println!("  Downloading {} (first time only)...", model_id);
        let repo = Repo::with_revision(model_id.to_string(), RepoType::Model, revision.to_string());
        let api = Api::new()?;
        let api = api.repo(repo);

        // Download necessary files
        let config_filename = api.get("config.json")?;
        let tokenizer_filename = api.get("tokenizer.json")?;
        let weights_filename = api.get("model.safetensors")?;

        println!("  Loading configuration...");
        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config)?;

        println!("  Loading tokenizer...");
        let mut tokenizer = Tokenizer::from_file(tokenizer_filename)
            .map_err(|e| E::msg(format!("Tokenizer error: {}", e)))?;

        // Configure padding for sequences
        if let Some(pp) = tokenizer.get_padding_mut() {
            pp.strategy = tokenizers::PaddingStrategy::BatchLongest;
        } else {
            let pp = PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                ..Default::default()
            };
            tokenizer.with_padding(Some(pp));
        }

        println!("  Loading model weights...");
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
        let model = BertModel::load(vb, &config)?;

        println!("  Embedding model loaded successfully\n");

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    /// Generates an embedding vector from text
    /// Uses mean pooling over token embeddings
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        // Tokenization
        let tokens = self.tokenizer
            .encode(text, true)
            .map_err(|e| E::msg(format!("Encoding error: {}", e)))?;

        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;

        // Forward pass through the model (3rd parameter is optional attention mask)
        // For simple text without padding, we can pass None
        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;

        // Mean pooling: average over all tokens
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;

        // L2 normalization (important for cosine similarity)
        let embeddings = self.normalize_l2(&embeddings)?;

        // Convert to Vec<f32>
        let embeddings = embeddings.squeeze(0)?.to_vec1::<f32>()?;

        Ok(embeddings)
    }

    /// L2 normalization of a tensor
    fn normalize_l2(&self, v: &Tensor) -> Result<Tensor> {
        Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
    }

    /// Returns the dimension of embeddings produced by this model
    pub fn dimension(&self) -> usize {
        384 // all-MiniLM-L6-v2 produces 384-dimensional vectors
    }
}
