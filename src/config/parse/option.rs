use super::super::Args;
use super::super::env::parse_bool;
use super::value::{help, parse_i64, parse_usize, required_value};
use crate::error::{AppError, AppResult};

pub(super) enum ParseAction {
    Continue,
    Help,
}

pub(super) fn apply_cli_option<I>(
    args: &mut Args,
    values: &mut I,
    arg: &str,
) -> AppResult<ParseAction>
where
    I: Iterator<Item = String>,
{
    match arg {
        "--nats-url" => args.nats.url = required_value(values, "--nats-url")?,
        "--raw-stream" => args.nats.raw_stream = required_value(values, "--raw-stream")?,
        "--raw-subject" => args.nats.raw_subject = required_value(values, "--raw-subject")?,
        "--raw-consumer" => args.nats.raw_consumer = required_value(values, "--raw-consumer")?,
        "--structured-stream" => {
            args.nats.structured_stream = required_value(values, "--structured-stream")?
        }
        "--structured-packet-subject" => {
            args.nats.structured_packet_subject =
                required_value(values, "--structured-packet-subject")?
        }
        "--context-flag-subject" => {
            args.nats.context_flag_subject = required_value(values, "--context-flag-subject")?
        }
        "--health-subject" => {
            args.nats.health_subject = required_value(values, "--health-subject")?
        }
        "--ensure-output-stream" => {
            args.nats.ensure_output_stream =
                parse_bool(&required_value(values, "--ensure-output-stream")?)?;
        }
        "--raw-s3-bucket" => args.raw_l0_store.bucket = required_value(values, arg)?,
        "--raw-s3-region" => args.raw_l0_store.region = required_value(values, arg)?,
        "--output-bucket" => args.output_store.bucket = required_value(values, "--output-bucket")?,
        "--aws-region" => apply_aws_region(args, required_value(values, "--aws-region")?),
        "--bedrock-region" => args.bedrock.region = required_value(values, "--bedrock-region")?,
        "--aws-profile" => apply_aws_profile(args, required_value(values, "--aws-profile")?),
        "--market-l1-bucket" => {
            args.market_l1_store.bucket = required_value(values, "--market-l1-bucket")?
        }
        "--market-l1-window-ms" => {
            args.market_l1_window_ms =
                parse_i64(&required_value(values, "--market-l1-window-ms")?)?;
        }
        "--enable-bedrock" => apply_enable_bedrock(
            args,
            parse_bool(&required_value(values, "--enable-bedrock")?)?,
        ),
        "--primary-model-id" => apply_primary_model_id(args, required_value(values, arg)?),
        "--escalation-model-id" => apply_escalation_model_id(args, required_value(values, arg)?),
        "--max-messages" => {
            args.max_messages = Some(parse_usize(&required_value(values, "--max-messages")?)?);
        }
        "--exit-on-idle" => {
            args.exit_on_idle = parse_bool(&required_value(values, "--exit-on-idle")?)?;
        }
        "--chunk-max-records" => {
            args.processing.chunk_max_records =
                parse_usize(&required_value(values, "--chunk-max-records")?)?;
        }
        "--help" | "-h" => return Ok(ParseAction::Help),
        other => {
            return Err(AppError::config(format!(
                "unknown argument: {other}\n\n{}",
                help()
            )));
        }
    }
    Ok(ParseAction::Continue)
}

fn apply_aws_region(args: &mut Args, region: String) {
    args.output_store.region = region.clone();
    args.market_l1_store.region = region;
}

fn apply_aws_profile(args: &mut Args, profile: String) {
    args.output_store.profile = Some(profile.clone());
    args.market_l1_store.profile = Some(profile.clone());
    args.bedrock.profile = Some(profile);
}

fn apply_enable_bedrock(args: &mut Args, enabled: bool) {
    args.model_policy.enable_bedrock = enabled;
    args.bedrock.enabled = enabled;
}

fn apply_primary_model_id(args: &mut Args, model_id: String) {
    args.model_policy.primary_model_id = model_id.clone();
    args.bedrock.primary_model_id = model_id;
}

fn apply_escalation_model_id(args: &mut Args, model_id: String) {
    args.model_policy.escalation_model_id = model_id.clone();
    args.bedrock.escalation_model_id = model_id;
}
