use structs::{Action, Prompt};

pub fn generate() -> Prompt {
    Prompt {
        text: "La brume s’élève lentement au-dessus des marais de Drathor, dissimulant les silhouettes des ruines oubliées. Vous avancez prudemment, l’épée encore humide du sang du dernier gobelin. Dans votre sac, la gemme écarlate pulse faiblement, comme un cœur impatient. Au loin, une lumière bleue traverse le brouillard : la tour du mage déchu, peut-être… ou un piège ancien. Le vent murmure des mots que vous ne comprenez pas encore.".to_string(),
        action: Action{
            question: "Que décidez-vous de faire ?".to_string(), 
            choices: vec![
                "Vous approcher de la lumière, prêt à tout affronter.".to_string(),
                "Explorer les ruines à la recherche d’indices.".to_string(),
                "Monter un camp et attendre le lever du jour.".to_string(),
                "Rebrousser chemin vers le village pour demander de l’aide.".to_string()
            ] },
    }
}
