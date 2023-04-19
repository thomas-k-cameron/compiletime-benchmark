// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_access_key_last_used_input_input(
    input: &crate::operation::get_access_key_last_used::GetAccessKeyLastUsedInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "GetAccessKeyLastUsed", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AccessKeyId");
    if let Some(var_2) = &input.access_key_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
