mod types;

use std::{env, error::Error};

use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
#[allow(unused_imports)] // Are you happy now, rust-analyzer?
use types::{request, response, sql};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?.clone()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let router = Router::new()
        .route("/quiz", routing::post(create_quiz).get(get_all_quizzes))
        .route("/quiz/{quiz_id}", routing::get(get_quiz).post(update_quiz).delete(delete_quiz))
        .route("/quiz/{quiz_id}/instance", routing::post(create_instance))
        .route("/quiz/instance/{instance_id}", routing::get(get_instance).delete(delete_instance))
        .route("/quiz/instance/{instance_id}/state", routing::post(update_instance_state))
        .route("/quiz/instance/{instance_id}/answer", routing::post(post_answer))
        .with_state(pool);
    let listener = TcpListener::bind(env::var("ROUTER_URL").unwrap_or("127.0.0.1:6767".to_owned())).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

type HandlerResult<T> = Result<(StatusCode, T), (StatusCode, String)>;

async fn create_quiz(State(pool): State<PgPool>, Json(payload): Json<request::Quiz>) -> HandlerResult<String> {
    // let mut tx = pool.begin().await.map_err(internal_error)?;
    let quiz_id = sqlx::query!("INSERT INTO quizzes (name) VALUES ($1) RETURNING id", payload.name)
        .fetch_one(&pool)
        .await
        .map_err(internal_error)?
        .id;
    for question in payload.questions {
        let question_id =
            sqlx::query!("INSERT INTO questions (quiz_id, text) VALUES ($1, $2) RETURNING id", quiz_id, question.text)
                .fetch_one(&pool)
                .await
                .map_err(internal_error)?
                .id;
        for answer in question.answers {
            sqlx::query!(
                "INSERT INTO answers (question_id, text, is_correct) VALUES ($1, $2, $3)",
                question_id,
                answer.text,
                answer.isCorrect
            )
            .execute(&pool)
            .await
            .map_err(internal_error)?;
        }
    }
    Ok((StatusCode::CREATED, quiz_id.to_string()))
}

async fn get_all_quizzes(State(pool): State<PgPool>) -> HandlerResult<Json<Vec<response::Quiz>>> {
    // let mut tx = pool.begin().await.map_err(internal_error)?;
    let quizzes =
        sqlx::query_as!(sql::Quiz, "SELECT id, name FROM quizzes").fetch_all(&pool).await.map_err(internal_error)?;
    let questions = sqlx::query_as!(sql::Question, "SELECT id, quiz_id, text FROM questions")
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;
    let answers = sqlx::query_as!(sql::Answer, "SELECT id, question_id, text, is_correct FROM answers")
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;
    let mut result = Vec::new();
    for quiz in quizzes {
        let quiz_questions: Vec<response::Question> = questions
            .iter()
            .filter(|q| q.quiz_id == quiz.id)
            .map(|q| {
                let question_answers: Vec<response::Answer> = answers
                    .iter()
                    .filter(|a| a.question_id == q.id)
                    .map(|a| response::Answer {
                        id: a.id,
                        text: a.text.clone(),
                        isCorrect: a.is_correct,
                    })
                    .collect();
                response::Question {
                    id: q.id,
                    text: q.text.clone(),
                    answers: question_answers,
                }
            })
            .collect();
        result.push(response::Quiz {
            id: quiz.id,
            name: quiz.name,
            questions: quiz_questions,
        });
    }
    Ok((StatusCode::OK, Json(result)))
}

