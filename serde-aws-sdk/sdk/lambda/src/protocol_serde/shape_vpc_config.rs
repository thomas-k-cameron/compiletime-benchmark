// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vpc_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VpcConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.subnet_ids {
        let mut array_2 = object.key("SubnetIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.security_group_ids {
        let mut array_5 = object.key("SecurityGroupIds").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}
