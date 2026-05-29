use crate::error::{AppError, AppResult};

pub(super) fn extract_converse_text(
    output: &aws_sdk_bedrockruntime::operation::converse::ConverseOutput,
) -> AppResult<String> {
    let message = output
        .output()
        .and_then(|value| value.as_message().ok())
        .ok_or_else(|| AppError::bedrock("Bedrock Converse response did not contain a message"))?;
    let text = message
        .content()
        .iter()
        .filter_map(|block| block.as_text().ok())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n");
    if text.trim().is_empty() {
        return Err(AppError::bedrock(
            "Bedrock Converse response did not contain text content",
        ));
    }
    Ok(text)
}

pub(super) fn extract_json_object(text: &str) -> AppResult<&str> {
    let start = text
        .find('{')
        .ok_or_else(|| AppError::bedrock("model response does not contain JSON object"))?;
    let end = text
        .rfind('}')
        .ok_or_else(|| AppError::bedrock("model response does not contain JSON object end"))?;
    if end < start {
        return Err(AppError::bedrock(
            "model response JSON object bounds invalid",
        ));
    }
    Ok(&text[start..=end])
}

#[cfg(test)]
mod tests {
    use super::extract_json_object;

    #[test]
    fn extracts_json_from_wrapped_model_text() {
        assert_eq!(
            extract_json_object("```json\n{\"a\":1}\n```").unwrap(),
            "{\"a\":1}"
        );
    }
}
