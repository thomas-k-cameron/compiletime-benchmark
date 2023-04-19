// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_customer_gateway_input_input(
    input: &crate::operation::create_customer_gateway::CreateCustomerGatewayInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "CreateCustomerGateway", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("BgpAsn");
    if let Some(var_2) = &input.bgp_asn {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PublicIp");
    if let Some(var_4) = &input.public_ip {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("CertificateArn");
    if let Some(var_6) = &input.certificate_arn {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Type");
    if let Some(var_8) = &input.r#type {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("TagSpecification");
    if let Some(var_10) = &input.tag_specifications {
        let mut list_12 = scope_9.start_list(true, Some("item"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_13, item_11,
            )?;
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("DeviceName");
    if let Some(var_15) = &input.device_name {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("IpAddress");
    if let Some(var_17) = &input.ip_address {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("DryRun");
    if let Some(var_19) = &input.dry_run {
        scope_18.boolean(*var_19);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
