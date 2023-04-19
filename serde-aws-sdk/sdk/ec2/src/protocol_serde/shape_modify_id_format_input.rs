// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_id_format_input_input(
    input: &crate::operation::modify_id_format::ModifyIdFormatInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyIdFormat", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Resource");
    if let Some(var_2) = &input.resource {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("UseLongIds");
    if let Some(var_4) = &input.use_long_ids {
        scope_3.boolean(*var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
