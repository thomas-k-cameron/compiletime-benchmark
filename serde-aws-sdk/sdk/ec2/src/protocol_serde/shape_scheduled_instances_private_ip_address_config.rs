// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_scheduled_instances_private_ip_address_config(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ScheduledInstancesPrivateIpAddressConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Primary");
    if let Some(var_2) = &input.primary {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PrivateIpAddress");
    if let Some(var_4) = &input.private_ip_address {
        scope_3.string(var_4);
    }
    Ok(())
}
