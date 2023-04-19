// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_network_acl_entry_input_input(
    input: &crate::operation::delete_network_acl_entry::DeleteNetworkAclEntryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteNetworkAclEntry", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Egress");
    if let Some(var_4) = &input.egress {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NetworkAclId");
    if let Some(var_6) = &input.network_acl_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("RuleNumber");
    if let Some(var_8) = &input.rule_number {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
