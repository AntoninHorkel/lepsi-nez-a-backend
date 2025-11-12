use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
pub enum QuizInstanceState {
    active,
    completed,
    paused,
}

impl Into<String> for QuizInstanceState {
    fn into(self) -> String {
        match self {
            Self::active => "active",
            Self::completed => "completed",
            Self::paused => "paused",
        }
        .to_owned()
    }
}

impl From<String> for QuizInstanceState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "completed" => Self::completed,
            "paused" => Self::paused,
            _ => Self::active, // TODO
        }
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

    pub type QuizInstanceState = crate::types::QuizInstanceState;

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    pub struct QuizInstanceAnswer {
        pub questionId: Uuid,
        pub answerId: Uuid,
        pub team: u32,
    }
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

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize)]
    pub struct QuizInstanceAnswer {
        pub id: Uuid,
        pub questionId: Uuid,
        pub answerId: Uuid,
        pub team: u32,
    }
}

pub mod sql {
    use sqlx::{FromRow, types::time::OffsetDateTime};
    use uuid::Uuid;

    #[derive(Debug, FromRow)]
    pub struct Quiz {
        pub id: Uuid,
        pub name: String,
    }

    #[derive(Debug, FromRow)]
    pub struct Question {
        pub id: Uuid,
        pub quiz_id: Uuid,
        pub text: String,
    }

    #[derive(Debug, FromRow)]
    pub struct Answer {
        pub id: Uuid,
        pub question_id: Uuid,
        pub text: String,
        pub is_correct: bool,
    }

    #[derive(Debug, FromRow)]
    pub struct QuizInstance {
        #[allow(unused)]
        pub id: Uuid,
        pub quiz_id: Uuid,
        pub state: crate::types::QuizInstanceState,
    }

    #[derive(Debug, FromRow)]
    pub struct QuizInstanceAnswer {
        pub id: Uuid,
        pub instance_id: Uuid,
        pub question_id: Uuid,
        pub answer_id: Uuid,
        pub team: i32,
        pub submitted_at: OffsetDateTime,
    }
}
