// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_vpn_connection_input_input(
    input: &crate::operation::delete_vpn_connection::DeleteVpnConnectionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteVpnConnection", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VpnConnectionId");
    if let Some(var_2) = &input.vpn_connection_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
