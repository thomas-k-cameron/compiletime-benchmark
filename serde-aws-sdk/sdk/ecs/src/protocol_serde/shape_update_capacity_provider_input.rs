// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_capacity_provider_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_capacity_provider::UpdateCapacityProviderInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.auto_scaling_group_provider {
        #[allow(unused_mut)]
        let mut object_3 = object.key("autoScalingGroupProvider").start_object();
        crate::protocol_serde::shape_auto_scaling_group_provider_update::ser_auto_scaling_group_provider_update(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
