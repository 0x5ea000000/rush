use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::errors::Error;
use crate::services::google_ai_service::get_ai_content;
use crate::stores::postgres_store::PostgresStore as Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let question = Question {
            id: question.id,
            title: question.title,
            content: question.content,
            tags: question.tags,
        };
        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => Ok(warp::reply::with_status(
                format!("Question {} deleted", id),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(Error::Unauthorized))
    }
}

pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    let question = NewQuestion {
        title: new_question.title,
        content: new_question.content,
        tags: new_question.tags,
    };

    match store.add_question(question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_answer(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    let question = match store.get_question(id).await {
        Ok(question) => question,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match get_ai_content(question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
        content: content,
        question_id: QuestionId(id),
    };

    match store.add_answer(answer, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
