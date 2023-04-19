// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_vpn_connection_route_input_input(
    input: &crate::operation::delete_vpn_connection_route::DeleteVpnConnectionRouteInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteVpnConnectionRoute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DestinationCidrBlock");
    if let Some(var_2) = &input.destination_cidr_block {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VpnConnectionId");
    if let Some(var_4) = &input.vpn_connection_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
