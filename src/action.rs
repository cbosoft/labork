use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    NoOp
}
