use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterResponseEvent {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_et", rename_all = "snake_case")]
pub enum Event {
    RegisterResponse(RegisterResponseEvent),
}
