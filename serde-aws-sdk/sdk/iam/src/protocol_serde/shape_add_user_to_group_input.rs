// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_user_to_group_input_input(
    input: &crate::operation::add_user_to_group::AddUserToGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AddUserToGroup", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("GroupName");
    if let Some(var_2) = &input.group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("UserName");
    if let Some(var_4) = &input.user_name {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
