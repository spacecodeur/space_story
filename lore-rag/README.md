# lore-rag

Rust RAG library for narrative lore: JSON → embeddings → vector search → context retrieval.

## Features

- Flexible JSON parsing (any object with "name" field)
- Real semantic embeddings (BERT via Candle)
- Fast HNSW vector search
- Smart type filtering (characters, locations, etc.)
- Pure Rust, offline-first

## Quick Start

```toml
[dependencies]
lore-rag = { path = "../lore-rag" }
```

```rust
use lore_rag::{LoreEngine, LoreEngineConfig};

let mut engine = LoreEngine::new(LoreEngineConfig::default())?;
engine.load_from_file("lore.json")?;
let context = engine.query("Who are the wizards?", 5)?;
println!("{}", context);
```

## JSON Format

```json
{
  "characters": [
    { "name": "King Arion", "description": "Wise ruler" }
  ],
  "locations": [
    { "name": "Northern Kingdom", "description": "Frozen lands" }
  ]
}
```

Recognized keys: `worlds`, `regions`, `locations`, `characters`, `events`, `factions`

## How It Works

1. Parse JSON recursively, extract items with hierarchy
2. Generate 384-dim embeddings (BERT)
3. Build HNSW index for fast search
4. Query with type detection (English/French keywords)
5. Return filtered top-k results

## Configuration

```rust
let config = LoreEngineConfig {
    hnsw_max_nb_conn: 16,       // More = better recall, more memory
    hnsw_max_layer: 16,
    hnsw_ef_construction: 200,  // Higher = slower build, better quality
};
```

## Technical Details

- **Model:** sentence-transformers/all-MiniLM-L6-v2 (384 dims)
- **Index:** HNSW (Hierarchical Navigable Small World)
- **Framework:** Candle (pure Rust ML)
- **Memory:** ~90MB model + ~1KB per item

## Requirements

- Rust 1.70+
- 8GB+ RAM
- Internet for first run (downloads model to cache)

## License

MIT OR Apache-2.0
