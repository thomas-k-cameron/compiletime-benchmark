// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_client_vpn_endpoint_input_input(
    input: &crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "CreateClientVpnEndpoint", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientCidrBlock");
    if let Some(var_2) = &input.client_cidr_block {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ServerCertificateArn");
    if let Some(var_4) = &input.server_certificate_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Authentication");
    if let Some(var_6) = &input.authentication_options {
        let mut list_8 = scope_5.start_list(true, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_client_vpn_authentication_request::ser_client_vpn_authentication_request(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("ConnectionLogOptions");
    if let Some(var_11) = &input.connection_log_options {
        crate::protocol_serde::shape_connection_log_options::ser_connection_log_options(
            scope_10, var_11,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("DnsServers");
    if let Some(var_13) = &input.dns_servers {
        let mut list_15 = scope_12.start_list(true, Some("item"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("TransportProtocol");
    if let Some(var_18) = &input.transport_protocol {
        scope_17.string(var_18.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("VpnPort");
    if let Some(var_20) = &input.vpn_port {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Description");
    if let Some(var_22) = &input.description {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("SplitTunnel");
    if let Some(var_24) = &input.split_tunnel {
        scope_23.boolean(*var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("DryRun");
    if let Some(var_26) = &input.dry_run {
        scope_25.boolean(*var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("ClientToken");
    if let Some(var_28) = &input.client_token {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("TagSpecification");
    if let Some(var_30) = &input.tag_specifications {
        let mut list_32 = scope_29.start_list(true, Some("item"));
        for item_31 in var_30 {
            #[allow(unused_mut)]
            let mut entry_33 = list_32.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_33, item_31,
            )?;
        }
        list_32.finish();
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("SecurityGroupId");
    if let Some(var_35) = &input.security_group_ids {
        let mut list_37 = scope_34.start_list(true, Some("item"));
        for item_36 in var_35 {
            #[allow(unused_mut)]
            let mut entry_38 = list_37.entry();
            entry_38.string(item_36);
        }
        list_37.finish();
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("VpcId");
    if let Some(var_40) = &input.vpc_id {
        scope_39.string(var_40);
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("SelfServicePortal");
    if let Some(var_42) = &input.self_service_portal {
        scope_41.string(var_42.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("ClientConnectOptions");
    if let Some(var_44) = &input.client_connect_options {
        crate::protocol_serde::shape_client_connect_options::ser_client_connect_options(
            scope_43, var_44,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("SessionTimeoutHours");
    if let Some(var_46) = &input.session_timeout_hours {
        scope_45.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_46).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("ClientLoginBannerOptions");
    if let Some(var_48) = &input.client_login_banner_options {
        crate::protocol_serde::shape_client_login_banner_options::ser_client_login_banner_options(
            scope_47, var_48,
        )?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
