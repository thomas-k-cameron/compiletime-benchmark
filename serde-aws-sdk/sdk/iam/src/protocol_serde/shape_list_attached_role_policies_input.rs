// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_attached_role_policies_input_input(
    input: &crate::operation::list_attached_role_policies::ListAttachedRolePoliciesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ListAttachedRolePolicies", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RoleName");
    if let Some(var_2) = &input.role_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PathPrefix");
    if let Some(var_4) = &input.path_prefix {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Marker");
    if let Some(var_6) = &input.marker {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MaxItems");
    if let Some(var_8) = &input.max_items {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
