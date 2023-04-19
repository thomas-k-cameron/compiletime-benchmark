// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attach_classic_link_vpc_input_input(
    input: &crate::operation::attach_classic_link_vpc::AttachClassicLinkVpcInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "AttachClassicLinkVpc", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SecurityGroupId");
    if let Some(var_4) = &input.groups {
        let mut list_6 = scope_3.start_list(true, Some("groupId"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("InstanceId");
    if let Some(var_9) = &input.instance_id {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("VpcId");
    if let Some(var_11) = &input.vpc_id {
        scope_10.string(var_11);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}