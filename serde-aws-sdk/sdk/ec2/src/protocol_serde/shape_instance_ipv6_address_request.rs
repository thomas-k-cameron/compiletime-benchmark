// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_ipv6_address_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::InstanceIpv6AddressRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Ipv6Address");
    if let Some(var_2) = &input.ipv6_address {
        scope_1.string(var_2);
    }
    Ok(())
}
