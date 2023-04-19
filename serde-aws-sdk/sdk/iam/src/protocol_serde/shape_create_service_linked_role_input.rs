// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_service_linked_role_input_input(
    input: &crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "CreateServiceLinkedRole", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AWSServiceName");
    if let Some(var_2) = &input.aws_service_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("CustomSuffix");
    if let Some(var_6) = &input.custom_suffix {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}