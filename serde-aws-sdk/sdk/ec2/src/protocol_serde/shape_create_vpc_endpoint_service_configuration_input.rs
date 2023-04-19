// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_vpc_endpoint_service_configuration_input_input(
    input: &crate::operation::create_vpc_endpoint_service_configuration::CreateVpcEndpointServiceConfigurationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "CreateVpcEndpointServiceConfiguration",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AcceptanceRequired");
    if let Some(var_4) = &input.acceptance_required {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PrivateDnsName");
    if let Some(var_6) = &input.private_dns_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("NetworkLoadBalancerArn");
    if let Some(var_8) = &input.network_load_balancer_arns {
        let mut list_10 = scope_7.start_list(true, Some("item"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("GatewayLoadBalancerArn");
    if let Some(var_13) = &input.gateway_load_balancer_arns {
        let mut list_15 = scope_12.start_list(true, Some("item"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("SupportedIpAddressType");
    if let Some(var_18) = &input.supported_ip_address_types {
        let mut list_20 = scope_17.start_list(true, Some("item"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            entry_21.string(item_19);
        }
        list_20.finish();
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("ClientToken");
    if let Some(var_23) = &input.client_token {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("TagSpecification");
    if let Some(var_25) = &input.tag_specifications {
        let mut list_27 = scope_24.start_list(true, Some("item"));
        for item_26 in var_25 {
            #[allow(unused_mut)]
            let mut entry_28 = list_27.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_28, item_26,
            )?;
        }
        list_27.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
