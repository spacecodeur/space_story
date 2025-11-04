# PROJECT CONTEXT

## Vision
Local-first narrative game engine in Rust: JSON lore → RAG retrieval → LLM generation. Everything runs offline.

## Current State (v0.3 - POC)

**What Works:**
- Workspace with `lore-rag` library + `story-cli` demo
- Real embeddings (BERT via Candle, 384-dim)
- HNSW vector search with type filtering
- Flexible JSON parsing with hierarchy tracking

**Architecture:**
```
story/
├── lore-rag/       # RAG library (parser, embeddings, retrieval)
└── story-cli/      # CLI demo
```

**Next:** LLM integration for narrative generation

## Core Design

**RAG Pipeline:**
JSON → parse items → embed text → HNSW index → query → retrieve context → (future: LLM)

**Key Modules:**
- `lib.rs` - LoreEngine API
- `parser.rs` - Extract items from JSON (any object with "name" field)
- `embeddings.rs` - BERT model (all-MiniLM-L6-v2)
- `types.rs` - Item types + query detection (multilingual keywords)
- `retrieval.rs` - Vector search with filtering

**Why This Way:**
- **Workspace**: Separates library from apps, prepares for future story-gen crate
- **Candle**: Pure Rust ML, no Python, offline-first
- **Flexible JSON**: Authors create freely, no rigid schema
- **Type filtering**: Keyword detection (English/French) prevents semantic drift

## Development Guide

**Run:**
```bash
cargo run --bin story -- lore4.json
```

**Add Item Type:**
1. Edit `ItemType` enum in types.rs
2. Update `from_key()`, `as_str()`, and query detection keywords

**Change Embedding Model:**
Update `model_id` in embeddings.rs, adjust dimension if needed

**Workspace Rules:**
- RAG logic → lore-rag/ only
- App logic → story-cli/ or new crates
- Keep LoreEngine API stable

## Known Limitations (POC)
- No session memory
- No index persistence (rebuilds each run)
- No LLM yet (outputs context strings)
- Naive keyword query detection

## Future
- Add `story-gen/` crate for LLM
- Session tracking for coherence
- Index caching
- Token budget optimization
