use crate::lib::{constants::TOOL_VERSION, types::Result};

pub fn get_tool_version_info() -> Result<String> {
    Ok(TOOL_VERSION.to_string())
}
