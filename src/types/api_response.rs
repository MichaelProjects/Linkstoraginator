use rocket::response::Responder;
use serde::Serialize;

#[derive(Debug, Serialize, Responder)]
pub struct ApiResponse {
    pub title: String,
}
