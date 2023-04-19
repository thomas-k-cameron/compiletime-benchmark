// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_role_permissions_boundary_input_input(
    input: &crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteRolePermissionsBoundary", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RoleName");
    if let Some(var_2) = &input.role_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}