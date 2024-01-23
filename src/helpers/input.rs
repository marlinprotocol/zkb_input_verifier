use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// The payload for the POST request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputPayload {
    pub public_inputs: String,
    pub encrypted_secret: Option<String>,
    pub acl: Option<String>,
}

// Responder
impl Responder for InputPayload {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskPayload {
    pub ask_id: u64,
    pub public_inputs: String,
    pub encrypted_secret: Option<String>,
    pub acl: Option<String>,
}