pub enum UnicornQuestionType {
    MULTI_SELECT,
    SINGLE_CHOICE,
    FILL_SENTENCE
}

impl UnicornQuestionType{
    pub fn from_string(input: &String) -> UnicornQuestionType{
        match input.as_str() {
            "Označ správnou odpověď" | 
            "Vyber správnou odpověď" => {return UnicornQuestionType::SINGLE_CHOICE; }

            "Označ správné odpovědi" |
            "Najdi všechny správné odpovědi" => { return UnicornQuestionType::MULTI_SELECT; }

            "Doplň část věty" => { return UnicornQuestionType::FILL_SENTENCE; }
            _ => { panic!("unknown type {}", input)}
        }
    }

    pub fn to_string(&self) -> &str {
        match &self {
            Self::MULTI_SELECT => { return "Označ správné odpovědi"; }
            Self::SINGLE_CHOICE => { return "Vyber správnou odpověď"; }
            Self::FILL_SENTENCE => { return "Doplň část věty"; }
        }
    }
}