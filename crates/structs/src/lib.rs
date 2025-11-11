pub struct Lore {
    pub title: String,
}

pub struct Prompt {
    pub text: String,
    pub action: Action,
}

pub struct Action {
    pub question: String,
    pub choices: Vec<String>,
}
