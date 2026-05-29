use super::Args;
use super::env::parse_bool;
use crate::error::{AppError, AppResult};

impl Args {
    pub fn parse<I>(mut values: I) -> AppResult<Self>
    where
        I: Iterator<Item = String>,
    {
        let _program = values.next();
        let mut args = Self::from_env();

        while let Some(arg) = values.next() {
            match arg.as_str() {
                "--nats-url" => args.nats.url = required_value(&mut values, "--nats-url")?,
                "--raw-stream" => {
                    args.nats.raw_stream = required_value(&mut values, "--raw-stream")?
                }
                "--raw-subject" => {
                    args.nats.raw_subject = required_value(&mut values, "--raw-subject")?
                }
                "--raw-consumer" => {
                    args.nats.raw_consumer = required_value(&mut values, "--raw-consumer")?
                }
                "--structured-stream" => {
                    args.nats.structured_stream =
                        required_value(&mut values, "--structured-stream")?
                }
                "--structured-packet-subject" => {
                    args.nats.structured_packet_subject =
                        required_value(&mut values, "--structured-packet-subject")?
                }
                "--context-flag-subject" => {
                    args.nats.context_flag_subject =
                        required_value(&mut values, "--context-flag-subject")?
                }
                "--health-subject" => {
                    args.nats.health_subject = required_value(&mut values, "--health-subject")?
                }
                "--ensure-output-stream" => {
                    args.nats.ensure_output_stream =
                        parse_bool(&required_value(&mut values, "--ensure-output-stream")?)?;
                }
                "--raw-s3-bucket" => args.raw_l0_store.bucket = required_value(&mut values, &arg)?,
                "--raw-s3-region" => args.raw_l0_store.region = required_value(&mut values, &arg)?,
                "--output-bucket" => {
                    args.output_store.bucket = required_value(&mut values, "--output-bucket")?
                }
                "--aws-region" => {
                    let region = required_value(&mut values, "--aws-region")?;
                    args.output_store.region = region.clone();
                    args.market_l1_store.region = region;
                }
                "--bedrock-region" => {
                    args.bedrock.region = required_value(&mut values, "--bedrock-region")?;
                }
                "--aws-profile" => {
                    let profile = required_value(&mut values, "--aws-profile")?;
                    args.output_store.profile = Some(profile.clone());
                    args.market_l1_store.profile = Some(profile.clone());
                    args.bedrock.profile = Some(profile);
                }
                "--market-l1-bucket" => {
                    args.market_l1_store.bucket = required_value(&mut values, "--market-l1-bucket")?
                }
                "--market-l1-window-ms" => {
                    args.market_l1_window_ms =
                        parse_i64(&required_value(&mut values, "--market-l1-window-ms")?)?;
                }
                "--enable-bedrock" => {
                    args.model_policy.enable_bedrock =
                        parse_bool(&required_value(&mut values, "--enable-bedrock")?)?;
                    args.bedrock.enabled = args.model_policy.enable_bedrock;
                }
                "--primary-model-id" => {
                    args.model_policy.primary_model_id =
                        required_value(&mut values, "--primary-model-id")?;
                    args.bedrock.primary_model_id = args.model_policy.primary_model_id.clone();
                }
                "--escalation-model-id" => {
                    args.model_policy.escalation_model_id =
                        required_value(&mut values, "--escalation-model-id")?;
                    args.bedrock.escalation_model_id =
                        args.model_policy.escalation_model_id.clone();
                }
                "--max-messages" => {
                    args.max_messages = Some(parse_usize(&required_value(
                        &mut values,
                        "--max-messages",
                    )?)?);
                }
                "--exit-on-idle" => {
                    args.exit_on_idle =
                        parse_bool(&required_value(&mut values, "--exit-on-idle")?)?;
                }
                "--chunk-max-records" => {
                    args.processing.chunk_max_records =
                        parse_usize(&required_value(&mut values, "--chunk-max-records")?)?;
                }
                "--help" | "-h" => return Err(AppError::config(help())),
                other => {
                    return Err(AppError::config(format!(
                        "unknown argument: {other}\n\n{}",
                        help()
                    )));
                }
            }
        }

        args.validate()?;
        Ok(args)
    }
}

fn required_value<I>(values: &mut I, name: &str) -> AppResult<String>
where
    I: Iterator<Item = String>,
{
    values
        .next()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::config(format!("{name} requires a value")))
}

fn parse_usize(value: &str) -> AppResult<usize> {
    value
        .parse::<usize>()
        .map_err(|_| AppError::config(format!("{value} must be a positive integer")))
}

fn parse_i64(value: &str) -> AppResult<i64> {
    value
        .parse::<i64>()
        .map_err(|_| AppError::config(format!("{value} must be an integer")))
}

fn help() -> String {
    "Usage: intel-structuring-app [--raw-s3-bucket BUCKET] [--output-bucket BUCKET] [--market-l1-bucket BUCKET] [--max-messages N] [--exit-on-idle true|false] [--enable-bedrock true|false] [--bedrock-region REGION]".to_owned()
}
