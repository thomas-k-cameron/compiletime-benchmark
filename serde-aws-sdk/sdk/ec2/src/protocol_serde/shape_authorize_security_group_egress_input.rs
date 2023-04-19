// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_authorize_security_group_egress_input_input(
    input: &crate::operation::authorize_security_group_egress::AuthorizeSecurityGroupEgressInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "AuthorizeSecurityGroupEgress", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("GroupId");
    if let Some(var_4) = &input.group_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("IpPermissions");
    if let Some(var_6) = &input.ip_permissions {
        let mut list_8 = scope_5.start_list(true, Some("item"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_ip_permission::ser_ip_permission(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("TagSpecification");
    if let Some(var_11) = &input.tag_specifications {
        let mut list_13 = scope_10.start_list(true, Some("item"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_14, item_12,
            )?;
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("CidrIp");
    if let Some(var_16) = &input.cidr_ip {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("FromPort");
    if let Some(var_18) = &input.from_port {
        scope_17.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("IpProtocol");
    if let Some(var_20) = &input.ip_protocol {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("ToPort");
    if let Some(var_22) = &input.to_port {
        scope_21.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("SourceSecurityGroupName");
    if let Some(var_24) = &input.source_security_group_name {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("SourceSecurityGroupOwnerId");
    if let Some(var_26) = &input.source_security_group_owner_id {
        scope_25.string(var_26);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
