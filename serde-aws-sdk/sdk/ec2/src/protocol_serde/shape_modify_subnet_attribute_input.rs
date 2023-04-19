// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_subnet_attribute_input_input(
    input: &crate::operation::modify_subnet_attribute::ModifySubnetAttributeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ModifySubnetAttribute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AssignIpv6AddressOnCreation");
    if let Some(var_2) = &input.assign_ipv6_address_on_creation {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_1, var_2,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MapPublicIpOnLaunch");
    if let Some(var_4) = &input.map_public_ip_on_launch {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_3, var_4,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SubnetId");
    if let Some(var_6) = &input.subnet_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MapCustomerOwnedIpOnLaunch");
    if let Some(var_8) = &input.map_customer_owned_ip_on_launch {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_7, var_8,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("CustomerOwnedIpv4Pool");
    if let Some(var_10) = &input.customer_owned_ipv4_pool {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("EnableDns64");
    if let Some(var_12) = &input.enable_dns64 {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_11, var_12,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("PrivateDnsHostnameTypeOnLaunch");
    if let Some(var_14) = &input.private_dns_hostname_type_on_launch {
        scope_13.string(var_14.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("EnableResourceNameDnsARecordOnLaunch");
    if let Some(var_16) = &input.enable_resource_name_dns_a_record_on_launch {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_15, var_16,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("EnableResourceNameDnsAAAARecordOnLaunch");
    if let Some(var_18) = &input.enable_resource_name_dns_aaaa_record_on_launch {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_17, var_18,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("EnableLniAtDeviceIndex");
    if let Some(var_20) = &input.enable_lni_at_device_index {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("DisableLniAtDeviceIndex");
    if let Some(var_22) = &input.disable_lni_at_device_index {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(
            scope_21, var_22,
        )?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
