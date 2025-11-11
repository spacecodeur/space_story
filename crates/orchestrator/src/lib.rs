use lore::get_all_availables_lore;
use prompt::build;
use structs::{Lore, Prompt};

pub fn main() -> Vec<Lore> {
    return get_all_availables_lore();
}

pub fn get_lore_titles() -> Vec<String> {
    get_all_availables_lore()
        .into_iter()
        .map(|lore| lore.title)
        .collect()
}

pub fn get_next_prompt(choosen_action: Option<String>) -> Prompt {
    build()
}
