// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_enable_ebs_encryption_by_default_input_input(
    input: &crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "EnableEbsEncryptionByDefault", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
