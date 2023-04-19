// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_vpn_gateway_input_input(
    input: &crate::operation::create_vpn_gateway::CreateVpnGatewayInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateVpnGateway", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AvailabilityZone");
    if let Some(var_2) = &input.availability_zone {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Type");
    if let Some(var_4) = &input.r#type {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TagSpecification");
    if let Some(var_6) = &input.tag_specifications {
        let mut list_8 = scope_5.start_list(true, Some("item"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("AmazonSideAsn");
    if let Some(var_11) = &input.amazon_side_asn {
        scope_10.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("DryRun");
    if let Some(var_13) = &input.dry_run {
        scope_12.boolean(*var_13);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
