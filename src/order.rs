use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub command_name: String,
    pub parameters: Vec<String>,
}
