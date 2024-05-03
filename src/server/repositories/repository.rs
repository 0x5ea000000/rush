use std::sync::Arc;
use crate::shared::errors::Error;
use crate::types::account::{Account, AccountId};
use crate::types::answer::{Answer, NewAnswer};
use crate::types::question::{NewQuestion, Question};
use async_trait::async_trait;


pub type Repository = Arc<dyn RepositoryPort + Send + Sync>;

#[async_trait]
pub trait RepositoryPort {
    async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error>;
    async fn get_question(
        &self,
        question_id: i32,
    ) -> Result<Question, Error>;
    async fn is_question_owner(
        &self,
        question_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, Error>;
    async fn add_question(
        &self,
        new_question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, Error>;
    async fn update_question(
        &self,
        question: Question,
        id: i32,
        account_id: AccountId,
    ) -> Result<Question, Error>;
    async fn delete_question(&self, id: i32, account_id: AccountId) -> Result<bool, Error>;
    async fn add_answer(
        &self,
        new_answer: NewAnswer,
        account_id: AccountId,
    ) -> Result<Answer, Error>;
    async fn add_account(&self, account: Account) -> Result<bool, Error>;
    async fn get_account(&self, email: String) -> Result<Account, Error>;
}
