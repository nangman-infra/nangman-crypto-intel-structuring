use crate::error::{AppError, AppResult};

pub(super) fn parse_bool(value: &str) -> AppResult<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(AppError::config(format!("{value} must be true or false"))),
    }
}

pub(super) fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

pub(super) fn env_opt(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

pub(super) fn env_bool(name: &str, default: bool) -> AppResult<bool> {
    parse_env_value(name, default, parse_bool)
}

pub(super) fn env_usize(name: &str, default: usize) -> AppResult<usize> {
    parse_env_value(name, default, parse_typed_env_value)
}

pub(super) fn env_u64(name: &str, default: u64) -> AppResult<u64> {
    parse_env_value(name, default, parse_typed_env_value)
}

pub(super) fn env_i64(name: &str, default: i64) -> AppResult<i64> {
    parse_env_value(name, default, parse_typed_env_value)
}

pub(super) fn env_i32(name: &str, default: i32) -> AppResult<i32> {
    parse_env_value(name, default, parse_typed_env_value)
}

pub(super) fn env_f32(name: &str, default: f32) -> AppResult<f32> {
    parse_env_value(name, default, parse_typed_env_value)
}

pub(super) fn env_f64(name: &str, default: f64) -> AppResult<f64> {
    parse_env_value(name, default, parse_typed_env_value)
}

fn parse_env_value<T>(
    name: &str,
    default: T,
    parser: impl FnOnce(&str) -> AppResult<T>,
) -> AppResult<T> {
    let Some(value) = env_opt(name) else {
        return Ok(default);
    };
    parser(&value).map_err(|error| AppError::config(format!("{name} invalid: {error}")))
}

fn parse_typed_env_value<T>(value: &str) -> AppResult<T>
where
    T: std::str::FromStr,
{
    value
        .parse::<T>()
        .map_err(|_| AppError::config(format!("{value} has invalid type")))
}
