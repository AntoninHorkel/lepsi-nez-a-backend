use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct CreateQuizPayloadQuestionAnswer {
    text: String,
    isCorrect: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct CreateQuizPayloadQuestion {
    questionText: String,
    answers: Vec<CreateQuizPayloadQuestionAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizPayload {
    name: String,
    question: Vec<CreateQuizPayloadQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInstancePayload {}

#[derive(Debug, Deserialize)]
pub struct UpdateInstanceStatePayload {}

#[derive(Debug, Deserialize)]
pub struct PostAnswerPayload {}
