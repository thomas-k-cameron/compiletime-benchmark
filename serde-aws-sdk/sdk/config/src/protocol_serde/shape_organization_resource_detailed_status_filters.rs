// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_organization_resource_detailed_status_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OrganizationResourceDetailedStatusFilters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status {
        object.key("Status").string(var_2.as_str());
    }
    Ok(())
}