

use warp::http::StatusCode;
use crate::repositories::repository::Repository;

use crate::types::account::Session;
use crate::types::answer::NewAnswer;


pub async fn add_answer(
    session: Session,
    store: Repository,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    match store.add_answer(new_answer, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
