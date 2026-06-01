use crate::error::{AppError, AppResult};

pub(super) fn required_value<I>(values: &mut I, name: &str) -> AppResult<String>
where
    I: Iterator<Item = String>,
{
    values
        .next()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::config(format!("{name} requires a value")))
}

pub(super) fn parse_usize(value: &str) -> AppResult<usize> {
    value
        .parse::<usize>()
        .map_err(|_| AppError::config(format!("{value} must be a positive integer")))
}

pub(super) fn parse_i64(value: &str) -> AppResult<i64> {
    value
        .parse::<i64>()
        .map_err(|_| AppError::config(format!("{value} must be an integer")))
}

pub(super) fn help() -> String {
    "Usage: intel-structuring-app [--raw-s3-bucket BUCKET] [--output-bucket BUCKET] [--market-l1-bucket BUCKET] [--max-messages N] [--exit-on-idle true|false] [--enable-bedrock true|false] [--bedrock-region REGION]".to_owned()
}
