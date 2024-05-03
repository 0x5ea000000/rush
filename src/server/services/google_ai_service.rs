use std::env;

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use crate::config;

use crate::shared::errors::{APILayerError, Error};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse {
    pub error: ErrorMsg,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMsg {
    pub code: i64,
    pub message: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleAIRequest {
    contents: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAIResponse {
    pub candidates: Vec<Candidate>,
    pub prompt_feedback: PromptFeedback,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: String,
    pub index: i64,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    pub safety_ratings: Vec<SafetyRating>,
}

pub async fn get_ai_content(content: String) -> Result<String, Error> {
    // We are already checking if the ENV VARIABLE is set inside main.rs, so safe to unwrap here
    let api_key = env::var(config::GOOGLE_AI_KEY).unwrap();

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .post(
            format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",api_key),
        )
        .json(&GoogleAIRequest{
            contents: vec![Content{
                parts: vec![Part{
                    text: content.to_string(),
                }],
                role: "".to_string(),
            }],
        })
        .send()
        .await
        .map_err(Error::MiddlewareReqwestAPIError)?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(Error::ClientError(err));
        } else {
            let err = transform_error(res).await;
            return Err(Error::ServerError(err));
        }
    }

    match res.json::<GoogleAIResponse>().await {
        Ok(res) => Ok(res.candidates[0].content.parts[0].text.clone()),
        Err(e) => Err(Error::ReqwestAPIError(e)),
    }
}

async fn transform_error(res: reqwest::Response) -> APILayerError {
    APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().error.message,
    }
}
