use std::{env, future};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::prelude::*;
use warp::{http::StatusCode, Filter};

use crate::errors::Error;
use crate::store::Store;
use crate::types::account::{Account, AccountId, Session};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = match hash_password(account.password.as_bytes()) {
        Ok(hash) => hash,
        Err(e) => return Err(warp::reject::custom(Error::PasswordHashLibraryError(e))),
    };

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(Error::PasswordHashLibraryError(e))),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn verify_token(token: String) -> Result<Session, Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| Error::CannotDecryptToken)
}

fn hash_password(password: &[u8]) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password, &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, password_hash::Error> {
    match Argon2::default().verify_password(password, &PasswordHash::new(hash)?) {
        Ok(_) => Ok(true),
        Err(err) => Err(err),
    }
}

fn issue_token(account_id: AccountId) -> String {
    let key = env::var("PASETO_KEY").unwrap();

    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };

        future::ready(Ok(token))
    })
}
