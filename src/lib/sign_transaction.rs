use crate::lib::types::Result;
use serde_json::{
    json,
    Value as JsonValue,
};

pub fn sign_transaction() -> Result<JsonValue> {
    Ok(json!({
        "result": "success",
    }))
}
