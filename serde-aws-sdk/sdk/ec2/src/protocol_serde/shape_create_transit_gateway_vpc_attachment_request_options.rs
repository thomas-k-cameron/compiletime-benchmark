// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_create_transit_gateway_vpc_attachment_request_options(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::CreateTransitGatewayVpcAttachmentRequestOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DnsSupport");
    if let Some(var_2) = &input.dns_support {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Ipv6Support");
    if let Some(var_4) = &input.ipv6_support {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ApplianceModeSupport");
    if let Some(var_6) = &input.appliance_mode_support {
        scope_5.string(var_6.as_str());
    }
    Ok(())
}
