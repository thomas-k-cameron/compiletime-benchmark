// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reset_instance_attribute_input_input(
    input: &crate::operation::reset_instance_attribute::ResetInstanceAttributeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ResetInstanceAttribute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Attribute");
    if let Some(var_2) = &input.attribute {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("InstanceId");
    if let Some(var_6) = &input.instance_id {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
