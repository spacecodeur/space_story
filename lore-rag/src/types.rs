/// Type of lore element (enables filtering and organization)
#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    World,
    Region,
    Location,
    Character,
    Event,
    Faction,
    Unknown,
}

impl ItemType {
    /// Converts a JSON key to an item type
    pub fn from_key(key: &str) -> Self {
        match key {
            "worlds" => ItemType::World,
            "regions" => ItemType::Region,
            "locations" => ItemType::Location,
            "characters" => ItemType::Character,
            "events" => ItemType::Event,
            "factions" => ItemType::Faction,
            _ => ItemType::Unknown,
        }
    }

    /// Returns the textual representation of the type
    pub fn as_str(&self) -> &str {
        match self {
            ItemType::World => "World",
            ItemType::Region => "Region",
            ItemType::Location => "Location",
            ItemType::Character => "Character",
            ItemType::Event => "Event",
            ItemType::Faction => "Faction",
            ItemType::Unknown => "Unknown",
        }
    }
}

/// Detects the type of element being searched for from a text query
///
/// This function analyzes the query to identify keywords indicating
/// the type of element being searched for (characters, locations, etc.)
///
/// Supports both English and French keywords for multilingual querying.
///
/// # Arguments
/// * `query` - Query text to analyze
///
/// # Returns
/// Option containing the detected type, or None if no specific type detected
pub fn detect_item_type_from_query(query: &str) -> Option<ItemType> {
    let query_lower = query.to_lowercase();

    // Keywords for characters (English + French)
    if query_lower.contains("personnage")
        || query_lower.contains("character")
        || query_lower.contains("héros")
        || query_lower.contains("roi")
        || query_lower.contains("reine")
        || query_lower.contains("empereur")
        || query_lower.contains("sultan")
        || query_lower.contains("archimage") {
        return Some(ItemType::Character);
    }

    // Keywords for locations (English + French)
    if query_lower.contains("lieu")
        || query_lower.contains("location")
        || query_lower.contains("endroit")
        || query_lower.contains("cité")
        || query_lower.contains("ville")
        || query_lower.contains("village")
        || query_lower.contains("forteresse") {
        return Some(ItemType::Location);
    }

    // Keywords for regions (English + French)
    if query_lower.contains("région")
        || query_lower.contains("region")
        || query_lower.contains("royaume")
        || query_lower.contains("empire")
        || query_lower.contains("territoire") {
        return Some(ItemType::Region);
    }

    // Keywords for events (English + French)
    if query_lower.contains("événement")
        || query_lower.contains("event")
        || query_lower.contains("quand")
        || query_lower.contains("guerre")
        || query_lower.contains("bataille")
        || query_lower.contains("conflit")
        || query_lower.contains("histoire") {
        return Some(ItemType::Event);
    }

    // Keywords for factions (English + French)
    if query_lower.contains("faction")
        || query_lower.contains("guilde")
        || query_lower.contains("organisation")
        || query_lower.contains("ordre") {
        return Some(ItemType::Faction);
    }

    // Keywords for worlds (English + French)
    if query_lower.contains("monde")
        || query_lower.contains("world")
        || query_lower.contains("univers") {
        return Some(ItemType::World);
    }

    None
}

/// Represents a lore element with its hierarchical metadata
#[derive(Clone)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub text: String,
    pub vec: Vec<f32>,
    /// Type of element (world, character, location, etc.)
    pub item_type: ItemType,
    /// Full hierarchical path (e.g., "World1 > Northern Region > City")
    pub parent_path: String,
    /// Level in the hierarchy (0 = root, 1 = direct child, etc.)
    pub hierarchy_level: usize,
}

impl Item {
    /// Creates a new item
    pub fn new(
        id: usize,
        name: String,
        text: String,
        vec: Vec<f32>,
        item_type: ItemType,
        parent_path: String,
        hierarchy_level: usize,
    ) -> Self {
        Self {
            id,
            name,
            text,
            vec,
            item_type,
            parent_path,
            hierarchy_level,
        }
    }

    /// Displays the item with its hierarchical context
    pub fn display(&self) -> String {
        if self.parent_path.is_empty() {
            format!(
                "[{}] {} : {}",
                self.item_type.as_str(),
                self.name,
                self.text
            )
        } else {
            format!(
                "[{}] {} (in '{}') : {}",
                self.item_type.as_str(),
                self.name,
                self.parent_path,
                self.text
            )
        }
    }
}
