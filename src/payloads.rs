use serde::Deserialize;

use crate::types::Quiz;

pub type CreateQuizPayload = Quiz;

#[derive(Debug, Deserialize)]
pub struct CreateInstancePayload {}

#[derive(Debug, Deserialize)]
pub struct UpdateInstanceStatePayload {}

#[derive(Debug, Deserialize)]
pub struct PostAnswerPayload {}
