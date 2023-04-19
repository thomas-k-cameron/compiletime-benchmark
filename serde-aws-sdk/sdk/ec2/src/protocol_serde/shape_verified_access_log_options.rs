// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_verified_access_log_options(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::VerifiedAccessLogOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("S3");
    if let Some(var_2) = &input.s3 {
        crate::protocol_serde::shape_verified_access_log_s3_destination_options::ser_verified_access_log_s3_destination_options(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CloudWatchLogs");
    if let Some(var_4) = &input.cloud_watch_logs {
        crate::protocol_serde::shape_verified_access_log_cloud_watch_logs_destination_options::ser_verified_access_log_cloud_watch_logs_destination_options(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("KinesisDataFirehose");
    if let Some(var_6) = &input.kinesis_data_firehose {
        crate::protocol_serde::shape_verified_access_log_kinesis_data_firehose_destination_options::ser_verified_access_log_kinesis_data_firehose_destination_options(scope_5, var_6)?;
    }
    Ok(())
}
