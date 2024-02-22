

use async_trait::async_trait;

use crate::errors::Error;
use crate::repositories::repository::{RepositoryPort};
use crate::stores::memory_store::MemoryStore;
use crate::types::account::{Account, AccountId};
use crate::types::answer::{Answer, AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Question, QuestionId};

#[derive(Debug, Clone)]
pub struct MemoryRepository {
    store: MemoryStore,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryRepository {
    pub fn new() -> Self {
        MemoryRepository {
            store: MemoryStore::new(),
        }
    }
}

#[async_trait]
impl RepositoryPort for MemoryRepository {
    async fn get_questions(&self, _limit: Option<i32>, _offset: i32) -> Result<Vec<Question>, Error> {
        let res = self.store.questions.read().await.values().cloned().collect();
        Ok(res)
    }

    async fn get_question(&self, question_id: i32) -> Result<Question, Error> {
        match self.store.questions.read().await.get(&QuestionId(question_id)) {
            None => Err(Error::MemoryDatabaseError),
            Some(question) => Ok(question.clone())
        }
    }

    async fn is_question_owner(&self, _question_id: i32, _account_id: &AccountId) -> Result<bool, Error> {
        todo!()
    }

    async fn add_question(&self, new_question: NewQuestion, _account_id: AccountId) -> Result<Question, Error> {
        let question_id = QuestionId(*self.store.question_index.read().await);
        *self.store.question_index.write().await += 1;
        match self
            .store.questions
            .write()
            .await
            .insert(question_id, Question {
                id: question_id,
                title: new_question.title,
                content: new_question.content,
                tags: new_question.tags,
            }) {
            Some(q) => Ok(q),
            None => Err(Error::MemoryDatabaseError)
        }
    }

    async fn update_question(&self, question: Question, id: i32, _account_id: AccountId) -> Result<Question, Error> {
        match self.store.questions.write().await.get_mut(&QuestionId(id)) {
            Some(q) => *q = question.clone(),
            None => return Err(Error::MemoryDatabaseError),
        };
        Ok(question)
    }

    async fn delete_question(&self, id: i32, _account_id: AccountId) -> Result<bool, Error> {
        match self.store.questions.write().await.remove(&QuestionId(id)) {
            Some(_) => Ok(true),
            None => return Err(Error::MemoryDatabaseError),
        }
    }

    async fn add_answer(&self, new_answer: NewAnswer, _account_id: AccountId) -> Result<Answer, Error> {
        let id = AnswerId(*self.store.answer_index.read().await);
        *self.store.answer_index.write().await += 1;
        let answer = Answer {
            id,
            content: new_answer.content,
            question_id: new_answer.question_id,
        };

        match self.store.answers.write().await.insert(answer.id.clone(), answer) {
            Some(a) => Ok(a),
            None => return Err(Error::MemoryDatabaseError),
        }
    }

    async fn add_account(&self, _account: Account) -> Result<bool, Error> {
        todo!()
    }

    async fn get_account(&self, _email: String) -> Result<Account, Error> {
        todo!()
    }
}