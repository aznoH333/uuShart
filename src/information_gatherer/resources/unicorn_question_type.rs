pub enum UnicornQuestionType {
    MULTI_SELECT,
    SINGLE_CHOICE,
    FILL_SENTENCE,
    YES_NO,
    JOIN_ANSWERS,
    ORDER_ANSWERS,
    ORDER_ALT_ANSWERS,
    FILL_IN_MULTIPLE_ANSWER,
    JOIN_MULTI_CHOICE,
}

impl UnicornQuestionType{
    pub fn from_string(input: &String) -> UnicornQuestionType{
        match input.as_str() {
            "Označ správnou odpověď" | 
            "Vyber správnou odpověď" => {return UnicornQuestionType::SINGLE_CHOICE; }

            "Označ správné odpovědi" |
            "Najdi všechny správné odpovědi" => { return UnicornQuestionType::MULTI_SELECT; }

            "Doplň část věty" => { return UnicornQuestionType::FILL_SENTENCE; }
            
            "Ano/Ne" => { return UnicornQuestionType::YES_NO; }
            "Spoj odpovědi" => { return UnicornQuestionType::JOIN_ANSWERS; }
            "Penis" => { return UnicornQuestionType::ORDER_ANSWERS; }

            "a" => { return UnicornQuestionType::ORDER_ALT_ANSWERS; }

            "b" => { return UnicornQuestionType::FILL_IN_MULTIPLE_ANSWER; }

            "c" => { return UnicornQuestionType::JOIN_MULTI_CHOICE}
            _ => { panic!("unknown type {}", input)}
        }
    }

    pub fn to_string(&self) -> &str {
        match &self {
            Self::MULTI_SELECT => { return "Označ správné odpovědi"; }
            Self::SINGLE_CHOICE => { return "Vyber správnou odpověď"; }
            Self::FILL_SENTENCE => { return "Doplň část věty"; }
            Self::YES_NO => { return "Ano/Ne"; }
            Self::JOIN_ANSWERS => { return "Spoj odpovědi"; }
            Self::ORDER_ANSWERS => { return "Penis"; }
            Self::ORDER_ALT_ANSWERS => { return "a"; }
            Self::FILL_IN_MULTIPLE_ANSWER => { return "b"; }
        }
    }
}