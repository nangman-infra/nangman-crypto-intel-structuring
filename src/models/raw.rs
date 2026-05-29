mod event;
mod pointer;

#[cfg(test)]
mod tests;

pub use event::RawIntelEvent;
pub use pointer::{RawIntelEventCreatedPointer, RawIntelEventStorageRef};

const RAW_STORAGE_KIND_AWS_S3_JSONL_RECORD: &str = "aws_s3_jsonl_record";
