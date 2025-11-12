use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
pub enum QuizInstanceState {
    active,
    completed,
    paused,
}

impl From<String> for QuizInstanceState {
    fn from(value: String) -> Self {
        return match value.as_str() {
            "active" => Self::active,
            "completed" => Self::completed,
            "paused" => Self::paused,
            _ => Self::active, // TODO
        };
    }
}

pub mod request {
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Debug, Deserialize)]
    pub struct Quiz {
        pub name: String,
        pub questions: Vec<Question>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Question {
        pub text: String,
        pub answers: Vec<Answer>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    pub struct Answer {
        pub text: String,
        pub isCorrect: bool,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    pub struct QuizInstanceAnswer {
        pub team: usize,
        pub questionId: Uuid,
        pub answerId: Uuid,
    }

    pub type QuizInstanceState = crate::types::QuizInstanceState;
}

pub mod response {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Debug, Serialize)]
    pub struct Quiz {
        pub id: Uuid,
        pub name: String,
        pub questions: Vec<Question>,
    }

    #[derive(Debug, Serialize)]
    pub struct Question {
        pub id: Uuid,
        pub text: String,
        pub answers: Vec<Answer>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize)]
    pub struct Answer {
        pub id: Uuid,
        pub text: String,
        pub isCorrect: bool,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize)]
    pub struct QuizInstance {
        pub quizId: Uuid,
        pub state: crate::types::QuizInstanceState,
    }
}

pub mod sql {
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Debug, Deserialize, sqlx::FromRow)]
    pub struct Quiz {
        pub id: Uuid,
        pub name: String,
    }

    #[derive(Debug, Deserialize, sqlx::FromRow)]
    pub struct Question {
        pub id: Uuid,
        pub quiz_id: Uuid,
        pub text: String,
    }

    #[derive(Debug, Deserialize, sqlx::FromRow)]
    pub struct Answer {
        pub id: Uuid,
        pub question_id: Uuid,
        pub text: String,
        pub is_correct: bool,
    }

    #[derive(Debug, Deserialize, sqlx::FromRow)]
    pub struct QuizInstance {
        #[allow(unused)]
        pub id: Uuid,
        pub quiz_id: Uuid,
        pub state: crate::types::QuizInstanceState,
    }
}
