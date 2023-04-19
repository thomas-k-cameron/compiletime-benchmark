// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_export_task_s3_location_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ExportTaskS3LocationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("S3Bucket");
    if let Some(var_2) = &input.s3_bucket {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("S3Prefix");
    if let Some(var_4) = &input.s3_prefix {
        scope_3.string(var_4);
    }
    Ok(())
}
