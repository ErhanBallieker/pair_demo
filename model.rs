use crate::utils::DateTimeWrapper;
use paperclip::actix::Apiv2Schema;
use redis::RedisError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct BaseReqModel<T> {
    pub payload: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct BaseResponseModel<Req, Res> {
    pub is_success: bool,
    pub response: Res,
    pub error: BaseResponseError,
    pub model_schema: ModelSchema<Req, Res>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct BaseResponseError {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ModelSchema<Req, Res> {
    pub request: Req,
    pub response: Res,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Conversation {
    pub user_id: String,
    pub conversation_id: i32,
    pub conversation_name: String,
    pub conversation_start_date: DateTimeWrapper,
    pub shared_conversation_type: Option<String>,
    pub conversation_type: String,
    pub deleted_time: Option<DateTimeWrapper>,
    pub must_known_text: String,
    pub conversation_summary: String,
    pub meet_id: Option<i32>,
    pub ingest_id: Option<i32>,
    pub share: bool,
    pub website_links: Option<Vec<String>>,
    pub youtube_link: Option<String>,
    pub github_repo_link: Option<String>,
    pub enable_pair_programming: Option<bool>,
    pub readthedocs_link: Option<String>,
    pub copy_paste_text: Option<String>,
    pub openapi_url: Option<String>,
    pub openapi_need_auth: Option<bool>,
    pub openapi_bearer_token: Option<String>,
    pub openapi_api_key: Option<String>,
    pub openapi_api_key_header: Option<String>,
    pub is_owner_creating: Option<bool>,
    pub owner_user_id: Option<String>,
    pub shared_conversation_id: Option<String>,
    pub generated_questions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationGetReqModel {
    pub user_id: String,
    pub doc_type: String,
    pub conversation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationDeleteReqModel {
    pub user_id: String,
    pub doc_type: String,
    pub conversation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationCreateReqModel {
    pub user_id: String,
    pub conversation_name: String,
    pub conversation_type: String,
    pub shared_conversation_type: Option<String>,
    pub share: bool,
    pub website_links: Option<Vec<String>>,
    pub youtube_link: Option<String>,
    pub github_repo_link: Option<String>,
    pub enable_pair_programming: Option<bool>,
    pub readthedocs_link: Option<String>,
    pub copy_paste_text: Option<String>,
    pub openapi_url: Option<String>,
    pub is_owner_creating: Option<bool>,
    pub owner_user_id: Option<String>,
    pub openapi_need_auth: Option<bool>,
    pub openapi_bearer_token: Option<String>,
    pub openapi_api_key: Option<String>,
    pub openapi_api_key_header: Option<String>,
    pub shared_conversation_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationUpdateReqModel {
    pub user_id: String,
    pub conversation_id: String,
    pub conversation_name: String,
    pub conversation_type: String,
    pub ingest_id: Option<i32>,
    pub meet_id: Option<i32>,
    pub generated_questions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationListReqModel {
    pub user_id: String,
    pub doc_type: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ConversationListAllReqModel {
    pub user_id: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Message {
    pub user_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub message_id: i32,
    pub role: String,
    pub content: String,
    pub send_date_time: DateTimeWrapper,
    pub llm: Option<String>,
    pub llm_model: Option<String>,
    pub summarize_algorithm: Option<String>,
    pub sources: Option<Vec<String>>,
    pub search_engine_results: Option<Vec<SearchEngineResult>>,
    pub wiki: Option<WikiResult>,
    pub arxiv: Option<Vec<ArxivPaper>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct CreateMessageReqModel {
    pub user_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub role: String,
    pub content: String,
    pub llm: Option<String>,
    pub llm_model: Option<String>,
    pub summarize_algorithm: Option<String>,
    pub sources: Option<Vec<String>>,
    pub search_engine_results: Option<Vec<SearchEngineResult>>,
    pub wiki: Option<WikiResult>,
    pub arxiv: Option<Vec<ArxivPaper>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ListMessageReqModel {
    pub user_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct MessageDeleteReqModel {
    pub user_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct SearchEngineResult {
    pub snippet: String,
    pub title: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct WikiResult {
    pub page: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct ArxivPaper {
    pub published: String,
    pub title: String,
    pub authors: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Ingest {
    pub user_id: String,
    pub conversation_id: i32,
    pub user_name: String,
    pub ingest_name: String,
    pub ingest_id: i32,
    pub shared: bool,
    pub add_date: DateTimeWrapper,
    pub documents: Vec<Document>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Document {
    pub document_id: Option<i32>,
    pub document_name: String,
    pub document_size: f64,
    pub summary: Option<String>,
    pub summary_prompt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct CreateIngestReqModel {
    pub user_id: String,
    pub conversation_id: i32,
    pub user_name: String,
    pub ingest_name: String,
    pub shared: bool,
    pub documents: Vec<Document>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ListIngestReqModel {
    pub user_id: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct GetIngestReqModel {
    pub user_id: String,
    pub ingest_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct UpdateIngestDocSummaryReqModel {
    pub user_id: String,
    pub ingest_id: String,
    pub document_id: i32,
    pub summary: String,
    pub summary_prompt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Meet {
    pub user_id: String,
    pub meet_id: i32,
    pub meet_name: String,
    pub start_date: DateTimeWrapper,
    pub end_date: Option<DateTimeWrapper>,
    pub total_duration: Option<i64>,
    pub meet_last_summary: Option<String>,
    pub meet_last_summarize_prompt: Option<String>,
    pub meet_summary_attempt_count: Option<i32>,
    pub meet_old_summaries: Option<Vec<MeetSummary>>,
    pub meet_full_transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct CreateMeetReqModel {
    pub user_id: String,
    pub meet_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct GetMeetReqModel {
    pub user_id: String,
    pub meet_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct UpdateMeetStopReqModel {
    pub user_id: String,
    pub meet_id: String,
    pub meet_full_transcript: String,
    pub meet_last_summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct UpdateMeetSummarizeReqModel {
    pub user_id: String,
    pub meet_id: String,
    pub meet_last_summary: Option<String>,
    pub meet_last_summarize_prompt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct DeleteMeetReqModel {
    pub user_id: String,
    pub meet_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct ListMeetReqModel {
    pub user_id: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct MeetSummary {
    pub summarize_prompt: Option<String>,
    pub meet_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct Settings {
    pub user_id: String,
    pub user_information: String,
    pub user_name: String,
    pub user_email: String,
    pub pair_name: String,
    pub pair_question_end_text: String,
    pub pair_answer_detail_level: i32,
    pub pair_humor_value: i32,
    pub pair_sarcasm_value: i32,
    pub pair_seriousness_value: i32,
    pub pair_empathy_value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct GetSettingsReqModel {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct GetUserConversationPromptInfos {
    pub user_id: String,
    pub conversation_type: String,
    pub conversation_id: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct UserConversationInfo {
    pub user_settings: Settings,
    pub user_conversation: Conversation,
    pub user_last_N_messages: Option<Vec<Message>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct UserOpenApiTool {
    pub tool_id: i32,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub shared: bool,
    pub need_auth: bool,
    pub bearer_token: Option<String>,
    pub api_key: Option<String>,
    pub api_key_header: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct CreateOpenApiToolRequestModel {
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub shared: bool,
    pub need_auth: bool,
    pub bearer_token: Option<String>,
    pub api_key: Option<String>,
    pub api_key_header: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Apiv2Schema)]
pub struct GetUserOpenApiReqModel {
    pub user_id: String,
    pub tool_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct ListUserOpenApiToolRequestModel {
    pub user_id: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct ListAllOpenApiToolRequestModel {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Apiv2Schema)]
pub struct DeleteUserOpenApiToolReqModel {
    pub user_id: String,
    pub open_api_tool_id: String,
}

#[derive(Serialize, Apiv2Schema)]
pub struct HealthCheckResponse {
    pub now: u128,
}

#[derive(Serialize, Apiv2Schema)]
pub struct AckResponse {
    pub status: &'static str,
}

#[derive(Debug)]
pub enum LLMMemoryManagerError {
    RedisError(RedisError),
    _IncrementalSummarizationError(String),
}

impl std::fmt::Display for LLMMemoryManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LLMMemoryManagerError::RedisError(e) => write!(f, "Redis error: {}", e),
            LLMMemoryManagerError::_IncrementalSummarizationError(e) => {
                write!(f, "Incremental summarization error: {}", e)
            }
        }
    }
}

impl From<RedisError> for LLMMemoryManagerError {
    fn from(err: RedisError) -> Self {
        LLMMemoryManagerError::RedisError(err)
    }
}
