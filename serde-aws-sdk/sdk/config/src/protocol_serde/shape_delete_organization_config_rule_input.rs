// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_organization_config_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_organization_config_rule::DeleteOrganizationConfigRuleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_config_rule_name {
        object
            .key("OrganizationConfigRuleName")
            .string(var_1.as_str());
    }
    Ok(())
}
