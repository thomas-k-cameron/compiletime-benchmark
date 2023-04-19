// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_transit_gateway_connect_peer_input_input(
    input: &crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "CreateTransitGatewayConnectPeer",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TransitGatewayAttachmentId");
    if let Some(var_2) = &input.transit_gateway_attachment_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TransitGatewayAddress");
    if let Some(var_4) = &input.transit_gateway_address {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PeerAddress");
    if let Some(var_6) = &input.peer_address {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("BgpOptions");
    if let Some(var_8) = &input.bgp_options {
        crate::protocol_serde::shape_transit_gateway_connect_request_bgp_options::ser_transit_gateway_connect_request_bgp_options(scope_7, var_8)?;
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("InsideCidrBlocks");
    if let Some(var_10) = &input.inside_cidr_blocks {
        let mut list_12 = scope_9.start_list(true, Some("item"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            entry_13.string(item_11);
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("TagSpecification");
    if let Some(var_15) = &input.tag_specifications {
        let mut list_17 = scope_14.start_list(true, Some("item"));
        for item_16 in var_15 {
            #[allow(unused_mut)]
            let mut entry_18 = list_17.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_18, item_16,
            )?;
        }
        list_17.finish();
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("DryRun");
    if let Some(var_20) = &input.dry_run {
        scope_19.boolean(*var_20);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