async fn get_quiz(State(pool): State<PgPool>, Path(quiz_id): Path<Uuid>) -> HandlerResult<Json<response::Quiz>> {
    // let mut tx = pool.begin().await.map_err(internal_error)?;
    let quiz = sqlx::query_as!(sql::Quiz, "SELECT id, name FROM quizzes WHERE id = $1", quiz_id)
        .fetch_optional(&pool)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Quiz not found".to_owned()))?;
    let questions =
        sqlx::query_as!(sql::Question, "SELECT id, quiz_id, text FROM questions WHERE quiz_id = $1", quiz_id)
            .fetch_all(&pool)
            .await
            .map_err(internal_error)?;
    let question_ids: Vec<Uuid> = questions.iter().map(|q| q.id).collect();
    let answers = sqlx::query_as!(
        sql::Answer,
        "SELECT id, question_id, text, is_correct FROM answers WHERE question_id = ANY($1)",
        &question_ids
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;
    let quiz_questions: Vec<response::Question> = questions
        .iter()
        .map(|q| {
            let question_answers: Vec<response::Answer> = answers
                .iter()
                .filter(|a| a.question_id == q.id)
                .map(|a| response::Answer {
                    id: a.id,
                    text: a.text.clone(),
                    isCorrect: a.is_correct,
                })
                .collect();
            response::Question {
                id: q.id,
                text: q.text.clone(),
                answers: question_answers,
            }
        })
        .collect();
    let result = response::Quiz {
        id: quiz.id,
        name: quiz.name,
        questions: quiz_questions,
    };
    Ok((StatusCode::OK, Json(result)))
}

async fn update_quiz(
    State(pool): State<PgPool>,
    Path(quiz_id): Path<Uuid>,
    Json(payload): Json<request::Quiz>,
) -> impl IntoResponse {
    // TODO
    drop(pool);
    println!("{quiz_id:#?}");
    println!("{payload:#?}");
    (StatusCode::OK, Json(()))
}

async fn delete_quiz(State(pool): State<PgPool>, Path(quiz_id): Path<Uuid>) -> HandlerResult<()> {
    let resp = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz_id)
        .execute(&pool)
        .await
        .map_err(internal_error)?
        .rows_affected();
    if resp == 0 { Err((StatusCode::NOT_FOUND, "Quiz not found".to_owned())) } else { Ok((StatusCode::OK, ())) }
}

async fn create_instance(State(pool): State<PgPool>, Path(quiz_id): Path<Uuid>) -> HandlerResult<String> {
    // TODO: Maybe `RETURNING`?
    let quiz = sqlx::query_as!(sql::Quiz, "SELECT id, name FROM quizzes WHERE id = $1", quiz_id)
        .fetch_optional(&pool)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Quiz not found".to_owned()))?;
    let instance_id =
        sqlx::query!("INSERT INTO quiz_instances (quiz_id, state) VALUES ($1, 'active') RETURNING id", quiz.id)
            .fetch_one(&pool)
            .await
            .map_err(internal_error)?
            .id;
    Ok((StatusCode::CREATED, instance_id.to_string()))
}

async fn get_instance(
    State(pool): State<PgPool>,
    Path(instance_id): Path<Uuid>,
) -> HandlerResult<Json<response::QuizInstance>> {
    let instance =
        sqlx::query_as!(sql::QuizInstance, "SELECT id, quiz_id, state FROM quiz_instances WHERE id = $1", instance_id)
            .fetch_optional(&pool)
            .await
            .map_err(internal_error)?
            .ok_or((StatusCode::NOT_FOUND, "Quiz instance not found".to_owned()))?;
    Ok((
        StatusCode::OK,
        Json(response::QuizInstance {
            quizId: instance.quiz_id,
            state: instance.state,
        }),
    ))
}

async fn delete_instance(State(pool): State<PgPool>, Path(instance_id): Path<Uuid>) -> HandlerResult<()> {
    let resp = sqlx::query!("DELETE FROM quiz_instances WHERE id = $1", instance_id)
        .execute(&pool)
        .await
        .map_err(internal_error)?
        .rows_affected();
    if resp == 0 {
        Err((StatusCode::NOT_FOUND, "Quiz instance not found".to_owned()))
    } else {
        Ok((StatusCode::OK, ()))
    }
}

async fn update_instance_state(
    State(pool): State<PgPool>,
    Path(instance_id): Path<Uuid>,
    Json(payload): Json<request::QuizInstanceState>,
) -> HandlerResult<()> {
    let state: String = payload.into();
    let resp = sqlx::query!("UPDATE quiz_instances SET state = $1 WHERE id = $2", state, instance_id)
        .execute(&pool)
        .await
        .map_err(internal_error)?
        .rows_affected();
    if resp == 0 {
        Err((StatusCode::NOT_FOUND, "Quiz instance not found".to_owned()))
    } else {
        Ok((StatusCode::OK, ()))
    }
}

async fn post_answer(
    State(pool): State<PgPool>,
    Path(instance_id): Path<Uuid>,
    Json(payload): Json<request::QuizInstanceAnswer>,
) -> HandlerResult<()> {
    let resp = sqlx::query!(
        "INSERT INTO team_answers (instance_id, question_id, team, answer_id) VALUES ($1, $2, $3, $4)",
        instance_id,
        payload.questionId,
        payload.team,
        payload.answerId
    )
    .execute(&pool)
    .await
    .map_err(internal_error)?
    .rows_affected();
    if resp == 0 {
        Err((StatusCode::NOT_FOUND, "Quiz instance not found".to_owned()))
    } else {
        Ok((StatusCode::OK, ()))
    }
}
