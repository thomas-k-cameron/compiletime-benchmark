// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_user_permissions_boundary_input_input(
    input: &crate::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteUserPermissionsBoundary", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UserName");
    if let Some(var_2) = &input.user_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
