use lore_rag::{LoreEngine, LoreEngineConfig};
use std::env;

/// Main entry point for the narrative engine
///
/// This CLI application uses the lore-rag crate to load a JSON lore file
/// and perform test queries.
fn main() -> Result<(), String> {
    // Read filename from arguments (default: lore1.json)
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).cloned().unwrap_or_else(|| "lore1.json".to_string());

    println!("Loading lore from: {}", filename);

    // Initialize the RAG engine
    let config = LoreEngineConfig::default();
    let mut engine = LoreEngine::new(config)?;

    // Load and index the lore
    engine.load_from_file(&filename)?;

    // Display statistics
    let stats = engine.stats();
    println!("Indexed {} items", stats.total_items);
    for (item_type, count) in stats.type_counts.iter() {
        println!("  {} : {}", item_type, count);
    }
    println!();

    // Example test queries (French queries to demonstrate multilingual support)
    let test_queries = vec![
        "Je suis dans la cité de l'Empire du Nord, le héros parle au roi Arion.",
        "Quels sont les personnages importants ?",
        "Décris-moi les régions du monde.",
    ];

    for (i, query) in test_queries.iter().enumerate() {
        println!("---");
        println!("Query {}: \"{}\"", i + 1, query);
        println!();

        let context = engine.query(query, 3)?;
        println!("{}", context);
    }

    Ok(())
}
