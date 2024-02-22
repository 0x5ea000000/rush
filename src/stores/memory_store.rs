use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::{Question, QuestionId};

const DEFAULT_FILE_PATH: &'static str = "../questions.json";

#[derive(Debug, Clone)]
pub struct MemoryStore {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}