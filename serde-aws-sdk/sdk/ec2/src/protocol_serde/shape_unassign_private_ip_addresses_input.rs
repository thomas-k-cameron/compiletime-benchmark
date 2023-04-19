// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_unassign_private_ip_addresses_input_input(
    input: &crate::operation::unassign_private_ip_addresses::UnassignPrivateIpAddressesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "UnassignPrivateIpAddresses", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NetworkInterfaceId");
    if let Some(var_2) = &input.network_interface_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PrivateIpAddress");
    if let Some(var_4) = &input.private_ip_addresses {
        let mut list_6 = scope_3.start_list(true, Some("PrivateIpAddress"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Ipv4Prefix");
    if let Some(var_9) = &input.ipv4_prefixes {
        let mut list_11 = scope_8.start_list(true, Some("item"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            entry_12.string(item_10);
        }
        list_11.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
