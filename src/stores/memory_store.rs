use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;



use crate::types::answer::{Answer, AnswerId};
use crate::types::question::{Question, QuestionId};

const DEFAULT_FILE_PATH: &str = "../questions.json";

#[derive(Debug, Clone)]
pub struct MemoryStore {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,

    pub question_index: Arc<RwLock<i32>>,
    pub answer_index: Arc<RwLock<i32>>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
            question_index: Arc::new(RwLock::new(1)),
            answer_index: Arc::new(RwLock::new(1)),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}