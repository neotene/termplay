use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterCommand {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_ct", rename_all = "snake_case")]
pub enum UserCommand {
    Register(RegisterCommand),
}
