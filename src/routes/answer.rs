use warp::http::StatusCode;

use crate::stores::postgres_store::PostgresStore as Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;

pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    match store.add_answer(new_answer, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
