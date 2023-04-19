// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_vpn_tunnel_log_options_specification(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::VpnTunnelLogOptionsSpecification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CloudWatchLogOptions");
    if let Some(var_2) = &input.cloud_watch_log_options {
        crate::protocol_serde::shape_cloud_watch_log_options_specification::ser_cloud_watch_log_options_specification(scope_1, var_2)?;
    }
    Ok(())
}