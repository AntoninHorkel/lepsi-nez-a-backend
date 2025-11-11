use serde::Serialize;
use uuid::Uuid;

use crate::types::Quiz;

#[derive(Serialize)]
pub struct CreateQuizResponse {
    pub id: Uuid,
}

// TODO: Maybe id for each?
pub type GetAllQuizzesResponse = Vec<Quiz>;

pub type GetQuizResponse = Quiz;
