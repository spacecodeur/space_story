# Story Engine - RAG-based Narrative POC

Local-first narrative engine using Retrieval-Augmented Generation. Everything runs offline after initial setup.

## Quick Start

```bash
# First run downloads ~90MB BERT model (cached afterward)
cargo run --bin story -- lore4.json
```

**Requirements:** Rust 1.70+, 8GB+ RAM, internet for first run

## What This Does

**Current (v0.3 POC):**
- Parse flexible JSON lore with hierarchy tracking
- Generate semantic embeddings (BERT via Candle)
- Fast vector search (HNSW index)
- Smart query filtering by type (characters, locations, etc.)

**Coming Next:**
- Local LLM for narrative text generation
- Session memory for story coherence
- Index persistence

## Project Structure

```
story/
├── lore-rag/       # Reusable RAG library
│   └── src/
│       ├── lib.rs          # LoreEngine API
│       ├── embeddings.rs   # BERT model
│       ├── parser.rs       # JSON → items
│       ├── types.rs        # Item types + query detection
│       └── retrieval.rs    # Vector search
├── story-cli/      # Demo CLI
│   └── src/main.rs
└── lore*.json      # Test data
```

## JSON Format

Any JSON with objects containing `"name"`:

```json
{
  "worlds": [{
    "name": "Aetheria",
    "description": "A magical realm",
    "regions": [{
      "name": "Northern Kingdom",
      "characters": [{
        "name": "King Arion",
        "description": "Wise ruler"
      }]
    }]
  }]
}
```

**Supported keys:** `worlds`, `regions`, `locations`, `characters`, `events`, `factions`

## How It Works

1. **Parse:** Extract all objects with "name" field, track hierarchy
2. **Embed:** Convert text → 384-dim vectors (BERT)
3. **Index:** Build HNSW graph for fast similarity search
4. **Query:** Detect type from keywords (multilingual) → search → filter → return top-k

## Using as Library

```rust
use lore_rag::{LoreEngine, LoreEngineConfig};

let mut engine = LoreEngine::new(LoreEngineConfig::default())?;
engine.load_from_file("lore.json")?;
let context = engine.query("Who are the wizards?", 5)?;
```

## Development

**Run tests:**
```bash
cargo run --bin story -- lore1.json  # Simple
cargo run --bin story -- lore4.json  # Full hierarchy
```

**Add new item type:**
1. Edit `ItemType` enum in lore-rag/src/types.rs
2. Update `from_key()`, `as_str()` methods
3. Add detection keywords

**Change embedding model:**
Edit `model_id` in lore-rag/src/embeddings.rs

## Technical Stack

- **ML:** Candle (pure Rust, no Python)
- **Embeddings:** all-MiniLM-L6-v2 (384 dims)
- **Vector Index:** HNSW
- **Memory:** ~90MB model + ~1KB per lore item

## Known Limitations (POC)

- Rebuilds index on every launch
- No LLM yet (outputs context strings only)
- Basic keyword query detection
- No session memory

## Roadmap

- **Phase 2:** Local LLM integration
- **Phase 3:** Session memory + index persistence
- **Phase 4:** Interactive gameplay loop

---

For AI contributors: See [CLAUDE.md](CLAUDE.md) for development context.
