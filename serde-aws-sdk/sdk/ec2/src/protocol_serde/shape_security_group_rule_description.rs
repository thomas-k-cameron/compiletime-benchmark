// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_security_group_rule_description(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::SecurityGroupRuleDescription,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SecurityGroupRuleId");
    if let Some(var_2) = &input.security_group_rule_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    Ok(())
}
