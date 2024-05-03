use std::sync::Arc;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;
use warp::http::Method;
use crate::{config};
use crate::config::{Config, DatabaseType};
use crate::server::repositories::memory_repository::MemoryRepository;
use crate::server::repositories::postgres_repository::PostgresRepository;
use crate::server::repositories::repository::Repository;
use crate::shared::errors::{Error, return_error};

pub mod repositories;
mod routes;
mod services;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new() -> Self {
        Self { config: Config::new().expect("Config can't be set") }
    }
    pub async fn run(&mut self) -> Result<(), Error> {
        let log_filter = format!(
            "handle_errors={},rush={},warp={}",
            self.config.log_level, self.config.log_level, self.config.log_level
        );

        let store: Repository = self.get_repository().await?;

        let repository_filter = warp::any().map(move || store.clone());


        tracing_subscriber::fmt()
            // Use the filter we built above to determine which traces to record.
            .with_env_filter(log_filter)
            // Record an event when each span closes. This can be used to time our
            // routes' durations!
            .with_span_events(FmtSpan::CLOSE)
            .init();

        let cors = warp::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

        let get_questions = warp::get()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(warp::query())
            .and(repository_filter.clone())
            .and_then(routes::question::get_questions);

        let update_question = warp::put()
            .and(warp::path("questions"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(repository_filter.clone())
            .and(warp::body::json())
            .and_then(routes::question::update_question);

        let delete_question = warp::delete()
            .and(warp::path("questions"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(repository_filter.clone())
            .and_then(routes::question::delete_question);

        let add_question = warp::post()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(repository_filter.clone())
            .and(warp::body::json())
            .and_then(routes::question::add_question);

        let add_ai_answer = warp::post()
            .and(warp::path("questions"))
            .and(warp::path::param::<i32>())
            .and(warp::path("answer"))
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(repository_filter.clone())
            .and_then(routes::question::add_answer);

        let add_answer = warp::post()
            .and(warp::path("answers"))
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(repository_filter.clone())
            .and(warp::body::form())
            .and_then(routes::answer::add_answer);

        let registration = warp::post()
            .and(warp::path("registration"))
            .and(warp::path::end())
            .and(repository_filter.clone())
            .and(warp::body::json())
            .and_then(routes::authentication::register);

        let login = warp::post()
            .and(warp::path("login"))
            .and(warp::path::end())
            .and(repository_filter.clone())
            .and(warp::body::json())
            .and_then(routes::authentication::login);

        let routes = get_questions
            .or(update_question)
            .or(add_question)
            .or(delete_question)
            .or(add_ai_answer)
            .or(add_answer)
            .or(registration)
            .or(login)
            .with(cors)
            .with(warp::trace::request())
            .recover(return_error);

        tracing::info!("Q&A service build ID {}", env!("RUSH_VERSION"));

        warp::serve(routes).run(([0, 0, 0, 0], self.config.port)).await;

        Ok(())
    }

    async fn get_repository(&self) -> Result<Repository, Error> {
        match self.config.db_type {
            DatabaseType::Postgres => {
                let repository = Arc::new(PostgresRepository::new(&format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.config.db_user, self.config.db_password, self.config.db_host, self.config.db_port, self.config.db_name
                ))
                    .await
                    .map_err(Error::DatabaseQueryError)?);
                sqlx::migrate!()
                    .run(&repository.clone().connection)
                    .await
                    .map_err(Error::MigrationError)?;
                Ok(repository)
            }
            DatabaseType::Memory => {
                Ok(Arc::new(MemoryRepository::new()))
            }
        }
    }

}
